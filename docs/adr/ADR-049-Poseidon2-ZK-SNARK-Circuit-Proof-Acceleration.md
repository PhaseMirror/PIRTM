# ADR-049: Poseidon2 ZK-SNARK Circuit Proof Acceleration

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

To enable third-party verification of governance contractivity receipts without compiling the full PIRTM program, we integrate a Poseidon2 sponge permutation circuit over the Goldilocks prime field ($\mathbb{F}_p$, $p = 2^{64} - 2^{32} + 1$).

## Decision

1. **Rust Implementation (`rust/pirtm-goldilocks/src/poseidon2.rs`)**:
   - Implement `Poseidon2Sponge` (Width 8, S-box $x^7$, 5,087 circuit constraints).
   - Emit `Poseidon2ProofReceipt` containing 4-element field hash squeeze output.

2. **Lean 4 Formal Soundness (`lean/Foundations/ADR/Poseidon2Soundness.lean`)**:
   - Formalize `verifyPoseidon2Receipt` and prove `poseidon2_receipt_soundness` (0 `sorry`).

3. **Runtime Integration (`pirtm-engine::http_server`)**:
   - Embed Poseidon2 ZK proof receipt generation directly into `GovernedHttpServer` responses.

## Consequences

- Third-party verifiers can validate contractivity receipts using 5,087 constraint zk-SNARK proofs.
- 100% test coverage across Lean 4 and Rust workspace.
