// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ndarray::prelude::*;
use ndarray::Array2;

pub const DIM: usize = 10;

pub fn rot_mat_to_vec(rot: &Array2<f64>) -> Array2<f64> {
    let mut rot_vec = Array2::<f64>::ones((DIM, 1));
    rot_vec
        .slice_mut(s![0..9, 0])
        .assign(&rot.clone().t().into_shape(9).unwrap());

    rot_vec
}

pub fn rot_vec_to_mat(rot_vec: &Array2<f64>) -> Array2<f64> {
    let mut rot = rot_vec
        .clone()
        .slice_move(s![0..9, 0])
        .into_shape((3, 3))
        .unwrap();
    rot.swap_axes(0, 1);

    rot
}
