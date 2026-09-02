# ADR-061: Strict Validation & Fail-Closed Errors for Missing Delimiters

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

To eliminate ambiguity between header-only packaging files and full application code contracts, validation must enforce clear fail-closed rules when delimiters or envelope headers are missing or malformed.

## Decision

1. **Validation Invariants**:
   - **Header + Body File**: If a file contains non-packaging application code (e.g. `fn main()`), the standalone `---` header delimiter **must** be present following the envelope declarations.
   - **Header-Only Envelope File**: If `---` is absent, the file MUST consist strictly of envelope declarations (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`). The presence of `fn` or `struct` in a header-only file without `---` triggers `MissingHeaderDelimiter` error.
   - **Malformed Header**: If `---` is present but `matrix` or `lambdas` declarations are missing or invalid, compilation fails closed with `MissingSpectralParams`.

2. **Error Taxonomy**:
   - `MissingHeaderDelimiter`: Non-packaging body detected without `---` header delimiter.
   - `MissingSpectralParams`: Missing or invalid rational matrix/lambdas declarations in envelope header.
   - `MissingTheoremAnchor`: Missing theorem anchor or forbidden `author_declared_lambda` parameter per ADR-055.

## Consequences

- Explicit error messages for end-users and client IDE extensions.
- Eliminates non-deterministic or ambiguous parsing behavior.
