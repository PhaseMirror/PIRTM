# ADR-051: Local PC Installation & Governed Developer Environment Protocol

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

To enable developers to install, execute, and program with PIRTM directly on local hardware without version drift or dynamic path instability, we require a sealed, reproducible local installation pipeline binding Lean 4 kernel verification and Rust workspace binary distribution.

## Decision

1. **Automated Workspace Installation (`install.sh`)**:
   - Verify toolchain requirements (`rustc` 1.81+, `elan`/Lean `v4.33.0-rc2`, `clang`/`LLVM 17+`).
   - Compile release binaries (`pirtm`, `pirtm-tools`, `pirtm-mcp`, `pirtm-monitor`, `pirtm-web-sdk`, `adr-verifier`, `lsp`).
   - Install and alias binaries into `~/.local/bin/` (`pirtm`, `pirtmc`, `pirtm-lsp`).
   - Run Lean kernel build and theorem verification via `./build.sh`.

2. **Lean 4 Formal Verification (`lean/Foundations/ADR/InstallationProtocol.lean`)**:
   - Formalize `InstallationState` and `verifyInstallation`.
   - Machine-check `installation_protocol_soundness` (0 `sorry`).

3. **Build Script Normalization (`build.sh`)**:
   - Extend `build.sh` to validate `lakefile.lean` alongside `lakefile.toml`.

## Consequences

- Full local installation of `pirtm`, `pirtmc`, `pirtm-mcp`, and `pirtm-lsp` in `~/.local/bin/`.
- Machine-checked zero-drift installation validation in Lean 4.
- Synchronized across Lean 4, Rust workspace, `install.sh`, and `registry.json`.
