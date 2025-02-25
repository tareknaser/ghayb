use std::collections::HashMap;

use crate::commitment::Commitment;
use crate::field_element::FieldElement;

/// the final “proof” object, containing:
/// - a map from signal name -> the committed values
/// - a map from signal name -> (value, blinding) that is revealed
///
/// In a real zk system, we shouldn't reveal the actual witness or blinding
/// Here we do so the verifier can re-check each commitment and constraint.
#[derive(Clone, Debug)]
pub struct Proof {
    /// For each private signal we store a named commitment
    pub commitments: HashMap<String, Commitment>,
    /// The (value, blinding) pairs for each private signal so the Verifier can re-check.
    pub revealed_witness: HashMap<String, (FieldElement, FieldElement)>,
}
