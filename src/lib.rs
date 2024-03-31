pub mod pair;
pub mod solver;
pub mod utils;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyclass, pymethods, pymodule, types::PyModule, Bound, PyResult, Python};

#[pyclass]
#[pyo3(name = "Solver")]
pub struct PySolver(solver::Solver);

#[pymethods]
impl PySolver {
    #[new]
    fn new(max_iteration: i32, tol: f64, c: f64) -> Self {
        PySolver(solver::Solver::new(max_iteration, tol, c))
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
fn geman_mcclure_rotation_solver(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySolver>()?;
    Ok(())
}
