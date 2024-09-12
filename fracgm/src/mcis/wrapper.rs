extern crate libc;

use ndarray::Array2;
use num_cpus;

use crate::mcis::array_helper;

#[repr(C)]
pub struct CIntBuffer {
    pub data: *mut libc::c_int,
    pub len: libc::size_t,
}

fn to_i32_vec(buf: &CIntBuffer) -> Vec<i32> {
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
    fn free_c_int_buffer(buf: CIntBuffer);

    fn inlier_selection(
        src_array: *mut libc::c_double,
        src_array_len: libc::size_t,
        dst_array: *mut libc::c_double,
        dst_array_len: libc::size_t,
        noise_bound: libc::c_double,
        pmc_timeout: libc::c_double,
        pmc_n_threads: libc::c_int,
    ) -> CIntBuffer;
}

pub fn max_clique_inlier_selection(
    pc1: &Array2<f64>,
    pc2: &Array2<f64>,
    noise_bound: f64,
    pmc_timeout: f64,
) -> Vec<usize> {
    let pmc_n_threads = num_cpus::get() as i32;

    let src = array_helper::to_buffer(pc1);
    let dst = array_helper::to_buffer(pc2);

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

        array_helper::free_buf(src);
        array_helper::free_buf(dst);
        free_c_int_buffer(buf);

        result.into_iter().map(|x| x as usize).collect()
    }
}
