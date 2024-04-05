use ndarray::Array2;

pub trait Solver {
    fn solve(&self, pc1: &Array2<f64>, pc2: &Array2<f64>) -> Array2<f64>;
}
