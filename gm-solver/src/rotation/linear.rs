use ndarray::linalg::kron;
use ndarray::prelude::*;
use ndarray::Array2;

use crate::rotation::utils as rot_utils;
use crate::solver::{
    Fractional, FractionalProgrammingMaterials, GemanMcclureSolver, GemanMcclureSolverDiagnostic,
    R2,
};
use crate::utils;

pub struct LinearSolver {
    pub max_iteration: usize,
    pub tol: f64,
    pub noise_bound: f64,
    pub c: f64,
}

impl LinearSolver {
    pub fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        Self {
            max_iteration,
            tol,
            noise_bound: noise_bound.unwrap_or(0.1),
            c: c.unwrap_or(1.0),
        }
    }
}

impl FractionalProgrammingMaterials for LinearSolver {
    fn dim(&self) -> usize {
        rot_utils::DIM
    }

    fn max_iteration(&self) -> usize {
        self.max_iteration
    }
    fn tol(&self) -> f64 {
        self.tol
    }
    fn c(&self) -> f64 {
        self.c
    }

    fn mat_to_vec(&self, rot: &Array2<f64>) -> Array2<f64> {
        rot_utils::rot_mat_to_vec(rot)
    }
    fn vec_to_mat(&self, rot_vec: &Array2<f64>) -> Array2<f64> {
        rot_utils::rot_vec_to_mat(rot_vec)
    }
    fn project(&self, mat: &Array2<f64>) -> Array2<f64> {
        utils::project(mat)
    }

    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional> {
        let mut terms: Vec<Fractional> = Vec::new();
        terms.reserve(pc1.dim().0);

        let id3 = Array2::eye(3);
        for i in 0..pc1.dim().0 {
            let mut mat_n = Array2::zeros((3, rot_utils::DIM));

            mat_n
                .slice_mut(s![.., 0..9])
                .assign(&kron(&pc1.row(i).into_shape((1, 3)).unwrap(), &id3));
            mat_n
                .slice_mut(s![.., 9])
                .assign(&pc2.row(i).t().mapv(|x| -1.0 * x));

            let mat_m = mat_n.t().dot(&mat_n) / (self.noise_bound * self.noise_bound);

            terms.push(Fractional::new(R2::new(mat_m), self.c()));
        }

        terms
    }

    fn compute_initial_guess(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        let (pc1, _) = &utils::get_zero_mean_point_cloud(pc1);
        let (pc2, _) = &utils::get_zero_mean_point_cloud(pc2);

        utils::project(&pc1.t().dot(pc2))
    }
}

impl GemanMcclureSolver for LinearSolver {}
impl GemanMcclureSolverDiagnostic for LinearSolver {}
