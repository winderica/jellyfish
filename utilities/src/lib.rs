#![no_std]

mod conversion;
mod hash_to_curve;
mod macros;
mod multi_pairing;
mod serialize;

use ark_ff::Field;
pub use ark_std::vec::Vec;

pub use conversion::*;
pub use hash_to_curve::*;
pub use macros::*;
pub use multi_pairing::*;
pub use serialize::*;

#[inline]
pub fn compute_len_to_next_multiple(len: usize, multiple: usize) -> usize {
    if len % multiple == 0 {
        len
    } else {
        len + multiple - len % multiple
    }
}

// Pad message with 0 until `msg` is multiple of `multiple`
#[inline]
pub fn pad_with_zeros<F: Field>(vec: &mut Vec<F>, multiple: usize) {
    let len = vec.len();
    let new_len = compute_len_to_next_multiple(len, multiple);
    vec.resize(new_len, F::zero())
}