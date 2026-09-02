# ADR-062: Syntactic Lean Declaration Membership Gate

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

Certified contractivity receipts contain a `theorem_name` anchor string (e.g. `Foundations.ADR.BoundedIteration.iterate_non_expansive`). Currently, spectral certification verifies 1-norm contractivity $\|A\|_1 < 1$ in Rust, but does not check whether `theorem_name` exists as a machine-checked declaration in the Lean 4 proof substrate.

## Decision

1. **Two-Stage Verification Progression**:
   - **Phase A (Syntactic Membership Gate)**: `pirtmd` and `pirtm-engine` verify that `theorem_name` exists syntactically within a Lake-exported declaration registry (`lake_export_decls.json`). If `theorem_name` is absent from the registry, certification fails closed with `MissingLeanDecl`.
   - **Phase B (Semantic Type-Matching Gate)**: Future milestone verifying that `theorem_name`'s Lean 4 type signature matches the formal 1-norm contractivity predicate in `lean/MOC/Core.lean` / `PosRatContractivity.lean`.

2. **Lake Export Artifact Integration**:
   - `lake env` builds `lake_export_decls.json` during CI runs.
   - `Ensemble::from_rationals` checks syntactic membership against `lake_export_decls.json`.

## Consequences

- Guarantees every certified receipt anchor corresponds to a real Lean 4 declaration.
- Decouples syntactic existence checks (Phase A, immediate) from deep type-checking (Phase B, subsequent).
