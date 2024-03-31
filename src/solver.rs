use ndarray::linalg::kron;
use ndarray::prelude::*;
use ndarray::Array2;

use osqp;

use crate::utils;

const DIM: usize = 10;

pub struct Solver {
    pub max_iteration: i32,
    pub tol: f64,
    pub c: f64,
}

struct F {
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

struct H {
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

struct Fractional {
    pub f: F,
    pub h: H,
}

fn compute_initial_guess(pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
    let pc1 = &utils::get_zero_mean_point_cloud(pc1);
    let pc2 = &utils::get_zero_mean_point_cloud(pc2);

    utils::project(&pc1.t().dot(pc2))
}

fn rot_mat_to_vec(rot: &Array2<f64>) -> Array2<f64> {
    let mut rot_vec = Array2::<f64>::ones((DIM, 1));
    rot_vec
        .slice_mut(s![0..9, 0])
        .assign(&rot.clone().t().into_shape(9).unwrap());

    rot_vec
}

fn rot_vec_to_mat(rot_vec: &Array2<f64>) -> Array2<f64> {
    let mut rot = rot_vec
        .clone()
        .slice_move(s![0..9, 0])
        .into_shape((3, 3))
        .unwrap();
    rot.swap_axes(0, 1);

    rot
}

fn solve_alpha(mat: &Array2<f64>) -> Array2<f64> {
    assert!(mat.dim().0 == mat.dim().1);
    assert!(mat.dim().0 == DIM);

    let mut mat_p = [[0.0; DIM]; DIM];

    for i in 0..mat.dim().0 {
        for j in 0..mat.dim().1 {
            mat_p[i][j] = mat[[i, j]];
        }
    }

    let q = [0.0; DIM];

    let mut mat_a = [[0.0; DIM]; 1];
    mat_a[0][9] = 1.0;
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

    let mut x = Array2::<f64>::zeros((DIM, 1));
    for i in 0..DIM {
        x[[i, 0]] = x_[i];
    }

    x
}

fn solve_beta_mu(
    alpha: &Array2<f64>,
    init_beta: &Vec<f64>,
    init_mu: &Vec<f64>,
    terms: &Vec<Fractional>,
) -> (Vec<f64>, Vec<f64>) {
    assert!(init_beta.len() == init_mu.len());
    assert!(alpha.dim().0 == DIM);

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

impl Solver {
    pub fn new(max_iteration: i32, tol: f64, c: f64) -> Solver {
        Solver {
            max_iteration,
            tol,
            c,
        }
    }

    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional> {
        let mut terms: Vec<Fractional> = Vec::new();
        terms.reserve(pc1.dim().0);

        let id3 = Array2::eye(3);
        for i in 0..pc1.dim().0 {
            let weight = 1.0;
            let mut mat_n = Array2::zeros((3, DIM));

            mat_n
                .slice_mut(s![.., 0..9])
                .assign(&kron(&pc1.row(i).into_shape((1, 3)).unwrap(), &id3));
            mat_n
                .slice_mut(s![.., 9])
                .assign(&pc2.row(i).t().mapv(|x| -1.0 * x));

            let mat_m = mat_n.t().dot(&mat_n) * weight;

            terms.push(Fractional {
                f: F::new(mat_m.mapv(|x| self.c * self.c * x)),
                h: H::new(mat_m.clone(), self.c),
            });
        }

        terms
    }

    pub fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        if pc1.dim().0 != pc2.dim().0 {
            panic!("The number of samples in pc1 and pc2 must be the same.");
        }

        let terms = self.compute_terms(pc1, pc2);

        let init_rot = compute_initial_guess(pc1, pc2);
        let mut rot_vec = rot_mat_to_vec(&init_rot);

        let mut beta: Vec<f64> = terms
            .iter()
            .map(|frac| frac.f.call(&rot_vec) / frac.h.call(&rot_vec))
            .collect();
        let mut mu: Vec<f64> = terms
            .iter()
            .map(|frac| 1.0 / frac.h.call(&rot_vec))
            .collect();

        for _ in 0..self.max_iteration {
            let mut mat_a = Array2::<f64>::zeros((DIM, DIM));
            for i in 0..pc1.dim().0 {
                mat_a = mat_a + mu[i] * &terms[i].f.mat - mu[i] * beta[i] * &terms[i].f.mat;
            }

            rot_vec = solve_alpha(&mat_a);

            let psi_norm = compute_psi_norm(&rot_vec, &beta, &mu, &terms);
            if psi_norm < self.tol {
                break;
            }

            (beta, mu) = solve_beta_mu(&rot_vec, &beta, &mu, &terms);
        }

        utils::project(&rot_vec_to_mat(&rot_vec))
    }
}
