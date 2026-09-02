# ADR-052: Rejection of v3.0.0 Claim Inflation and False Completeness

- **Status**: Accepted
- **Deciders**: Formal Methods Steward, Compiler Engineering, Governance Steward
- **Date**: 2026-09-02

## Context
Commit `11e45a5` designated the PIRTM repository as `v3.0.0` in `README.md` while `CHANGELOG.md` listed `1.0.0-mvp` and `lakefile.lean` listed `0.1.0`. Furthermore, badges pointed to non-existent workflow files (e.g., `governed_toolchain.yml`), and claim counts fluctuated across documentation files (7, 18, 38, 50). This represents a violation of ADR-015 ("No Phantom Claims / No Claim Inflation").

## Decision
1. **Unification of Version Identifiers**: The repository version string must be single-sourced across `README.md`, `CHANGELOG.md`, `lakefile.lean`, and `Cargo.toml`. Speculative version jumping to `v3.0.0` without green formal verification gates is strictly rejected.
2. **Badge-to-Tree Parity**: Every badge URL in `README.md` must link directly to an existing, active workflow file in `.github/workflows/`. Badges with missing workflow targets must be demoted or removed immediately.
3. **Claim Table Synchronization**: Every item marked `[x]` in `PIRTM-README-Claim-Table.md` must cite an executable unit test, Lean theorem without `sorry`, or Kani harness that passes on CI.

## Consequences
- README claims are demoted to match empirical test coverage.
- Single source of truth for versioning is restored.
- CI pipeline enforcement prevents merging unverified badges.
