use std::collections::HashMap;

use crate::{
    Circuit, Constraint, FieldElement, Operation, Proof, Prover, Signal, Verifier, Witness, PRIME,
};

#[test]
fn simple_zk_example() {
    let x_signal = Signal::Input("x".to_string());
    let w_signal = Signal::Witness("w".to_string());
    let out_signal = Signal::Output("out".to_string());

    // constraint: w + x = out
    let constraint = Constraint {
        left: w_signal.clone(),
        right: x_signal.clone(),
        output: out_signal.clone(),
        operation: Operation::Add,
    };

    let circuit = Circuit::new(
        vec![x_signal.clone()], // public input
        vec![out_signal.clone()],
        vec![constraint],
    );

    let w_value = FieldElement::new(5, PRIME);
    let x_value = FieldElement::new(10, PRIME);

    // build witness (w=5, x=10)
    let mut witness_map = HashMap::new();
    witness_map.insert("w".to_string(), w_value);
    witness_map.insert("x".to_string(), x_value.clone());

    let witness = Witness::new(witness_map);

    // prover generates a proof
    let proof: Proof = Prover::prove(&circuit, &witness);

    // verifier checks the proof
    let public_inputs = vec![x_value];
    let is_valid = Verifier::verify(&circuit, &public_inputs, &proof);

    println!("Proof verification result: {}", is_valid);
    assert!(is_valid);
}
