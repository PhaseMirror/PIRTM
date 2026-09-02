# ADR-057: Lexical Header Boundary Pre-Processor & Splitter

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

Phase 1 (Logos tokenization and AST Let-binding extraction) previously operated over the full file buffer. When a standalone `---` header delimiter appeared, operating over the full buffer created phase-ordering ambiguity between Phase 1 spectral param extraction and Phase 2 application body parsing.

## Decision

1. **Pre-Processing Header Splitter**:
   - Before statement parsing or AST construction begins, `pirtmd` and `PhaseMirrorCompiler` shall pass the raw source through a lexical pre-processor `split_header_body(source) -> (envelope_text, body_text)`.
   - The pre-processor locates the standalone `---` delimiter outside strings and comments.
   - If `---` is present, `envelope_text` contains all text prior to `---`, and `body_text` contains all text following `---`.
   - If `---` is absent, the entire source is evaluated as a header-only envelope or single-block application file according to validation rules.

2. **Phase 1 Isolation**:
   - Spectral param extraction (`extract_spectral_params`) shall operate exclusively over `envelope_text`.
   - Application AST parsing and MLIR lowering shall operate exclusively over `body_text`.

## Consequences

- Completely eliminates phase-ordering ambiguity between spectral extraction and body parsing.
- Ensures `---` is never passed to Logos or Pratt as statement syntax.
