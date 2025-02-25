use std::collections::HashMap;

use crate::field_element::FieldElement;

/// Holds the prover’s secret assignments (mapping from signal names to their actual values)
#[derive(Clone, Debug)]
pub struct Witness {
    pub values: HashMap<String, FieldElement>,
}

impl Witness {
    pub fn new(values: HashMap<String, FieldElement>) -> Self {
        Self { values }
    }
}
