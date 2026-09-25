# ADR-069: Executable Proof Debt Is Never Invisible

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the zero-sorry portion of ADR-019

## Context

The current Lean tree contains executable `sorry` proof terms in:

- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:30` and `:35`
- `lean/Foundations/ADR/FailClosedValidation.lean:59`
- `lean/Foundations/ADR/HundianSocialPhysics.lean:70`

The README badge, claim table, and Axiom Ledger describe the core as zero-sorry. `lake build --rehash` and `lake test` succeed while Lean emits proof-debt warnings, so a passing build is not evidence of a discharged proof.

## Hidden Assumption

A theorem name in a test driver or a successful elaboration is treated as a proof. A `sorry` closes the elaboration without proving the proposition and therefore cannot satisfy an L0 claim.

## Decision

1. Executable `sorry` in `lean/` is a hard failure. Docstrings may not be used to hide proof debt.
2. Every outstanding obligation is recorded in `docs/PIRTM-axiom-ledger.md` with an `AX-*` identifier, affected proposition, impact, and closure condition.
3. A subsystem with an open proof obligation is Partial or Broken; it cannot be marked Complete.
4. CI runs `lake build --rehash`, `lake test`, and a zero-match proof-debt check after the check has been made precise enough to distinguish source text from generated build output.
5. ADR-057, ADR-061, and ADR-064 remain Partial until their affected proofs are discharged and the ledger entries are closed.

## Consequences

- Proof status is explicit and machine-auditable.
- A passing test driver cannot mask an unproved theorem.
- Future proof debt is visible before a claim is promoted to Complete.

## Validation

- `grep -RIn --include='*.lean' 'sorry' lean/` returns no matches.
- `lake build --rehash` emits no declaration-level sorry warnings.
- Every `AX-*` entry has a closure test or remains visibly open in the claim table.
