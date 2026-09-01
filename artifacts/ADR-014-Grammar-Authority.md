# ADR-014: Dual-Grammar Authority and Control-Flow Quarantine

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-08-31

## Context
Two parallel grammar definitions existed in the repository:
1. `tree-sitter-pirtm`: Defined for core prime operators, tensor sheaf declarations, operator chains (`|>`), and `assert_contractive` assertions.
2. Pest / EBNF grammar: Defined for `ensemble`, `use`, and previously contained experimental general-purpose control flow tokens (`if`, `loop`, `fn`, `struct`).

Allowing general-purpose tokens in the kernel grammar compromised the integrity of the formal mathematical substrate.

## Decision
1. **Tree-Sitter as Kernel Authority**: `tree-sitter-pirtm` remains the sole, uncompromised syntactic authority for mathematical governance tokens (`tensor`, `|>`, `assert_contractive`, `p_N`, `\Lambda_m`).
2. **Quarantine of Control Flow**: General-purpose control-flow tokens (`if`, `loop`, `fn`, `struct`) are strictly quarantined from the production mathematical grammar.
3. **Packaging Grammar**: Pest/EBNF is restricted to package-level envelopes (`ensemble`, `use`, imports, spectral budgets).
4. **General-Purpose Application Grammar**: Application code is parsed by `pirtm-parser` and lowered through `pirtm-mlir` without altering the formal kernel syntax.

## Consequences
- The formal mathematical kernel remains pure and invariant.
- Control flow cannot be used to circumvent kernel contractivity assertions.
