// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use ndarray::prelude::*;
use ndarray::Array2;

use fracgm::mcis::max_clique_inlier_selection;
use fracgm::registration::LinearSolver as LinearRegistrationSolver;
use fracgm::rotation::LinearSolver as LinearRotationSolver;
use fracgm::solver::GemanMcclureSolver;

const ENABLE_MAX_CLIQUE_INLIER_SELECTION: bool = true;

fn read_matrix(path: impl AsRef<Path>) -> Array2<f64> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<f64>> = Vec::new();
    let mut n_cols = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_data: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()
            .unwrap();

        if n_cols == 0 {
            n_cols = line_data.len();
        } else if n_cols != line_data.len() {
            panic!("Expected {} columns, got {}", n_cols, line_data.len());
        }

        data.push(line_data);
    }

    Array2::from_shape_vec((data.len(), n_cols), data.concat()).unwrap()
}

fn get_rotation_test_data() -> (Array2<f64>, Array2<f64>, Array2<f64>) {
    let src = read_matrix("../../../data/cloud_src.txt");
    let dst = read_matrix("../../../data/cloud_dst.txt");
    let gt = read_matrix("../../../data/gt.txt");

    (src, dst, gt)
}

fn get_registration_test_data() -> (Array2<f64>, Array2<f64>, Array2<f64>) {
    let src = read_matrix("../../../data/cloud_src.txt");
    let mut dst = read_matrix("../../../data/cloud_dst.txt");

    let mut col = dst.column_mut(0);
    col += 0.3;
    let mut col = dst.column_mut(1);
    col += 0.2;
    let mut col = dst.column_mut(2);
    col += 0.6;

    let mut gt = Array2::<f64>::eye(4);
    let rot_gt = read_matrix("../../../data/gt.txt");
    gt.slice_mut(s![0..3, 0..3]).assign(&rot_gt);
    gt[[0, 3]] = 0.3;
    gt[[1, 3]] = 0.2;
    gt[[2, 3]] = 0.6;

    (src, dst, gt)
}

fn perform_max_clique_inlier_selection(
    src: &Array2<f64>,
    dst: &Array2<f64>,
    noise_bound: f64,
    pmc_timeout: f64,
) -> (Array2<f64>, Array2<f64>) {
    let indices = max_clique_inlier_selection(src, dst, noise_bound, pmc_timeout);

    let src = src.select(Axis(0), &indices);
    let dst = dst.select(Axis(0), &indices);

    (src, dst)
}

fn main() {
    let max_iteration = 100;
    let tol = 1e-6;
    let noise_bound: Option<f64> = Some(0.1);
    let c = Some(1.0);
    let pmc_timeout = 3600.0;

    println!("[[ Example for FracGM-based rotation solver ]]\n");
    let (src_rot, dst_rot, gt_rot) = get_rotation_test_data();

    let est_rot =
        LinearRotationSolver::new(max_iteration, tol, noise_bound, c).solve(&src_rot, &dst_rot);

    println!("Ground Truth:\n{:?}\n", gt_rot);
    println!("FracGM:\n{:?}\n", est_rot);

    println!("[[ Example for FracGM-based registration solver ]]\n");
    let (src_reg, dst_reg, gt_reg) = get_registration_test_data();

    if ENABLE_MAX_CLIQUE_INLIER_SELECTION {
        let (src_reg, dst_reg) = perform_max_clique_inlier_selection(
            &src_reg,
            &dst_reg,
            noise_bound.unwrap(),
            pmc_timeout,
        );

        let est_reg = LinearRegistrationSolver::new(max_iteration, tol, noise_bound, c)
            .solve(&src_reg, &dst_reg);

        println!("Ground Truth:\n{:?}\n", gt_reg);
        println!("FracGM:\n{:?}\n", est_reg);
    } else {
        let est_reg = LinearRegistrationSolver::new(max_iteration, tol, noise_bound, c)
            .solve(&src_reg, &dst_reg);

        println!("Ground Truth:\n{:?}\n", gt_reg);
        println!("FracGM:\n{:?}\n", est_reg);
    }

    println!("[[ Done ]]");
}
