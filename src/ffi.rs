#![allow(non_camel_case_types)]

use std::{
	ffi::c_void,
	os::raw::{c_float, c_int},
};

pub const FFTS_FORWARD: c_int = -1;
pub const FFTS_BACKWARD: c_int = 1;

pub type FftsPlan = c_void;
type size_t = usize;

// #[link(name = "ffts_static", kind = "static")]
extern "C" {
	pub fn ffts_init_1d_real(n: size_t, sign: c_int) -> *mut FftsPlan;
	pub fn ffts_init_2d_real(n1: size_t, n2: size_t, sign: c_int) -> *mut FftsPlan;
	pub fn ffts_init_nd_real(rank: c_int, ns: *const size_t, sign: c_int) -> *mut FftsPlan;

	pub fn ffts_init_1d(n: size_t, sign: c_int) -> *mut FftsPlan;
	pub fn ffts_init_2d(n1: size_t, n2: size_t, sign: c_int) -> *mut FftsPlan;
	pub fn ffts_init_nd(rank: c_int, ns: *const size_t, sign: c_int) -> *mut FftsPlan;

	pub fn ffts_execute(plan: *mut FftsPlan, input: *const c_float, output: *mut c_float);
	pub fn ffts_free(plan: *mut FftsPlan);
}
