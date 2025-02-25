use crate::constraint::Constraint;
use crate::signal::Signal;

// a Circuit is a collection of constraints plus input and output signals
#[derive(Clone, Debug)]
pub struct Circuit {
    pub inputs: Vec<Signal>,
    pub outputs: Vec<Signal>,
    pub constraints: Vec<Constraint>,
}

impl Circuit {
    pub fn new(inputs: Vec<Signal>, outputs: Vec<Signal>, constraints: Vec<Constraint>) -> Self {
        Self {
            inputs,
            outputs,
            constraints,
        }
    }
}
