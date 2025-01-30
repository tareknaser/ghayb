# `Ghayb`: A Domain-Specific Language for Zero-Knowledge Proofs

## Domain and Motivation

Zero-knowledge (ZK) proof systems have become an essential component in modern cryptographic applications, particularly in scenarios demanding strong privacy guarantees. These systems enable a prover to demonstrate knowledge of a statement's truth without exposing any additional information.

A zero-knowledge proof (ZKP) is a cryptographic protocol where a prover seeks to convince a verifier of a claim’s validity without disclosing any underlying information. This makes ZKPs a valuable tool in privacy-preserving technologies that allow secure authentication.

Their applications extend across various domains, with blockchain technology being one of the most famous adopters. Privacy-centric cryptocurrencies like ZCash rely on ZKPs for confidential transactions, and layer-2 scalability solutions such as ZK-rollups use them to enable efficient, trustless verification.

Implementing zero-knowledge proofs remains complex and error-prone due to mathematical foundations and the challenges of designing efficient circuits. Writing correct and optimized ZKP implementations requires expertise in cryptographic primitives and computational efficiency.

`Ghayb` is a domain-specific language (DSL) made for expressing and verifying zero-knowledge proofs in a structured, intuitive manner. By abstracting away the complexities of circuit construction and proof composition, `Ghayb` aims to provide a high-level interface for developers to work with ZKPs safely and efficiently.

### Scope of the Project

The project is designed to be a minimal DSL for zero-knowledge proofs. The core functionalities to be implemented include:

1. MVP
	- Allows defining circuits with arithmetic constraints, submitting proofs with public and private signals, and verifying proof validity without parsing
2. Core DSL Implementation 
    - Constraints:
        - Boolean constraints (equality and logical conditions).
        - Modular arithmetic (for cryptographic proofs).
    - Advanced Features:
        - Support for multiple proofs against a single circuit execution?
        - Composite constraints (combining multiple conditions in a proof statement)?
        - Move to tagless final
    - Meaningful Examples:
        - Verifying a hidden number
        - Proving knowledge of a secret key
        - Other reasonable features that align with ZKP use cases
3. Finalization and Benchmarking
    - Formalizing operational semantics.
    - Running benchmarks and evaluating performance.
    - Preparing documentation and final presentation.
> What other meaningful features to have?
## Technologies and Approaches
- Language: Rust
- Semantics: operational semantics (small-step/big-step)
- The Finally Tagless, Partially Evaluated can be applied
- Parsing is out of scope

## Milestones and Project Timeline

| Date Range                | Milestone                                                                                                                           |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Feb 10 - Feb 24 (2 weeks) | Literature review, ZKP study, and MVP implementation (basic circuit constraints, proof submission, and verification) - Get feedback |
| Feb 25 - Mar 24 (4 weeks) | Core DSL development: advanced constraints, modular arithmetic, composite constraints, and meaningful examples                      |
| Mar 25 - Apr 7 (2 weeks)  | Semantics definition, benchmarks, and presentation preparation                                                                      |

