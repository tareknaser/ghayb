# `Ghayb`

`Ghayb` ("غَيب" - meaning "unseen" in Arabic) aims to be a Domain-Specific Language for Zero-Knowledge Proofs.

At its current state, `Ghayb` implements some of the basic building blocks of zero-knowledge proof (ZKP) systems, including:

- Finite field elements
- Local constraint evaluation (currently only arithmetic constraints)
- Hash-based commitments
- A very minimal verification step where
  - The prover generates a proof by committing to the witness and validating constraints locally.
  - The verifier validates the proof by checking commitments and re-evaluating the circuit constraints.
