# ADR-073: Rust Workspace and Dependency Boundary

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board

## Context

The Rust workspace declares 24 members, while `rust/pirtm-clinical/` exists on disk but is not a member and cannot be tested directly because it believes it belongs to the parent workspace. Its package name is `c_pirtm_rs`, not `pirtm-clinical`. Several `#[cfg(kani)]` harnesses compile as zero tests under ordinary Cargo, and orphaned orchestrator sources are present without a clear module boundary.

The hidden assumption is that a directory's existence or a Cargo build's success means the crate and its proofs are in the verification boundary. Neither is true.

## Decision

1. Every production crate is either a workspace member or explicitly listed in `workspace.exclude` with a documented reason.
2. `pirtm-clinical` is added as a workspace member under its declared package identity, or removed from the tree if it is not a supported product. It must not remain an untested shadow crate.
3. Workspace member count and package names are asserted by CI from `cargo metadata`; documentation uses the same values.
4. Ordinary `cargo test --workspace --all-targets` is the baseline. Kani harnesses are a separately named optional gate and cannot be described as verified by ordinary Cargo.
5. Source files that are not reachable from a crate root are either wired into a module or moved to an explicitly quarantined directory; orphaned production-looking code cannot satisfy a claim.
6. Crate-level package names must match the documented invocation form, or the documentation must use the actual package name.

## Consequences

- No supported crate can hide outside the workspace test boundary.
- Kani evidence is reported with the correct scope.
- Documentation and CI derive crate counts from one metadata source.
- Orphaned code is either maintained and tested or explicitly excluded from ground-truth claims.

## Validation

- `cargo metadata` lists every supported crate exactly once.
- `cargo test --workspace --all-targets` includes the clinical crate when it is supported.
- CI reports Kani as optional unless `cargo kani` actually runs.
- No orphaned source file is cited as implementation evidence.
