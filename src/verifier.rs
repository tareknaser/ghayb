use std::collections::HashMap;

use crate::{
    circuit::Circuit,
    commitment::Commitment,
    constraint::{Constraint, Operation},
    field_element::FieldElement,
    proof::Proof,
    signal::Signal,
};

pub struct Verifier;

impl Verifier {
    ///  - checks that the commitments match the revealed
    ///  - check constraints
    pub fn verify(circuit: &Circuit, public_inputs: &[FieldElement], proof: &Proof) -> bool {
        if !Self::check_commitments(&proof.commitments, &proof.revealed_witness) {
            return false;
        }

        // a local map from name -> FieldElement
        let mut value_map: HashMap<String, FieldElement> = HashMap::new();
        for (name, (val, _blinding)) in &proof.revealed_witness {
            value_map.insert(name.clone(), val.clone());
        }

        // FIXME: assuming circuit.inputs correspond to public_inputs (meaning they are ordered)
        //        Is this a safe assumption?
        for (i, sig) in circuit.inputs.iter().enumerate() {
            if let Some(pub_val) = public_inputs.get(i) {
                // If sig is Input("x"), store "x" -> pub_val
                if let Signal::Input(ref name) = sig {
                    value_map.insert(name.clone(), pub_val.clone());
                }
            }
        }
        if !Self::constraints_satisfied(&circuit.constraints, &mut value_map) {
            return false;
        }

        true
    }

    /// For each named witness, reconstruct the commitment and compare
    fn check_commitments(
        commitments: &HashMap<String, Commitment>,
        revealed_witness: &HashMap<String, (FieldElement, FieldElement)>,
    ) -> bool {
        // sanity check
        if commitments.len() != revealed_witness.len() {
            eprintln!("Mismatch: #commitments != #revealed_witness");
            return false;
        }

        for (name, (val, blind)) in revealed_witness.iter() {
            // recompute the commitment
            let recomputed = Commitment::new(val, blind);
            let stored = match commitments.get(name) {
                Some(c) => c,
                None => {
                    eprintln!("No commitment stored for witness name '{}'", name);
                    return false;
                }
            };
            if recomputed.committed_hash != stored.committed_hash {
                eprintln!("Commitment mismatch for '{}'", name);
                return false;
            }
        }
        true
    }

    /// FIXME: This is exactly the same as the logic in Prover::is_satisfied
    ///        Should we refactor this into a shared function?
    fn constraints_satisfied(
        constraints: &[Constraint],
        value_map: &mut HashMap<String, FieldElement>,
    ) -> bool {
        for constraint in constraints {
            let left_val = Self::eval_signal(&constraint.left, value_map).unwrap();
            let right_val = Self::eval_signal(&constraint.right, value_map).unwrap();

            let result = match constraint.operation {
                Operation::Add => left_val.add(&right_val),
                Operation::Mul => left_val.mul(&right_val),
                Operation::Sub => left_val.sub(&right_val),
                Operation::Eq => {
                    if !left_val.equals(&right_val) {
                        eprintln!(
                            "Verifier constraint failed: {:?} != {:?}",
                            left_val, right_val
                        );
                        return false;
                    }
                    left_val.clone()
                }
            };

            match &constraint.output {
                Signal::Output(ref nm) | Signal::Witness(ref nm) => {
                    value_map.insert(nm.clone(), result);
                }
                _ => {}
            }
        }
        true
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
}
