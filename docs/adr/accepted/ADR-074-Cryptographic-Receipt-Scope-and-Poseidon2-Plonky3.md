# ADR-074: Cryptographic Receipt Scope and Poseidon2/Plonky3

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the ZK-soundness completion claim in ADR-049 where current-tree evidence conflicts

## Context

`lean/Foundations/ADR/Poseidon2Soundness.lean` states that it does not define a sponge, field, constraint system, or knowledge soundness. Its theorem is a conjunction of author-supplied Boolean flags. The README and generated ADR-049 nevertheless describe Poseidon2 ZK-SNARK soundness and a 5,087-constraint circuit. The current Rust integration in `rust/pirtm-goldilocks/src/poseidon2.rs` is a permutation/hash wrapper, and the untracked ADR-067 proposes canonical Plonky3 integration.

The hidden assumption is that a cryptographic permutation output, a hardcoded constraint count, or a Boolean receipt flag constitutes a zero-knowledge proof. It does not.

## Decision

1. ADR-049 is rebaselined as Defect for ZK soundness. It may describe a permutation or receipt encoding only after its title and claims are narrowed.
2. ADR-067 remains Proposed until canonical Plonky3 known-answer tests, field arithmetic, circuit constraints, verifier acceptance, and receipt binding are physically on tree and tested.
3. A hash output from Poseidon2/Plonky3 is labeled a digest or commitment output, not a ZK proof, unless a verifier and soundness argument are present.
4. A Complete cryptographic claim requires a machine-checked or independently verified constraint-system proof, a non-tautological theorem, and a receipt that binds the statement, witness commitment, and verifier transcript.
5. Existing HTTP and engine paths must not advertise `POSEIDON2-ZK-SNARK-RECEIPT` or equivalent wording for a permutation-only result.

## Consequences

- Cryptographic scope is explicit and falsifiable.
- The current implementation can be used as a digest primitive without inheriting unsupported ZK claims.
- ADR-067 has a measurable acceptance gate instead of an aspirational completion marker.
- The claim table and README stop treating a Boolean flag conjunction as knowledge soundness.

## Validation

- ADR-049 is marked Defect in the claim table and registry.
- No production label claims ZK proof generation for the current wrapper.
- ADR-067 moves to Accepted only after its KAT, verifier, constraint, and receipt-binding tests pass.
- The Lean proof module contains no tautological substitute for the claimed soundness property.
