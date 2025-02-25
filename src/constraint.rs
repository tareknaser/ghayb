use crate::signal::Signal;

/// The operations allowed in a constraint
#[derive(Clone, Debug)]
pub enum Operation {
    Add,
    Mul,
    Sub,
    Eq,
}

/// a single constraint with an operation
#[derive(Clone, Debug)]
pub struct Constraint {
    pub left: Signal,
    pub right: Signal,
    pub output: Signal,
    pub operation: Operation,
}
