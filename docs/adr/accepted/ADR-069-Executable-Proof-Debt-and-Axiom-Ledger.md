# ADR-069: Executable Proof Debt and Axiom Ledger

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the zero-sorry completion claim in ADR-019 where current-tree evidence conflicts

## Context

The current Lean tree contains executable `sorry` terms in:

- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:30`
- `lean/Foundations/ADR/LexicalHeaderSplitter.lean:35`
- `lean/Foundations/ADR/FailClosedValidation.lean:59`
- `lean/Foundations/ADR/HundianSocialPhysics.lean:70`

The README badge, claim table, and Axiom Ledger nevertheless describe the core as zero-sorry. The hidden assumption is that a successful Lake build proves the proof obligations are discharged. Lake accepts `sorry` and emits warnings; therefore build success is not proof completeness.

## Decision

1. No executable `sorry` is permitted in `lean/`.
2. Every outstanding proof obligation is recorded in `docs/PIRTM-axiom-ledger.md` before a containing claim may be marked Complete.
3. The current obligations are registered as `AX-069-001` through `AX-069-004` and map to the four executable occurrences above.
4. CI fails if any `sorry` token remains in Lean source, including proof bodies and proof-debt documentation that could be mistaken for executable code.
5. ADR-057 and ADR-061 remain Partial until their associated proofs are discharged. ADR-064 remains Proposed and cannot satisfy a Complete claim while its proof is unfinished.
6. Proof modules must link to the physical theorem and the test or build receipt that exercises it.

## Consequences

- A warning-producing build can no longer be reported as axiom-clean.
- Proof debt is visible, countable, and attributable.
- The claim table can distinguish a machine-checked theorem from a theorem that merely elaborates with an axiom.
- The Axiom Ledger becomes the authoritative index for unresolved obligations.

## Validation

- `grep -RIn 'sorry' lean/` returns no matches.
- `lake build --rehash` and `lake test` pass without proof warnings.
- The ledger contains one closed or open record for every proof obligation introduced by a change.
