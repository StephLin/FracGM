use ndarray::prelude::*;
use ndarray::Array2;

use crate::rotation::utils as rot_utils;

pub const DIM: usize = 13;

pub fn se3_mat_to_vec(mat: &Array2<f64>) -> Array2<f64> {
    let mut vec = Array2::<f64>::ones((DIM, 1));
    vec.slice_mut(s![0..9, 0]).assign(
        &rot_utils::rot_mat_to_vec(&mat.slice(s![0..3, 0..3]).to_owned()).slice_move(s![0..9, 0]),
    );
    vec.slice_mut(s![9..12, 0]).assign(&mat.slice(s![0..3, 3]));

    vec
}

pub fn se3_vec_to_mat(vec: &Array2<f64>) -> Array2<f64> {
    let mut mat = Array2::<f64>::eye(4);
    mat.slice_mut(s![0..3, 0..3])
        .assign(&rot_utils::rot_vec_to_mat(
            &vec.slice(s![0..9, 0..1]).to_owned(),
        ));
    mat.slice_mut(s![0..3, 3]).assign(&vec.slice(s![9..12, 0]));

    mat
}
