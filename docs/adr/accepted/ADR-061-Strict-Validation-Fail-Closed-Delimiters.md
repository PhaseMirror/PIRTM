# ADR-061: Strict Validation & Fail-Closed Errors for Missing Delimiters

- **Status**: Accepted
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

To eliminate ambiguity between header-only packaging files and full application code contracts, validation must enforce clear fail-closed rules when delimiters or envelope headers are missing or malformed.

## Decision

1. **Validation Invariants**:
   - **Header + Body File**: If a file contains non-packaging application code (e.g. `fn main()`), the standalone `---` header delimiter **must** be present following the envelope declarations or split at item boundary.
   - **Header-Only Envelope File**: If `---` is absent, the file MUST consist strictly of envelope declarations (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`). The presence of `fn` or `struct` in a header-only file without `---` triggers `MissingHeaderDelimiter` error.
   - **Malformed Header**: If `---` is present but `matrix` or `lambdas` declarations are missing, or if unexpected `let foo = ...` statements appear in the header, compilation fails closed with `InvalidHeaderStatement` or `MissingSpectralParams`.
   - **Multiple Delimiters**: If multiple `---` lines appear in the file, compilation fails closed with `MultipleHeaderDelimiters`.

2. **Error Taxonomy**:
   - `MultipleHeaderDelimiters`: Multiple `---` header delimiters detected in document.
   - `InvalidHeaderStatement`: Unexpected non-packaging statement in envelope header.
   - `MissingHeaderDelimiter`: Non-packaging body detected without `---` header delimiter.
   - `MissingSpectralParams`: Missing or invalid rational matrix/lambdas declarations in envelope header.
   - `MissingTheoremAnchor`: Missing theorem anchor or forbidden `author_declared_lambda` parameter per ADR-055.

## Consequences

- Explicit error messages for end-users and client IDE extensions.
- Eliminates non-deterministic or ambiguous parsing behavior.
