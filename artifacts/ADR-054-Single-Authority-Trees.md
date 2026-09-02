# ADR-054: Single Authority Trees (Consolidation of Dual Roots & Dual Lexers)

- **Status**: Accepted
- **Deciders**: Systems Architect, Formal Methods Steward
- **Date**: 2026-09-02

## Context
The repository contained dual Lean roots (`lean/ADR` and `lean/Foundations/ADR`), dual lexers/parsers (`pirtm-lexer` vs `pirtm-kernel-lexer`), and mismatched workspace definitions in `Cargo.toml` vs directory structures. Dual authorities create governance ambiguity over which tree constitutes canonical law.

## Decision
1. **Single Lean Root**: `lean/` is established as the sole canonical Lean 4 root configured via a single `lakefile.lean`. Secondary roots under `lean/Foundations` or `lean/ADR` are consolidated or removed.
2. **Single Lexer Authority**: Per ADR-014, `tree-sitter-pirtm` is the sole authority for kernel language tokens, while the Rust compiler pipeline uses a unified `pirtm-lexer` crate.
3. **Workspace Cargo Alignment**: All Rust crates on disk must be explicitly enumerated as workspace members in `Cargo.toml`. Discrepancies between disk directories and manifest members are forbidden.

## Consequences
- Governance ambiguity between dual roots is eliminated.
- Lake build and Cargo build commands operate deterministically from single top-level configuration files.
