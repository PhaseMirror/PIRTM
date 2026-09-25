# ADR-067: Canonical Plonky3 Poseidon2 Integration & Toy Cryptography Removal

- **Status**: Proposed
- **Date**: 2026-09-22
- **Author**: Formal-methods / Crate steward
- **Decider**: PIRTM Architectural Review Board

## Context

The multiplicity-crypto and PIRTM packages previously operated on a toy mix of Poseidon2 with a hardcoded constraint count of 5,087 and boolean literals. This implementation did not pass the official Plonky3 known-answer tests and absorbed the commitment field into its own preimage, causing circularity. Furthermore, any fallback logic (such as checking seals against the M61 path instead of Goldilocks) created fragmentation and non-canonical behavior. 

To restore coherence, ensure exact cryptographic integrity, and align with canonical zero-knowledge standards, the toy implementations must be retired and officially vended Plonky3 crates must be integrated into the Rust workspace.

## Decision

1. **Retirement of Toy Poseidon2**:
   - Completely remove the toy Poseidon2 implementation in `packages/PIRTM/rust/pirtm-goldilocks/src/poseidon2.rs` along with its false 5,087 constraint outputs.
   - Remove any fallback checking logic, specifically the non-canonical M61 path, from the cryptographic pipeline.

2. **Integration of Canonical Plonky3 Crates**:
   - Vendor or wrap `p3-goldilocks` and `p3-poseidon2` within the Rust workspace.
   - Implement canonical `t = 8` compression (absorbing 8 elements and producing 4-element output receipts) securely using Plonky3's `default_goldilocks_poseidon2_8()`.
   
3. **Known-Answer Verification**:
   - Introduce specific known-answer tests (KAT) asserting that the Plonky3 width-8 vectors match byte-for-byte in the wrapped `Poseidon2Sponge` permutation, proving equivalence with the official protocol.

## Consequences

- **Positive**:
  - Restores absolute coherence by actively preventing the use of the toy hash.
  - Aligns the workspace with the official Plonky3 ecosystem, ensuring robust zero-knowledge snark receipts.
  - Prepares the protocol for production-grade governance and secure state progression.
- **Negative**:
  - Requires updating all dependent code that previously relied on the toy constraint count or M61 fallbacks to use the strict Goldilocks seal.
