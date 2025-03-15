#![allow(unused)]

mod circuit;
mod commitment;
mod constraint;
mod dsl;
mod field_element;
mod proof;
mod prover;
mod signal;
mod verifier;
mod witness;

pub use circuit::Circuit;
pub use commitment::Commitment;
pub use constraint::{Constraint, Operation};
pub use dsl::*;
pub use field_element::FieldElement;
pub use proof::Proof;
pub use prover::Prover;
pub use signal::Signal;
pub use verifier::Verifier;
pub use witness::Witness;

#[cfg(test)]
mod tests;

// FIXME: A prime number in the thousands for now. Is this enough?
pub const PRIME: u128 = 7489;
