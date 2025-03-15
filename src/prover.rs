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
        let mut value_map: HashMap<String, FieldElement> = witness.values.clone();

        // Compute all intermediate witness values before committing
        for constraint in &circuit.constraints {
            let left_val = Self::eval_signal(&constraint.left, &value_map).unwrap();
            let right_val = Self::eval_signal(&constraint.right, &value_map).unwrap();

            let result = match constraint.operation {
                Operation::Add => left_val.add(&right_val),
                Operation::Mul => left_val.mul(&right_val),
                Operation::Sub => left_val.sub(&right_val),
                Operation::Eq => {
                    assert!(left_val.equals(&right_val), "Constraint equation failed!");
                    left_val.clone()
                }
            };

            match &constraint.output {
                Signal::Output(out_name) | Signal::Witness(out_name) => {
                    value_map.insert(out_name.clone(), result);
                }
                _ => {}
            }
        }

        // Now create commitments
        let mut commitments = HashMap::new();
        let mut revealed_witness = HashMap::new();

        let witness_signal_names = Self::collect_witness_names(circuit);
        for name in witness_signal_names {
            if let Some(value) = value_map.get(&name) {
                let mut rng = rand::thread_rng();
                let random_u128: u128 = rng.gen();
                let blinding = FieldElement::new(random_u128, value.prime);
                let commitment = Commitment::new(value, &blinding);

                commitments.insert(name.clone(), commitment);
                revealed_witness.insert(name.clone(), (value.clone(), blinding));
            } else {
                panic!("Missing witness value for signal '{}'", name);
            }
        }

        Proof {
            commitments,
            revealed_witness,
        }
    }

    fn eval_signal(
        signal: &Signal,
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
