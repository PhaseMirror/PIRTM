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

5. **Envelope-Only Pest Quarantine**:
   - `pirtm.pest` is strictly envelope-only (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`). Legacy control flow tokens (`if`, `loop`, `fn`, `struct`) are completely removed from Pest.
   - Application control flow is parsed exclusively by `pirtm-parser` (Pratt parser) and lowered to MLIR, ensuring complete quarantine across all grammar layers.

6. **v2.0.0 Decoupled Two-Parser Execution Architecture**:
   - At v2.0.0 unification, `pirtmd` runs a **Decoupled Two-Parser Architecture**:
     1. **Packaging Envelope Parser (Pest)**: Parses module metadata header (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`) into formal envelope AST nodes.
     2. **Application Logic Parser (Pratt `pirtm-parser`)**: Parses application logic (`fn`, `struct`, `loop`, `match`) and lowers to MLIR.
   - `pirtmd` does NOT force a single concatenated Pest grammar containing application `fn` rules. The envelope spec and application parser remain decoupled layers.

7. **Envelope Boundary Delimiter & Theorem Anchor Grammar**:
   - The Pest envelope parser processes packaging statements (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`) until encountering an explicit `---` header delimiter (or EOI for header-only files). Implicit cuts on application keywords are strictly forbidden.
   - `theorem_decl` accepts formal string literals or qualified Lean theorem path identifiers (`theorem_anchor`).
   - Both `let matrix = ...;` and `matrix = ...;` syntax forms are recognized by `pirtm.pest` to ensure seamless transition between Phase 1 and Phase 2.

8. **Deterministic Header Delimiter Specification**:
   - For files containing both packaging envelopes and application code bodies, `---` is the mandatory deterministic header delimiter rule (`header_delimiter = { "---" }`).
   - Files lacking `---` are evaluated as header-only packaging envelope files (`header_only_file = { SOI ~ envelope_decls ~ EOI }`).
   - This eliminates all lexer ambiguity regarding identifiers or implicit keyword cuts.

9. **Phase 1 Daemon & System-Wide Extractor Freeze Mandate**:
   - `pirtmd` (`extract_spectral_params` + `split_header_body`) is the **sole authorized Phase 1 extraction entrypoint** across `pirtmd`, `pirtm-mcp`, `pirtm-compiler`, and Sentinel gates.
   - All tool entrypoints and background daemons MUST route contractivity verification through this canonical extractor.
   - No crate or daemon (including `pirtm-mcp` or language server tools) may implement an independent `let matrix` or header extractor path prior to v2.0.0 Pest unification.
