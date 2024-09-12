use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyclass, pyfunction, pymethods, pymodule, Bound, PyResult, Python};

use fracgm::{
    mcis, registration, rotation,
    solver::{self, GemanMcclureSolver, GemanMcclureSolverDiagnostic},
    translation,
};

#[pyclass]
pub struct IterationComponent(solver::IterationComponent);

impl IterationComponent {
    pub fn from(iteration_component: &solver::IterationComponent) -> Self {
        Self(iteration_component.clone())
    }
}

#[pymethods]
impl IterationComponent {
    #[getter]
    fn alpha_vec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        Ok(self.0.alpha_vec.clone().into_pyarray_bound(py))
    }

    #[getter]
    fn alpha_mat<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        Ok(self.0.alpha_mat.clone().into_pyarray_bound(py))
    }

    #[getter]
    fn alpha_proj<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        Ok(self.0.alpha_proj.clone().into_pyarray_bound(py))
    }

    #[getter]
    fn beta(&self) -> PyResult<Vec<f64>> {
        Ok(self.0.beta.clone())
    }

    #[getter]
    fn mu(&self) -> PyResult<Vec<f64>> {
        Ok(self.0.mu.clone())
    }

    #[getter]
    fn psi_norm(&self) -> PyResult<f64> {
        Ok(self.0.psi_norm)
    }
}

#[pyclass]
pub struct Diagnostic(solver::Diagnostic);

impl Diagnostic {
    pub fn from(diagnostic: &solver::Diagnostic) -> Self {
        Self(diagnostic.clone())
    }
}

#[pymethods]
impl Diagnostic {
    #[getter]
    fn iterations(&self) -> PyResult<Vec<IterationComponent>> {
        Ok(self
            .0
            .iterations
            .iter()
            .map(IterationComponent::from)
            .collect())
    }

    #[getter]
    fn solution<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray2<f64>>> {
        Ok(self.0.solution.clone().into_pyarray_bound(py))
    }

    #[getter]
    fn n_iters(&self) -> PyResult<usize> {
        Ok(self.0.n_iters)
    }
}

#[pyclass]
pub struct LinearRotationSolver(rotation::LinearSolver);

#[pymethods]
impl LinearRotationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        LinearRotationSolver(rotation::LinearSolver::new(
            max_iteration,
            tol,
            noise_bound,
            c,
        ))
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let rot = GemanMcclureSolver::solve(&self.0, &pc1, &pc2);

        rot.into_pyarray_bound(py)
    }

    unsafe fn solve_with_diagnostic<'py>(
        &self,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Diagnostic {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let diagnostic = GemanMcclureSolverDiagnostic::solve(&self.0, &pc1, &pc2);

        Diagnostic::from(&diagnostic)
    }
}

#[pyclass]
pub struct LinearRegistrationSolver(registration::LinearSolver);

#[pymethods]
impl LinearRegistrationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        LinearRegistrationSolver(registration::LinearSolver::new(
            max_iteration,
            tol,
            noise_bound,
            c,
        ))
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let mat = GemanMcclureSolver::solve(&self.0, &pc1, &pc2);

        mat.into_pyarray_bound(py)
    }

    unsafe fn solve_with_diagnostic<'py>(
        &self,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Diagnostic {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let diagnostic = GemanMcclureSolverDiagnostic::solve(&self.0, &pc1, &pc2);

        Diagnostic::from(&diagnostic)
    }
}

#[pyclass]
pub struct LinearTranslationSolver(translation::LinearSolver);

#[pymethods]
impl LinearTranslationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        LinearTranslationSolver(translation::LinearSolver::new(
            max_iteration,
            tol,
            noise_bound,
            c,
        ))
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let mat = GemanMcclureSolver::solve(&self.0, &pc1, &pc2);

        mat.into_pyarray_bound(py)
    }

    unsafe fn solve_with_diagnostic<'py>(
        &self,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Diagnostic {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let diagnostic = GemanMcclureSolverDiagnostic::solve(&self.0, &pc1, &pc2);

        Diagnostic::from(&diagnostic)
    }
}

#[pyclass]
#[derive(Clone)]
pub enum TIMPolicy {
    CHAIN,
    COMPLETE,
}

#[pyclass]
pub struct DecoupledRegistrationSolver(registration::decoupled::Solver);

#[pymethods]
impl DecoupledRegistrationSolver {
    #[new]
    fn new(max_iteration: usize, tol: f64, noise_bound: Option<f64>, c: Option<f64>) -> Self {
        DecoupledRegistrationSolver(registration::decoupled::Solver::new(
            max_iteration,
            tol,
            noise_bound,
            c,
        ))
    }

    unsafe fn set_tim_policy<'py>(&mut self, tim_policy: TIMPolicy) {
        match tim_policy {
            TIMPolicy::CHAIN => self
                .0
                .set_tim_policy(registration::decoupled::TIMPolicy::CHAIN),
            TIMPolicy::COMPLETE => self
                .0
                .set_tim_policy(registration::decoupled::TIMPolicy::COMPLETE),
        }
    }

    unsafe fn solve<'py>(
        &self,
        py: Python<'py>,
        pc1: PyReadonlyArray2<'py, f64>,
        pc2: PyReadonlyArray2<'py, f64>,
    ) -> Bound<'py, PyArray2<f64>> {
        let pc1 = pc1.as_array().to_owned();
        let pc2 = pc2.as_array().to_owned();

        let mat = self.0.solve(&pc1, &pc2);

        mat.into_pyarray_bound(py)
    }
}

#[pyfunction]
pub fn max_clique_inlier_selection<'py>(
    pc1: PyReadonlyArray2<'py, f64>,
    pc2: PyReadonlyArray2<'py, f64>,
    noise_bound: f64,
    pmc_timeout: f64,
) -> Vec<usize> {
    let pc1 = pc1.as_array().to_owned();
    let pc2 = pc2.as_array().to_owned();

    mcis::max_clique_inlier_selection(&pc1, &pc2, noise_bound, pmc_timeout)
}

#[pymodule]
#[pyo3(name = "fracgm")]
mod py_module {
    use super::*;

    #[pymodule_export]
    use IterationComponent;

    #[pymodule_export]
    use Diagnostic;

    #[pymodule_export]
    use LinearRotationSolver;

    #[pymodule_export]
    use LinearRegistrationSolver;

    #[pymodule_export]
    use LinearTranslationSolver;

    #[pymodule_export]
    use TIMPolicy;

    #[pymodule_export]
    use DecoupledRegistrationSolver;

    #[pymodule_export]
    use max_clique_inlier_selection;
}
