# ADR-023: Enforce Grammar Quarantine via Separate Kernel Lexer

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-09-01

## Context

ADR-014 established a dual-grammar authority:
- `tree-sitter-pirtm` as the sole kernel authority for mathematical tokens.
- Pest/EBNF and `pirtm-parser` for application-level control flow.

However, `rust/pirtm-lexer/src/lib.rs` defines a single `Token` enum containing both kernel tokens (`Ap`, `tensor`, `assert_contractive`) and application control-flow tokens (`if`, `else`, `while`, `loop`, `fn`, `struct`, `enum`, `impl`, `match`). This violates the quarantine because the kernel grammar cannot be purified without breaking the application parser.

## Hidden Assumption

That a single lexer can serve both grammars without compromising kernel purity. In practice, the presence of control-flow tokens in the kernel lexer means any tool consuming the token stream cannot distinguish kernel from application tokens.

## Decision

1. **Split the lexer** into two crates:
   - `pirtm-kernel-lexer`: tokens for `tensor`, `Ap(n)`, `|>`, `assert_contractive`, `\Lambda_m`, `p_N`.
   - `pirtm-app-lexer`: tokens for `let`, `mut`, `if`, `else`, `while`, `loop`, `fn`, `struct`, `enum`, `impl`, `match`, etc.
2. **Update `pirtm-parser`** to use `pirtm-app-lexer` exclusively.
3. **Update `tree-sitter-pirtm`** to use `pirtm-kernel-lexer` exclusively.
4. **Remove control-flow tokens** from the kernel lexer grammar definition.

## Consequences

- Kernel and application grammars are physically separated in the build graph.
- `tree-sitter-pirtm` can be audited independently for mathematical purity.
- ADR-014's quarantine is enforced at the crate boundary, not just in documentation.
