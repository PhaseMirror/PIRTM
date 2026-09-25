# ADR-069: Executable Proof Debt and Axiom Ledger Closure

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: ADR-019 where it claims a permanent zero-sorry state without a current-tree gate

## Context

`lake build --rehash` currently succeeds while Lean emits executable `sorry` warnings in:

- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:30`
- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:35`
- `lean/Foundations/ADR/FailClosedValidation.lean:59`
- `lean/Foundations/ADR/HundianSocialPhysics.lean:70`

The README, claim table, and axiom ledger still describe the core as zero-sorry. A successful build is therefore not evidence of a discharged proof.

The hidden assumption is that `sorry` is harmless when the surrounding theorem appears obvious or the module is Proposed. Phase Mirror treats every executable `sorry` as proof debt until it is replaced or explicitly recorded as an open obligation.

## Decision

1. No executable `sorry` may exist in `lean/` on a branch that marks Lean proofs Complete.
2. Every temporary obligation is recorded in `docs/PIRTM-axiom-ledger.md` with an `AX-*` identifier, affected theorem, impact, and closure evidence.
3. CI runs a hard failure when `grep -RIn 'sorry' lean/` finds any occurrence, including proof text and documentation strings.
4. ADR-069-001 through ADR-069-004 record the four current obligations. They close only after the proofs are replaced and `lake build --rehash` plus `lake test` pass without warnings.
5. A module with open proof debt is Partial or Broken in the claim table, regardless of whether Lake accepts it.

## Consequences

- Build success and proof completeness are distinct claims.
- Proof debt is visible in the axiom ledger before it can affect a status marker.
- Future regressions fail CI instead of silently entering a release artifact.

## Validation

- `grep -RIn 'sorry' lean/` returns no matches.
- `lake build --rehash` emits no `declaration uses sorry` warnings.
- `lake test` passes and the ledger entries identify the replacing proof terms.
