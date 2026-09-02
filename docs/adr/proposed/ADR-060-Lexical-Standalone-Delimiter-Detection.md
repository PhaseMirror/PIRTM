# ADR-060: Lexical Standalone Delimiter Detection

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

The header delimiter `---` must be recognized lexically as a standalone token rather than matched via naive string search, to prevent false positives when `---` appears inside string literals (`"---"`), comments (`// ---`), or identifiers.

## Decision

1. **Standalone Lexical Scanning**:
   - `find_header_delimiter(source) -> Option<usize>` scans line boundaries in raw source.
   - A line is identified as the header delimiter if and only if:
     1. It consists of `---` (optionally surrounded by whitespace).
     2. It is not contained within a multi-line string literal or block comment `/* ... */`.
2. **Deterministic Split Index**:
   - The split index returned by `find_header_delimiter` uniquely partitions source text into envelope prefix `source[..idx]` and application body suffix `source[idx + 3..]`.

## Consequences

- Prevents false-positive splitting when `---` appears inside comments or raw string literals.
- Deterministic, unambiguous boundary detection across all compiler tools.
