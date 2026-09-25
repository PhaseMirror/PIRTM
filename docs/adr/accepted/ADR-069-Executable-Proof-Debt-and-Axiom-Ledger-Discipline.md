# ADR-069: Executable Proof Debt and Axiom Ledger Discipline

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: The zero-sorry portion of ADR-019 where it conflicts with this decision

## Context

`lake build --rehash` currently emits proof warnings for executable `sorry` terms in:

- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:30` and `:35`
- `lean/Foundations/ADR/FailClosedValidation.lean:59`
- `lean/Foundations/ADR/HundianSocialPhysics.lean:70`

The README, claim table, and the Axiom Ledger and claim table and README claim that the core is axiom-clean and the README and the claim table and ledger and README and the README and claim table.

The hidden assumption is that a successful Lake build means a proof is complete. Lean accepts `sorry` as an axiom-like proof term, so build success is not proof completion.

## Decision

1. No executable `sorry` in `lean/`
2. Every executable `sorry` in `lean/`
3. A claim that depends on a module containing `sorry` is Partial or Broken until the obligation is closed and the ledger entry is resolved.
4. The CI workflow must fail if any executable `sorry` remains. Documentation references to the word are not proof terms, but the canonical audit will keep the source free of both to make the gate unambiguous.
5. The Axiom Ledger records the obligation, affected claim, closure condition, and verifying command.

## Consequences

- Lean build success and proof completion are no longer conflated.
- ADR-057, ADR-061, and ADR-064 cannot be represented as fully verified while their proof modules contain `sorry`.
- The claim table and README must expose proof debt instead of hiding it behind a green build badge.
