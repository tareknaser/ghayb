use sha2::{Digest, Sha256};

use crate::field_element::FieldElement;

/// hash-based commitment (hides the actual value behind a hash)
#[derive(Clone, Debug)]
pub struct Commitment {
    pub committed_hash: Vec<u8>,
}

impl Commitment {
    /// C = SHA256( val_bytes || blind_bytes || prime_bytes )
    pub fn new(val: &FieldElement, blinding: &FieldElement) -> Self {
        let val_bytes = val.value.to_le_bytes();
        let blind_bytes = blinding.value.to_le_bytes();

        let mut hasher = Sha256::new();
        hasher.update(&val_bytes);
        hasher.update(&blind_bytes);
        hasher.update(&val.prime.to_le_bytes());

        let result = hasher.finalize();
        Self {
            committed_hash: result.to_vec(),
        }
    }
}
