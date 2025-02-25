use crate::field_element::FieldElement;

#[derive(Clone, Debug)]
pub enum Signal {
    /// Public input by name, e.g. "x" or "y"
    Input(String),
    /// Private witness by name, e.g. "secret1"
    Witness(String),
    /// An output signal by name, e.g. "out"
    Output(String),
}
