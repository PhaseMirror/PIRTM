# ADR-055: Refuse f64 Float Scaling & Establish Exact Rational Constructor Membrane

- **Status**: Accepted
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods & Language Steward
- **Decider**: PIRTM Architectural Review Board
- **Replaces**: `0092f80e` (Reverted Float Scaling & Unresolved Merge Conflict)

## Executive Summary

ADR-055 ratifies elimination of IEEE 754 scaling membranes ($10^6$) and un-anchored `author_declared_lambda` defaults from the production constructor path. `Ensemble::from_rationals` is the canonical fail-closed constructor over GCD-reduced `PosRat` in $\mathbb{Q}$. The machine-checked gate is $\|G\|_1 < 1$.

Phase 1 code lands in a constructor cutover commit after this document. `Ensemble::new(f64)` remains deprecated SDK surface until Phase 2.

## Decision Contract

### 1. Phase 1 (Deprecation & Kernel Cutover)
- Mark `Ensemble::new(f64)` `#[deprecated]`.
- Canonical constructor:
  `from_rationals(name, Vec<Vec<(u64,u64)>>, Vec<(u64,u64)>, theorem_name) -> Result<Ensemble, EnsembleError>`.
- Store reduced `PosRat`. `(2,4)` and `(1,2)` are equal after construction.
- Empty `theorem_name` is `MissingTheoremAnchor`. No production default to `author_declared_lambda`.
- `pirtm-compiler` / `linker.rs`, Sentinel callers, and `pirtm-mcp` invoke `from_rationals` only.
- Manifest/JSON floats may be converted at the crate edge via `PosRat::from_f64_membrane`, never inside `from_rationals`.

### 2. Phase 2 (Hard Sunset)
Purge `Ensemble::new(f64)` at the first of:
- `2026-10-01T00:00:00Z`
- annotated tag `v1.0.1-mvp` on `PhaseMirror/PIRTM`

`v1.0.0-Stable` is forbidden by ADR-012. `v1.1.0-mvp` is not a Phase 2 trigger.

### 3. Boundary & Error Conditions
- `(p,0)` and `(0,0)`: `InvalidRational`
- $\lambda_j = 0/1$: allowed
- $\lambda_j < 0$ on the f64 path: `InvalidGain`
