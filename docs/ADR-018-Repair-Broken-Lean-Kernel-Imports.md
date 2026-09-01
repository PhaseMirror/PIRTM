# ADR-018: Repair Broken Lean Kernel Import Graph

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`prime_tensors` modules implemented** under `lean/prime_tensors/`:
   - `Transition.lean` — fixed `def PrimeIndices` → `abbrev PrimeIndices` to propagate `Membership` instance; corrected proof terms.
   - `Stability.lean` — added self-contained `iterate` definition; rewrote `spectral_stable_of_length_non_increasing` with proper induction generalizing `s`; established composition stability via length-non-increasing composition lemma.
   - `CPIRTM.lean` — removed `deriving Repr` from `CPIRTMKernel` (function types lack `Repr` in core Lean).
   - `DRMM.lean` — already valid; no changes needed.
2. **`prime_tensors.lean` root file created** at `lean/prime_tensors.lean` to satisfy Lake's multi-file library requirement under `srcDir = "lean"`.
3. **`lakefile.toml` updated**:
   - Added `[[lean_lib]] name = "prime_tensors"` and `[[lean_lib]] name = "PIRTM"`.
   - Extended `defaultTargets` to `["ADR", "prime_tensors", "PIRTM"]`.
4. **CI gate created** at `.github/workflows/sedona_spine_ci.yml`:
   - Triggers on push/PR to `main`.
   - Sets up Lean toolchain `leanprover/lean4:v4.33.0-rc2`.
   - Runs `lake build`, which now validates the kernel.

## Validation

```bash
$ lake build
Build completed successfully (18 jobs).
```

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
