use ndarray::Array2;
use osqp;

pub struct R2 {
    pub mat: Array2<f64>,
    cache: f64,
}

impl R2 {
    pub fn new(mat: Array2<f64>) -> R2 {
        R2 { mat, cache: 0.0 }
    }

    pub fn call(&self, x: &Array2<f64>) -> f64 {
        x.t().dot(&self.mat).dot(x)[[0, 0]]
    }

    pub fn update_cache(&mut self, x: &Array2<f64>) {
        self.cache = self.call(x);
    }

    pub fn mat(&self) -> &Array2<f64> {
        &self.mat
    }

    pub fn cache(&self) -> f64 {
        self.cache
    }
}

pub struct Fractional {
    r2: R2,
    c: f64,
    f_mat: Array2<f64>,
}

impl Fractional {
    pub fn new(r2: R2, c: f64) -> Fractional {
        let f_mat = c * c * r2.mat();

        Fractional { r2, c, f_mat }
    }

    pub fn update_cache(&mut self, x: &Array2<f64>) {
        self.r2.update_cache(x);
    }

    pub fn f(&self) -> f64 {
        self.c * self.c * self.r2.cache()
    }

    pub fn h(&self) -> f64 {
        self.r2.cache() + self.c * self.c
    }

    pub fn f_mat(&self) -> &Array2<f64> {
        &self.f_mat
    }

    pub fn h_mat(&self) -> &Array2<f64> {
        self.r2.mat()
    }
}

pub trait FractionalProgrammingMaterials {
    fn dim(&self) -> usize;

    fn max_iteration(&self) -> usize;
    fn tol(&self) -> f64;
    fn c(&self) -> f64;

    fn mat_to_vec(&self, mat: &Array2<f64>) -> Array2<f64>;
    fn vec_to_mat(&self, vec: &Array2<f64>) -> Array2<f64>;
    fn project(&self, mat: &Array2<f64>) -> Array2<f64>;

    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional>;
    fn compute_initial_guess(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64>;

    fn solve_alpha(&self, mat: &Array2<f64>) -> Array2<f64> {
        assert!(mat.dim().0 == mat.dim().1);
        assert!(mat.dim().0 == self.dim());

        let mut mat_p = vec![vec![0.0; self.dim()]; self.dim()];

        for i in 0..mat.dim().0 {
            for j in 0..mat.dim().1 {
                mat_p[i][j] = mat[[i, j]];
            }
        }

        let q = vec![0.0; self.dim()];

        let mut mat_a = vec![vec![0.0; self.dim()]; 1];
        mat_a[0][self.dim() - 1] = 1.0;
        let l = [1.0; 1];
        let u = [1.0; 1];

        let mut problem = osqp::Problem::new(
            osqp::CscMatrix::from(&mat_p).into_upper_tri(),
            &q,
            &mat_a,
            &l,
            &u,
            &osqp::Settings::default().verbose(false),
        )
        .unwrap();

        let result = problem.solve();

        let x_ = result.x().unwrap();

        let mut x = Array2::<f64>::zeros((self.dim(), 1));
        for i in 0..self.dim() {
            x[[i, 0]] = x_[i];
        }

        x
    }

    fn solve_beta_mu(
        &self,
        init_beta: &Vec<f64>,
        init_mu: &Vec<f64>,
        terms: &Vec<Fractional>,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(init_beta.len() == init_mu.len());

        let mut beta = init_beta.clone();
        let mut mu = init_mu.clone();

        init_beta
            .iter()
            .zip(terms.iter().zip(init_mu.iter()))
            .zip(beta.iter_mut().zip(mu.iter_mut()))
            .for_each(|((init_beta_, (term, init_mu_)), (beta_, mu_))| {
                let f = term.f();
                let h = term.h();
                *beta_ = init_beta_ - 1.0 * (init_beta_ - f / h);
                *mu_ = init_mu_ - 1.0 * (init_mu_ - 1.0 / h);
            });

        (beta, mu)
    }

    fn compute_psi_norm(&self, beta: &Vec<f64>, mu: &Vec<f64>, terms: &Vec<Fractional>) -> f64 {
        assert!(beta.len() == mu.len());
        assert!(beta.len() == terms.len());

        beta.iter()
            .zip(terms.iter().zip(mu.iter()))
            .map(|(beta_, (term, mu_))| {
                let f = term.f();
                let h = term.h();

                let a = -1.0 * f + beta_ * h;
                let b = -1.0 + mu_ * h;
                a * a + b * b
            })
            .sum::<f64>()
            .sqrt()
    }

    fn update_terms_cache(&self, terms: &mut Vec<Fractional>, alpha: &Array2<f64>) {
        terms.iter_mut().for_each(|term| term.update_cache(alpha));
    }
}

pub trait GemanMcclureSolver: FractionalProgrammingMaterials {
    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let mut terms = self.compute_terms(pc1, pc2);

        let init_mat = self.compute_initial_guess(pc1, pc2);
        let mut vec = self.mat_to_vec(&init_mat);
        self.update_terms_cache(&mut terms, &vec);

        let mut beta: Vec<f64> = terms.iter().map(|frac| frac.f() / frac.h()).collect();
        let mut mu: Vec<f64> = terms.iter().map(|frac| 1.0 / frac.h()).collect();

        for _ in 0..self.max_iteration() {
            let mut mat_a = Array2::<f64>::zeros((self.dim(), self.dim()));
            for i in 0..pc1.dim().0 {
                mat_a = mat_a + mu[i] * terms[i].f_mat() - mu[i] * beta[i] * terms[i].h_mat();
            }

            vec = self.solve_alpha(&mat_a);
            self.update_terms_cache(&mut terms, &vec);

            let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);
            if psi_norm < self.tol() {
                break;
            }

            (beta, mu) = self.solve_beta_mu(&beta, &mu, &terms);
        }

        self.project(&self.vec_to_mat(&vec))
    }
}

#[derive(Clone)]
pub struct IterationComponent {
    pub alpha_vec: Array2<f64>,
    pub alpha_mat: Array2<f64>,
    pub alpha_proj: Array2<f64>,
    pub beta: Vec<f64>,
    pub mu: Vec<f64>,
    pub psi_norm: f64,
}

#[derive(Clone)]
pub struct Diagnostic {
    pub iterations: Vec<IterationComponent>,
    pub solution: Array2<f64>,
    pub n_iters: usize,
}

pub trait GemanMcclureSolverDiagnostic: FractionalProgrammingMaterials {
    fn update_diagnostics(
        &self,
        alpha: &Array2<f64>,
        beta: &Vec<f64>,
        mu: &Vec<f64>,
        terms: &Vec<Fractional>,
        diagnostics: &mut Vec<IterationComponent>,
    ) {
        let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);

        let component = IterationComponent {
            alpha_vec: alpha.clone(),
            alpha_mat: self.vec_to_mat(&alpha),
            alpha_proj: self.project(&self.vec_to_mat(&alpha)),
            beta: beta.clone(),
            mu: mu.clone(),
            psi_norm,
        };

        diagnostics.push(component);
    }

    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Diagnostic {
        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let mut iterations: Vec<IterationComponent> = Vec::new();

        let mut terms = self.compute_terms(pc1, pc2);

        let init_mat = self.compute_initial_guess(pc1, pc2);
        let mut vec = self.mat_to_vec(&init_mat);
        self.update_terms_cache(&mut terms, &vec);

        let mut beta: Vec<f64> = terms.iter().map(|frac| frac.f() / frac.h()).collect();
        let mut mu: Vec<f64> = terms.iter().map(|frac| 1.0 / frac.h()).collect();

        self.update_diagnostics(&vec, &beta, &mu, &terms, &mut iterations);

        let mut n_iters: usize = 0;
        for _ in 0..self.max_iteration() {
            n_iters += 1;

            let mut mat_a = Array2::<f64>::zeros((self.dim(), self.dim()));
            for i in 0..pc1.dim().0 {
                mat_a = mat_a + mu[i] * terms[i].f_mat() - mu[i] * beta[i] * terms[i].h_mat();
            }

            vec = self.solve_alpha(&mat_a);
            self.update_terms_cache(&mut terms, &vec);

            let psi_norm = self.compute_psi_norm(&beta, &mu, &terms);
            self.update_diagnostics(&vec, &beta, &mu, &terms, &mut iterations);
            if psi_norm < self.tol() {
                break;
            }

            (beta, mu) = self.solve_beta_mu(&beta, &mu, &terms);
        }

        Diagnostic {
            iterations,
            solution: self.project(&self.vec_to_mat(&vec)),
            n_iters,
        }
    }
}
