# ADR-018: Repair Broken Lean Kernel Import Graph

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

`lean/PIRTM.lean` is the canonical mathematical kernel for PIRTM/MOC. It imports four modules (`prime_tensors.Transition`, `prime_tensors.Stability`, `prime_tensors.CPIRTM`, `prime_tensors.DRMM`) that do not exist anywhere in the repository. The CI gate (`sedona_spine_ci.yml`) only builds the `ADR` target, never `PIRTM`, so this breakage is invisible to automated enforcement.

## Decision

1. **Implement `prime_tensors` modules** under `lean/PrimeTensors/` with the exact names imported by `PIRTM.lean`:
   - `Transition.lean`
   - `Stability.lean`
   - `CPIRTM.lean`
   - `DRMM.lean`
2. **Add `PIRTM` to `defaultTargets`** in `lakefile.toml` so `lake build` validates the kernel.
3. **Extend CI Gate 3** to run `lake build PiLang` (or the project name) and fail on any missing import.

## Consequences

- The kernel becomes buildable and machine-checked.
- CI enforces kernel integrity, not just ADR scaffolding.
- Any future import breakage fails fast.
