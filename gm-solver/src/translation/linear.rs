use ndarray::prelude::*;
use ndarray::Array2;

use crate::solver::{
    Fractional, FractionalProgrammingMaterials, GemanMcclureSolver, GemanMcclureSolverDiagnostic,
    R2Sym,
};
use crate::translation::utils as trans_utils;
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

impl FractionalProgrammingMaterials<R2Sym> for LinearSolver {
    fn dim(&self) -> usize {
        trans_utils::DIM
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
        let mut vec = Array2::<f64>::zeros((trans_utils::DIM, 1));
        vec.slice_mut(s![0..3, ..]).assign(mat);

        vec
    }

    fn vec_to_mat(&self, vec: &Array2<f64>) -> Array2<f64> {
        vec.slice(s![0..3, ..]).to_owned()
    }

    fn project(&self, mat: &Array2<f64>) -> Array2<f64> {
        mat.clone()
    }

    fn compute_terms(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Vec<Fractional<R2Sym>> {
        let mut terms: Vec<Fractional<R2Sym>> = Vec::new();
        terms.reserve(pc1.dim().0);

        for i in 0..pc1.dim().0 {
            let mut mat_m = Array2::<f64>::eye(trans_utils::DIM);

            let diff = &pc1.row(i) - &pc2.row(i);

            mat_m[[0, 3]] = diff[0];
            mat_m[[1, 3]] = diff[1];
            mat_m[[2, 3]] = diff[2];

            mat_m[[3, 0]] = diff[0];
            mat_m[[3, 1]] = diff[1];
            mat_m[[3, 2]] = diff[2];

            mat_m[[3, 3]] = diff[0] * diff[0] + diff[1] * diff[1] + diff[2] * diff[2];

            terms.push(Fractional::new(R2Sym::new(mat_m), self.c()));
        }

        terms
    }

    fn compute_initial_guess(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        let (_, mean1) = &utils::get_zero_mean_point_cloud(pc1);
        let (_, mean2) = &utils::get_zero_mean_point_cloud(pc2);

        let mut trans = Array2::<f64>::zeros((3, 1));
        trans.slice_mut(s![.., 0]).assign(&(mean2 - mean1));

        trans
    }
}

impl GemanMcclureSolver<R2Sym> for LinearSolver {}
impl GemanMcclureSolverDiagnostic<R2Sym> for LinearSolver {}
