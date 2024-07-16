use ndarray::arr1;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::Axis;
use ndarray_linalg::Determinant;
use ndarray_linalg::SVD;

pub fn project(mat: &Array2<f64>) -> Array2<f64> {
    assert!(mat.dim().0 == mat.dim().1, "Matrix must be square");
    assert!(mat.dim().0 == 3, "Matrix must be 3x3");

    let (u, _, vt) = mat.svd(true, true).unwrap();

    let u = u.unwrap();
    let vt = vt.unwrap();

    let rot = u.dot(&vt);

    if rot.det().unwrap() > 0.0 {
        rot
    } else {
        let s = Array2::from_diag(&arr1(&[1.0, 1.0, -1.0]));
        u.dot(&s).dot(&vt)
    }
}

pub fn get_zero_mean_point_cloud(pc: &Array2<f64>) -> (Array2<f64>, Array1<f64>) {
    let mean = pc.mean_axis(Axis(0)).unwrap();
    let mut c_pc = Array2::zeros(pc.raw_dim());

    for i in 0..pc.dim().0 {
        let c_row = &pc.row(i) - &mean;
        c_pc.row_mut(i).assign(&c_row);
    }

    (c_pc, mean)
}

pub fn compute_complete_translation_invariant_measurements(pc: &Array2<f64>) -> Array2<f64> {
    let n_tims = pc.dim().0 * (pc.dim().0 - 1) / 2;
    let mut tims = Array2::<f64>::zeros((n_tims, 3));

    let mut idx = 0;
    for i in 0..pc.dim().0 {
        for j in (i + 1)..pc.dim().0 {
            let diff = &pc.row(i) - &pc.row(j);
            tims.row_mut(idx).assign(&diff);
            idx += 1;
        }
    }

    tims
}

pub fn compute_chain_translation_invariant_measurements(pc: &Array2<f64>) -> Array2<f64> {
    let n_tims = pc.dim().0;
    let mut tims = Array2::<f64>::zeros((n_tims, 3));

    for i in 0..pc.dim().0 {
        let j = if i == pc.dim().0 - 1 { 0 } else { i + 1 };
        let diff = &pc.row(i) - &pc.row(j);
        tims.row_mut(i).assign(&diff);
    }

    tims
}
