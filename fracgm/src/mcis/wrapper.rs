// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate libc;

use ndarray::Array2;
use num_cpus;

use crate::mcis::array_helper;

#[repr(C)]
pub struct CBufferI32 {
    pub data: *mut libc::c_int,
    pub len: libc::size_t,
}

fn to_i32_vec(buf: &CBufferI32) -> Vec<i32> {
    if buf.len == 0 {
        vec![]
    } else {
        unsafe {
            let s = std::slice::from_raw_parts_mut(buf.data, buf.len);
            s.to_vec()
        }
    }
}

#[link(name = "mcis", kind = "static")]
extern "C" {
    fn free_c_int_buffer(buf: CBufferI32);

    fn inlier_selection(
        src_array: *mut libc::c_double,
        src_array_len: libc::size_t,
        dst_array: *mut libc::c_double,
        dst_array_len: libc::size_t,
        noise_bound: libc::c_double,
        pmc_timeout: libc::c_double,
        pmc_n_threads: libc::c_int,
    ) -> CBufferI32;
}

/// This function takes two point clouds and their noise bound, and timeout in
/// seconds for the max clique solver, and returns the indices of inlier
/// correspondences.
///
/// This function invokes the max clique inlier selection (MCIS) algorithm
/// coming from the TEASER++ solver.
///
/// The max clique solver is run with the number of threads equal to the number
/// of logical CPU cores available.
///
/// # Returns
///
/// Indices of inlier correspondences.
pub fn max_clique_inlier_selection(
    pc1: &Array2<f64>,
    pc2: &Array2<f64>,
    noise_bound: f64,
    pmc_timeout: f64,
) -> Vec<usize> {
    let pmc_n_threads = num_cpus::get() as i32;

    let src = array_helper::to_f64_buf(pc1);
    let dst = array_helper::to_f64_buf(pc2);

    unsafe {
        let buf = inlier_selection(
            src.data,
            src.len,
            dst.data,
            dst.len,
            noise_bound,
            pmc_timeout,
            pmc_n_threads,
        );

        let result = to_i32_vec(&buf);

        array_helper::free_f64_buf(src);
        array_helper::free_f64_buf(dst);
        free_c_int_buffer(buf);

        result.into_iter().map(|x| x as usize).collect()
    }
}
