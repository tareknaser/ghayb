# `Ghayb`

> The project was developed as part of CMPT 982: Special Topics in Networks and Systems

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

## Formal Semantics

### Commitment Scheme

The commitment scheme uses **SHA-256** to bind witness values with blinding factors:

```math
C = \text{SHA256}(\text{val}_{\text{bytes}} \parallel \text{blind}_{\text{bytes}} \parallel \text{prime}_{\text{bytes}})
```

For each revealed witness _(s, w<sub>s</sub>, r<sub>s</sub>)_, the verifier checks:

```math
H(w_s, r_s, p) \stackrel{?}{=} \mathcal{K}(s)
```

where:

- **_H_** is the cryptographic hash function
- **_𝒦(s)_** is the stored commitment for signal _s_

### Verification Inference Rules

The verification process follows formal inference rules. The complete verification succeeds when **all three conditions** hold:

#### Success Rule

<div align="center">

```
    P ⇓_commit true     (C, P) ⇓_map σ     ℂ, σ ⇓_constraints true
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                            C, I, P ⇓ true
```

</div>

#### Failure Rule

<div align="center">

```
    P ⇓_commit false   ∨   ℂ, σ ⇓_constraints false
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                    C, I, P ⇓ false
```

</div>

#### **Legend**

| Symbol | Description                              |
| ------ | ---------------------------------------- |
| **ℂ**  | Circuit (constraints, inputs, outputs)   |
| **I**  | Set of public inputs                     |
| **P**  | Proof (commitments + revealed witnesses) |
| **σ**  | Value mapping: _S → 𝔽<sub>p</sub>_       |
| **⇓**  | Evaluation judgment                      |

### Constraint Evaluation

For each constraint _(s₁ ⊕ s₂ → s₃) ∈ ℂ_:

1. **Retrieve** values: _v₁ = σ(s₁)_, _v₂ = σ(s₂)_
2. **Compute** result: _v₃ = v₁ ⊕ v₂ mod p_
3. **Verify** output: _σ(s₃) ≟ v₃_

> See `semantics.pdf` for more details.
