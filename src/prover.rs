use std::collections::HashMap;

use rand::Rng;

use crate::{
    circuit::Circuit, commitment::Commitment, constraint::Operation, field_element::FieldElement,
    proof::Proof, signal::Signal, witness::Witness,
};

/// The prover:
/// - Generates the proof
/// - does a local constraint check
/// - commits to each witness signal
#[derive(Clone, Debug)]
pub struct Prover;

impl Prover {
    #[allow(deprecated)]
    pub fn prove(circuit: &Circuit, witness: &Witness) -> Proof {
        // local constraint check
        let satisfied = Self::is_satisfied(circuit, witness);
        assert!(satisfied, "Circuit constraints not satisfied");

        // a map of signal_name -> Commitment
        let mut commitments = HashMap::new();
        // Also a map of signal_name -> (value, blinding) - THIS IS NOT ZK
        let mut revealed_witness = HashMap::new();

        let witness_signal_names = Self::collect_witness_names(circuit);
        // For each witness signal, commit
        for name in witness_signal_names {
            if let Some(value) = witness.values.get(&name) {
                // random blinding
                let mut rng = rand::thread_rng();
                let random_u128: u128 = rng.gen();
                // FIXME: should the blinding be a FieldElement?
                let blinding = FieldElement::new(random_u128, value.prime);

                let commitment = Commitment::new(value, &blinding);

                commitments.insert(name.clone(), commitment);
                revealed_witness.insert(name.clone(), (value.clone(), blinding));
            } else {
                // FIXME: Better error handling
                panic!("Missing witness value for signal '{}'", name);
            }
        }

        Proof {
            commitments,
            revealed_witness,
        }
    }

    fn is_satisfied(circuit: &Circuit, witness: &Witness) -> bool {
        let mut value_map: HashMap<String, FieldElement> = witness.values.clone();

        for constraint in &circuit.constraints {
            let left_val = Self::eval_signal(&constraint.left, &value_map).unwrap();
            let right_val = Self::eval_signal(&constraint.right, &value_map).unwrap();

            let result = match constraint.operation {
                Operation::Add => left_val.add(&right_val),
                Operation::Mul => left_val.mul(&right_val),
                Operation::Sub => left_val.sub(&right_val),
                Operation::Eq => {
                    if !left_val.equals(&right_val) {
                        return false;
                    }
                    left_val.clone()
                }
            };

            // If the result is an output or witness, update the map
            match &constraint.output {
                Signal::Output(out_name) | Signal::Witness(out_name) => {
                    value_map.insert(out_name.clone(), result);
                }
                _ => {}
            }
        }

        true
    }

    fn eval_signal(
        signal: &Signal,
        // witness values
        value_map: &HashMap<String, FieldElement>,
    ) -> Option<FieldElement> {
        match signal {
            Signal::Input(name) => value_map.get(name).cloned(),
            Signal::Witness(name) => value_map.get(name).cloned(),
            Signal::Output(name) => value_map.get(name).cloned(),
        }
    }

    fn collect_witness_names(circuit: &Circuit) -> Vec<String> {
        let mut names = Vec::new();
        for constraint in &circuit.constraints {
            for sig in [&constraint.left, &constraint.right, &constraint.output] {
                if let Signal::Witness(nm) = sig {
                    if !names.contains(nm) {
                        names.push(nm.clone());
                    }
                }
            }
        }
        names
    }
}
