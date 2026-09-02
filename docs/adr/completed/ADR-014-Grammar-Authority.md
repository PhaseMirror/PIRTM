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

---

## Addendum (2026-09-02): Two-Stage Spectral Interlock Progression

To maintain zero-drift alignment between compiler IPC daemons (`pirtmd`), client editors, and the L0 kernel grammar authority:

1. **Phase 1 Interlock (`main` @ `9e593ee`)**:
   - `pirtmd` parses application source via `pirtm_parser` and walks AST `Stmt::Let` / `Stmt::LetMut` nodes to extract rational tuples for `matrix` and `lambdas`.
   - String scanning, comment-scraping, and canned $2\times 2$ fallback matrices are strictly prohibited.
   - Exact rational small-gain evaluation operates exclusively via `Ensemble::from_rationals` over reduced `PosRat` in $\mathbb{Q}$.

2. **Phase 2 Packaging Grammar Production (Roadmap for v2.0.0)**:
   - First-class packaging keywords (`matrix`, `lambdas`, `theorem`) belong strictly to the **Pest Packaging Grammar** alongside `ensemble` and `use`. They define module-level gain boundaries and Lean theorem anchor links.
   - The **Tree-Sitter Kernel Authority** remains strictly reserved for pure mathematical L0 operations (`tensor`, `|>`, `assert_contractive`, `p_N`, `\Lambda_m`). This eliminates dual-root syntax ambiguity.

3. **Decoupling from ADR-055 Sunset**:
   - The ADR-055 hard sunset (`2026-10-01` / `v1.0.1-mvp`) strictly mandates the total deletion of `Ensemble::new(f64)` in favor of exact rational constructors in $\mathbb{Q}$.
   - Phase 1 `pirtmd` Let-binding extraction operates strictly over `PosRat` in $\mathbb{Q}$ and remains fully valid through and beyond the `2026-10-01` constructor sunset. Phase 2 grammar productions are not a blocker for the ADR-055 `f64` deletion.

4. **Execution Authority vs Reference Specification**:
   - `pirtm-parser` (`pirtm-app-lexer` + hand-written Pratt `Parser` in `pirtm-parser/src/lib.rs`) is the active production execution authority for application AST construction and MLIR lowering in Phase 1.
   - `pirtm.pest` is the formal reference specification for packaging envelope rules and is permitted to remain non-executing until the v2.0.0 parser unification milestone.
