use ndarray::Array2;
use osqp;

pub struct F {
    pub mat: Array2<f64>,
}

impl F {
    pub fn new(mat: Array2<f64>) -> F {
        F { mat }
    }

    pub fn call(&self, x: &Array2<f64>) -> f64 {
        x.t().dot(&self.mat).dot(x)[[0, 0]]
    }
}

pub struct H {
    pub mat: Array2<f64>,
    c: f64,
}

impl H {
    pub fn new(mat: Array2<f64>, c: f64) -> H {
        H { mat, c }
    }

    pub fn call(&self, x: &Array2<f64>) -> f64 {
        x.t().dot(&self.mat).dot(x)[[0, 0]] + self.c * self.c
    }
}

pub struct Fractional {
    pub f: F,
    pub h: H,
}

pub trait GemanMcclureLinearSolver {
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
        alpha: &Array2<f64>,
        init_beta: &Vec<f64>,
        init_mu: &Vec<f64>,
        terms: &Vec<Fractional>,
    ) -> (Vec<f64>, Vec<f64>) {
        assert!(init_beta.len() == init_mu.len());
        assert!(alpha.dim().0 == self.dim());

        let mut beta = init_beta.clone();
        let mut mu = init_mu.clone();
        let mut beta_delta = vec![0.0; init_beta.len()];
        let mut mu_delta = vec![0.0; init_mu.len()];

        for i in 0..beta.len() {
            let f = terms[i].f.call(&alpha);
            let h = terms[i].h.call(&alpha);
            beta_delta[i] = -1.0 * (init_beta[i] - f / h);
            mu_delta[i] = -1.0 * (init_mu[i] - 1.0 / h);
        }

        for i in 0..beta.len() {
            beta[i] = beta[i] + beta_delta[i];
            mu[i] = mu[i] + mu_delta[i];
        }

        (beta, mu)
    }

    fn compute_psi_norm(
        &self,
        alpha: &Array2<f64>,
        beta: &Vec<f64>,
        mu: &Vec<f64>,
        terms: &Vec<Fractional>,
    ) -> f64 {
        assert!(beta.len() == mu.len());
        assert!(beta.len() == terms.len());

        let mut psi2: f64 = 0.0;

        for i in 0..beta.len() {
            let f = terms[i].f.call(&alpha);
            let h = terms[i].h.call(&alpha);

            let a = -1.0 * f + beta[i] * h;
            let b = -1.0 + mu[i] * h;
            psi2 = psi2 + a * a + b * b;
        }

        psi2.sqrt()
    }

    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let terms = self.compute_terms(pc1, pc2);

        let init_mat = self.compute_initial_guess(pc1, pc2);
        let mut vec = self.mat_to_vec(&init_mat);
        println!("Init vec: {:}", vec);

        let mut beta: Vec<f64> = terms
            .iter()
            .map(|frac| frac.f.call(&vec) / frac.h.call(&vec))
            .collect();
        let mut mu: Vec<f64> = terms.iter().map(|frac| 1.0 / frac.h.call(&vec)).collect();

        println!("Init BETA: {:?}", beta);
        println!("Init MU: {:?}", mu);

        for c in 0..self.max_iteration() {
            let mut mat_a = Array2::<f64>::zeros((self.dim(), self.dim()));
            for i in 0..pc1.dim().0 {
                mat_a = mat_a + mu[i] * &terms[i].f.mat - mu[i] * beta[i] * &terms[i].h.mat;
            }

            vec = self.solve_alpha(&mat_a);

            let psi_norm = self.compute_psi_norm(&vec, &beta, &mu, &terms);
            if psi_norm < self.tol() {
                println!("Converged at iteration {:}", c);
                break;
            }

            (beta, mu) = self.solve_beta_mu(&vec, &beta, &mu, &terms);
        }

        println!("Final vec: {:}", vec);

        self.project(&self.vec_to_mat(&vec))
    }
}
