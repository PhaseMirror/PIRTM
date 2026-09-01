# ADR-024: Lock Lean Toolchain for Zero Drift

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

`lake-manifest.json` contains `"fixedToolchain": false`. The `lean-toolchain` file pins `leanprover/lean4:v4.33.0-rc2`, but Lake will silently upgrade or downgrade Lean if the manifest permits. The Sedona Spine mandate (CONTRACT.md §3) requires "Zero Drift" across all formal artifacts.

## Hidden Assumption

That the `lean-toolchain` file alone guarantees toolchain stability. In reality, `fixedToolchain: false` allows Lake to override the toolchain based on package dependencies or user environment.

## Decision

1. **Set `"fixedToolchain": true`** in `lake-manifest.json`.
2. **Pin the toolchain hash** in CI by installing the exact version via `elan` and verifying `lean --version` matches.
3. **Add a CI gate** that fails if `lake build` reports a toolchain override or if `lean --version` differs from `lean-toolchain`.

## Consequences

- All developers and CI runners use the identical Lean binary.
- Proofs cannot diverge due to library version drift.
- The zero-drift mandate is enforced at the package-manager level.
