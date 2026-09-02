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

2. **Lake Custom Script Generator Authority (`lakefile.lean`)**:
   - `lake_export_decls.json` is generated strictly by a Lake custom script target in `lakefile.lean` (`lake run export_decls`), querying `Lean.Environment` over compiled `.olean` artifacts during `lake build`.
   - External CI scripts, checked-in hand-edited JSON files, or secondary claim tables are strictly forbidden. The Lean 4 compiler frontend is the sole authority for declaration existence.
   - If `lake_export_decls.json` is missing or empty, `pirtmd` fails closed with `MissingLeanDecl`.

3. **Declaration Kind Filtering (Axiom Exclusion Rule)**:
   - `export_decls` emits proved non-axiom declarations (`theorem`, `lemma`, `def`, `structure`).
   - Raw `axiom` declarations and unproven axiom stubs are **strictly excluded** from `lake_export_decls.json`. A receipt anchor referencing an unproven `axiom` fails closed with `MissingLeanDecl`.

## Consequences

- Guarantees every certified receipt anchor corresponds to a real Lean 4 declaration.
- Decouples syntactic existence checks (Phase A, immediate) from deep type-checking (Phase B, subsequent).
