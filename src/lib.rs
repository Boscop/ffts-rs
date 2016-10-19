//! ffts-rs provides Rust bindings for [FFTS](http://anthonix.com/ffts)
//! 
//! Complex data is stored in the interleaved format
//! (i.e, the real and imaginary parts composing each
//! element of complex data are stored adjacently in memory).
//! 
//! The multi-dimensional arrays passed are expected to be
//! stored as a single contiguous block in row-major order.
//! 
//! For real transforms, dir == Dir::Forward implies a real-to-complex
//! forwards tranform, and dir == Dir::Backward implies a complex-to-real
//! backwards transform.
//! 
//! The output of a real-to-complex transform is N/2+1 complex numbers,
//! where the redundant outputs have been omitted.

extern crate libc;

mod ffi;
use ffi::*;

pub struct Fft {
	plan: *mut FftsPlan
}

/// The direction of the transform
/// (i.e, the sign of the exponent in the transform.)
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Dir {
	Forward = FFTS_FORWARD,
	Backward = FFTS_BACKWARD,
}

/// use one of the constructors to set up a FFT, the call execute() on it
impl Fft {
	pub fn new_1d_complex(n: usize, dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_1d(n, dir as i32)}}
	}
	pub fn new_2d_complex(n1: usize, n2: usize, dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_2d(n1, n2, dir as i32)}}
	}
	pub fn new_nd_complex(rank: usize, ns: &[usize], dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_nd(rank as i32, ns.as_ptr(), dir as i32)}}
	}
	pub fn new_1d_real(n: usize, dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_1d_real(n, dir as i32)}}
	}
	pub fn new_2d_real(n1: usize, n2: usize, dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_2d_real(n1, n2, dir as i32)}}
	}
	pub fn new_nd_real(rank: usize, ns: &[usize], dir: Dir) -> Fft {
		Fft {plan: unsafe {ffts_init_nd_real(rank as i32, ns.as_ptr(), dir as i32)}}
	}
	pub fn execute(&mut self, input: &[f32], output: &mut [f32]) {
		unsafe {ffts_execute(self.plan, input.as_ptr(), output.as_mut_ptr())}
	}
}

impl Drop for Fft {
	fn drop(&mut self) {
		unsafe {ffts_free(self.plan)}
	}
}