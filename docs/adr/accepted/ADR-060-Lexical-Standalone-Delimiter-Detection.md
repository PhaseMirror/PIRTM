# ADR-060: Lexical Standalone Delimiter Detection

- **Status**: Accepted
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

The header delimiter `---` must be recognized lexically as a standalone token rather than matched via naive string search, to prevent false positives when `---` appears inside string literals (`"---"`), comments (`// ---`), or identifiers.

## Decision

1. **Standalone Lexical Scanning**:
   - `#[token("---")] HeaderDelim` in `pirtm-app-lexer` (Logos) tokenizes `---` as a first-class delimiter token before unary minus.
   - `find_header_split_offset(source) -> Option<(usize, bool)>` scans line boundaries in raw source outside block comments `/* ... */`.
2. **Deterministic Split Index**:
   - The split index returned by `find_header_split_offset` uniquely partitions source text into envelope prefix `source[..idx]` and application body suffix `source[idx + 3..]`.

## Consequences

- Prevents false-positive splitting when `---` appears inside comments or raw string literals.
- Deterministic, unambiguous boundary detection across all compiler tools.
