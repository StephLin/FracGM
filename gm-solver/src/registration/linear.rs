use ndarray::linalg::kron;
use ndarray::prelude::*;
use ndarray::Array2;

use crate::registration::utils as reg_utils;
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
        reg_utils::DIM
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

    fn mat_to_vec(&self, mat: &Array2<f64>) -> Array2<f64> {
        reg_utils::se3_mat_to_vec(mat)
    }
    fn vec_to_mat(&self, vec: &Array2<f64>) -> Array2<f64> {
        reg_utils::se3_vec_to_mat(vec)
    }
    fn project(&self, mat: &Array2<f64>) -> Array2<f64> {
        let mut proj_mat = mat.clone();
        proj_mat
            .slice_mut(s![0..3, 0..3])
            .assign(&utils::project(&mat.clone().slice_move(s![0..3, 0..3])));

        proj_mat
    }

    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional> {
        let mut terms: Vec<Fractional> = Vec::new();
        terms.reserve(pc1.dim().0);

        let id3 = Array2::eye(3);
        for i in 0..pc1.dim().0 {
            let mut mat_n = Array2::zeros((3, reg_utils::DIM));

            mat_n
                .slice_mut(s![.., 0..9])
                .assign(&kron(&pc1.row(i).into_shape((1, 3)).unwrap(), &id3));
            mat_n.slice_mut(s![.., 9..12]).assign(&id3);
            mat_n
                .slice_mut(s![.., 12])
                .assign(&pc2.row(i).mapv(|x| -1.0 * x));

            let mat_m = mat_n.t().dot(&mat_n) / (self.noise_bound * self.noise_bound);

            terms.push(Fractional::new(R2::new(mat_m), self.c()));
        }

        terms
    }

    fn compute_initial_guess(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        let (pc1, mean1) = &utils::get_zero_mean_point_cloud(pc1);
        let (pc2, mean2) = &utils::get_zero_mean_point_cloud(pc2);

        let mut mat = Array2::eye(4);

        mat.slice_mut(s![0..3, 0..3])
            .assign(&utils::project(&pc1.t().dot(pc2)));
        mat.slice_mut(s![0..3, 3]).assign(&(mean2 - mean1));

        mat
    }
}

impl GemanMcclureSolver for LinearSolver {}
impl GemanMcclureSolverDiagnostic for LinearSolver {}
