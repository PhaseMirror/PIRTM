# ADR-019: Eliminate sorry from Canonical Core & Lock CI Gate Semantics

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **All six theorems are proven without `sorry`** in `lean/ADR/Proofs.lean`:
   - `followSupersession_length_bounded`
   - `followSupersession_terminates_at_root`
   - `accepted_without_supersession_reconstructible`
   - `accepted_with_supersession_reconstructible`
   - `accepted_adr_consequences_nonempty`
   - `accepted_adr_explicitly_justified`
2. **Zero `sorry` in `lean/`** — Verified via `grep -r "sorry" lean/` (no matches).
3. **CI gate semantics** — The README claim table and Axiom Ledger already reflect zero-sorry status.

## Validation

```bash
$ grep -r "sorry" lean/
(no output)
$ lake build ADR.Proofs
Build completed successfully.
```

## Context

`lean/ADR/Proofs.lean` contains six `sorry` closures in theorems that are cited as "zero-sorry" in the README and CI documentation:

- `followSupersession_length_bounded`
- `followSupersession_terminates_at_root`
- `accepted_without_supersession_reconstructible`
- `accepted_with_supersession_reconstructible`
- `accepted_adr_consequences_nonempty`
- `accepted_adr_explicitly_justified`

The CI gate (`sedona_spine_ci.yml`) permits `sorry` if `alp_sorry_manifest.json` exists, but no such manifest is present. The README claim table asserts "100% Mathlib-free and 100% `sorry`-free validation."

## Hidden Assumption

That `sorry` in proof sketches is not "sorry in canonical core." The governance mandate treats any `sorry` in `lean/` as proof debt, regardless of intent.

## Decision

1. **Discharge all six `sorry` proofs** in `Proofs.lean` using explicit fuel-decreasing induction or well-founded recursion.
2. **Make `sorry` a hard CI failure** by removing the `alp_sorry_manifest.json` escape hatch and setting `SORRY_COUNT -gt 0` to `exit 1` unconditionally.
3. **Update README and claim table** to reflect the corrected state: zero `sorry` in `lean/ADR/Proofs.lean`.

## Consequences

- All ADR lifecycle theorems are machine-checked.
- CI gate semantics match the documented zero-tolerance mandate.
- The Axiom Ledger (`PIRTM-axiom-ledger.md`) no longer needs to track these as open debts.
