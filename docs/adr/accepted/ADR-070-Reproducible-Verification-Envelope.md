# ADR-070: Reproducible Verification Envelope

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the toolchain and CI completion claims in ADR-024 and ADR-001 where current-tree evidence conflicts

## Context

`lean-toolchain` pins `leanprover/lean4:v4.33.0-rc2`, but the active local Lake binary is Lean 4.34.0-rc2 and `lake-manifest.json` contains `"fixedToolchain": false`. The active configuration is `lakefile.lean`; the repository also contains a stale `lakefile.toml.bak`. The CI workflow only checks the Lean version and runs `lake build`.

The hidden assumption is that a version check plus one build is equivalent to full verification. It is not: the current gate does not run Lean tests, Rust tests, proof-debt detection, artifact synchronization, or claim-table integrity checks.

## Decision

1. `lakefile.lean` is the active Lake configuration. `lakefile.toml.bak` is historical and must not be cited as the active configuration.
2. The active Lake configuration builds the canonical `Foundations`, `PIRTM`, and `prime_tensors` source roots and the Lean test driver.
3. `lake-manifest.json` must contain `"fixedToolchain": true`; CI installs and verifies the exact `lean-toolchain` value before any build.
4. The Sedona Spine CI job must run, in order:
   - `lake build --rehash`
   - `lake test`
   - a hard Lean `sorry` scan
   - `cargo test --workspace --all-targets`
   - canonical artifact comparison and SHA-256 sidecar verification
   - fixed-toolchain and ADR identity checks
5. A CI pass is evidence only for the commands actually run. Kani harnesses remain an explicitly optional gate until the CI runner installs and invokes Kani.
6. `build.sh` must invoke the same verification envelope or fail closed.

## Consequences

- Toolchain drift is rejected at package resolution and at CI entry.
- Lean proof tests and Rust workspace tests are part of the governance gate rather than local folklore.
- A green badge cannot coexist with known `sorry`, artifact drift, or skipped runtime tests.
- The active configuration and its historical backup have unambiguous authority.

## Validation

- `lake build --rehash` and `lake test` pass on the pinned toolchain.
- `cargo test --workspace --all-targets` passes.
- CI fails on toolchain drift, proof debt, workspace failure, or mirror drift.
- The manifest reports `fixedToolchain: true`.
