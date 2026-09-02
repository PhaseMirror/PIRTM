# ADR-057: Lexical Header Boundary Pre-Processor & Splitter

- **Status**: Accepted
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

Phase 1 (Logos tokenization and AST Let-binding extraction) previously operated over the full file buffer. When a standalone `---` header delimiter appeared, operating over the full buffer created phase-ordering ambiguity between Phase 1 spectral param extraction and Phase 2 application body parsing.

## Decision

1. **Pre-Processing Header Splitter**:
   - Before statement parsing or AST construction begins, `pirtmd` passes the raw source through a lexical pre-processor `split_header_body(source) -> (envelope_text, body_text)`.
   - `find_header_split_offset(source)` locates the standalone `---` delimiter line (or implicit item boundary) outside strings and block comments `/* ... */`.
   - `envelope_text` contains all text prior to the split boundary, and `body_text` contains all text following the split boundary.

2. **Phase 1 Isolation**:
   - Spectral param extraction (`extract_spectral_params`) operates exclusively over `envelope_text`.
   - Application AST parsing and MLIR lowering operate exclusively over `body_text`.

## Consequences

- Completely eliminates phase-ordering ambiguity between spectral extraction and body parsing.
- Ensures `---` is never passed to Logos or Pratt as statement syntax.
