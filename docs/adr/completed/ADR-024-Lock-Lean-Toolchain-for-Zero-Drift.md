# ADR-024: Lock Lean Toolchain for Zero Drift

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`lake-manifest.json` locked**:
   - Set `"fixedToolchain": true` to prevent Lake from overriding the toolchain.
   - Added `"toolchain": "leanprover/lean4:v4.33.0-rc2"` to pin the exact version in the manifest.
2. **CI toolchain verification gate added** in `.github/workflows/sedona_spine_ci.yml`:
   - Reads the pinned version from `lean-toolchain`.
   - Compares it against `lean --version`.
   - Fails with `::error::` annotation if drift is detected.
3. **`lean-toolchain` remains pinned** at `leanprover/lean4:v4.33.0-rc2` (both root and `lean/lean-toolchain`).

## Validation

```yaml
# .github/workflows/sedona_spine_ci.yml
      - run: |
          EXPECTED=$(cat lean-toolchain | tr -d '\n')
          ACTUAL=$(lean --version | awk '{print $3}')
          if [ "$EXPECTED" != "$ACTUAL" ]; then
            echo "::error::Toolchain drift detected: expected $EXPECTED, got $ACTUAL"
            exit 1
          fi
```

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
