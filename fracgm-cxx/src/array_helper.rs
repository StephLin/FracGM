// Copyright 2024 the FracGM authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use libc;
use ndarray::Array2;
use std::slice;

pub fn to_array2(
    data_ptr: *const libc::c_double,
    n_rows: libc::size_t,
    n_cols: libc::size_t,
) -> Array2<f64> {
    unsafe {
        // assert!(data_ptr.is_null());
        assert!(n_rows > 0);
        assert!(n_cols > 0);

        let len = n_rows * n_cols;

        let v = slice::from_raw_parts(data_ptr as *mut libc::c_double, len).to_vec();

        Array2::<f64>::from_shape_vec_unchecked((n_rows, n_cols), v)
    }
}

#[repr(C)]
pub struct CBufferF64 {
    pub data: *mut f64,
    pub len: usize,
}

#[repr(C)]
pub struct CBufferUSize {
    pub data: *mut usize,
    pub len: usize,
}

pub fn to_f64_buf(arr: &Array2<f64>) -> CBufferF64 {
    let mut vec: Vec<f64> = Vec::new();
    for i in 0..arr.nrows() {
        for j in 0..arr.ncols() {
            vec.push(arr[[i, j]]);
        }
    }

    let mut buf = vec.into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);
    CBufferF64 { data, len }
}

pub fn to_usize_buf(arr: &[usize]) -> CBufferUSize {
    let mut buf = arr.to_vec().into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);
    CBufferUSize { data, len }
}

#[no_mangle]
pub extern "C" fn free_f64_buf(buf: CBufferF64) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let s = s.as_mut_ptr();
    let _ = unsafe { Box::from_raw(s) };
}

#[no_mangle]
pub extern "C" fn free_usize_buf(buf: CBufferUSize) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let s = s.as_mut_ptr();
    let _ = unsafe { Box::from_raw(s) };
}
