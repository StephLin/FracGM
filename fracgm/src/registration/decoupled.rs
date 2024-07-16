use ndarray::prelude::*;
use ndarray::Array2;

use crate::rotation::LinearSolver as RotationSolver;
use crate::solver::GemanMcclureSolver;
use crate::translation::LinearSolver as TranslationSolver;
use crate::utils;

pub enum TIMPolicy {
    CHAIN,
    COMPLETE,
}

pub struct Solver {
    pub rotation_solver: RotationSolver,
    pub translation_solver: TranslationSolver,

    pub tim_policy: TIMPolicy,
}

impl Solver {
    pub fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        let translation_solver = TranslationSolver::new(max_iteration, tol, noise_bound, c);

        let rotation_solver = RotationSolver::new(
            max_iteration,
            tol,
            Some(translation_solver.noise_bound * 2.0),
            c,
        );

        Self {
            rotation_solver,
            translation_solver,
            tim_policy: TIMPolicy::CHAIN,
        }
    }

    pub fn set_tim_policy(&mut self, tim_policy: TIMPolicy) {
        self.tim_policy = tim_policy;
    }
}

impl Solver {
    pub fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64> {
        let pc1_tims = match self.tim_policy {
            TIMPolicy::CHAIN => utils::compute_chain_translation_invariant_measurements(pc1),
            TIMPolicy::COMPLETE => utils::compute_complete_translation_invariant_measurements(pc1),
        };
        let pc2_tims = match self.tim_policy {
            TIMPolicy::CHAIN => utils::compute_chain_translation_invariant_measurements(pc2),
            TIMPolicy::COMPLETE => utils::compute_complete_translation_invariant_measurements(pc2),
        };

        let rot = self.rotation_solver.solve(&pc1_tims, &pc2_tims);

        let rot_pc1 = rot.dot(&pc1.t()).t().to_owned();
        let trans = self.translation_solver.solve(&rot_pc1, pc2);

        let mut transform = Array2::<f64>::eye(4);
        transform.slice_mut(s![0..3, 0..3]).assign(&rot);
        transform.slice_mut(s![0..3, 3..4]).assign(&trans);

        transform
    }
}
