use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyclass, pymethods, pymodule, Bound, Python};

use gm_solver::{registration, rotation, solver::GemanMcclureLinearSolver};

#[pyclass]
pub struct LinearRotationSolver(rotation::LinearSolver);

#[pymethods]
impl LinearRotationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, c: f64) -> Self {
        LinearRotationSolver(rotation::LinearSolver::new(max_iteration, tol, c))
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let rot = self.0.solve(&pc1, &pc2);

        rot.into_pyarray_bound(py)
    }
}

#[pyclass]
pub struct LinearRegistrationSolver(registration::LinearSolver);

#[pymethods]
impl LinearRegistrationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, c: f64) -> Self {
        LinearRegistrationSolver(registration::LinearSolver::new(max_iteration, tol, c))
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        assert!(
            pc1.shape() == pc2.shape(),
            "Input point clouds must have the same shape"
        );
        assert!(
            pc1.shape()[1] == 3,
            "Input point clouds must have 3 columns"
        );

        let rot = self.0.solve(&pc1, &pc2);

        rot.into_pyarray_bound(py)
    }
}

#[pymodule]
#[pyo3(name = "gm_solver")]
mod py_module {
    use super::*;

    #[pymodule_export]
    use LinearRotationSolver;

    #[pymodule_export]
    use LinearRegistrationSolver;
}
