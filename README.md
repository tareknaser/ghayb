# `Ghayb`

`Ghayb` ("غَيب" - meaning "unseen" in Arabic) aims to be a Domain-Specific Language for Zero-Knowledge Proofs.

At its current state, `Ghayb` implements some of the basic building blocks of zero-knowledge proof (ZKP) systems, including:

- Finite field elements
- Local constraint evaluation (currently only arithmetic constraints)
- Hash-based commitments
- A very minimal verification step where
  - The prover generates a proof by committing to the witness and validating constraints locally.
  - The verifier validates the proof by checking commitments and re-evaluating the circuit constraints.

## Example

```rust
use ghayb::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

fn simple_zk_example() {
    let c = circuit! {
        [x] -> [out],
        constraints = [
            // This is how to mark signals as Witness, Input (public by default), or Output
            constraint!((w@Witness + x@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 5,
        x = 10
    };

    let proof = prove!(c, w);

    let is_ok = verify!(c, [FieldElement::new(10, PRIME)], proof);

    println!("Proof verified: {}", is_ok);
    assert!(is_ok);
}
```
