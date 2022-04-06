//! This module implements Rescue hash function over the following fields
//! - bls12_377 base field
//! - ed_on_bls12_377 base field
//! - ed_on_bls12_381 base field
//! - ed_on_bn254 base field
//!
//! It also has place holders for
//! - bls12_381 base field
//! - bn254 base field
//! - bw6_761 base field
//!
//! Those three place holders should never be used.

mod param;
mod permutation;
mod rescue_constants;
mod sponge;
mod structs;

pub use param::{RescueParameter, RATE, ROUNDS, STATE_SIZE};
pub use permutation::{Permutation, PRP};
pub use structs::*;

#[derive(Clone, Default)]
/// A rescue hash function consists of a permutation function and
/// an internal state.
pub struct RescueHash<F: RescueParameter> {
    pub(crate) state: RescueVector<F>,
}