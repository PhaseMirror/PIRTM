# ADR-058: Formal Header Envelope Grammar & Scope Isolation

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

The envelope header must possess a rigorous, machine-checkable EBNF grammar in Pest that specifies exact envelope declarations (`ensemble`, `matrix`, `lambdas`, `theorem`, `use`) while strictly excluding application control flow tokens (`if`, `loop`, `fn`, `struct`).

## Decision

1. **Envelope-Only Grammar**:
   - `pirtm.pest` specifies `envelope_header = { SOI ~ (ensemble_decl | import_stmt | matrix_decl | lambdas_decl | theorem_decl)* ~ header_delimiter }`.
   - General-purpose application statements are prohibited from `pirtm.pest`.
2. **Exact Rational Value Grammar**:
   - `matrix_decl` and `lambdas_decl` match rational pairs `tuple_pair = { "(" ~ integer ~ "," ~ integer ~ ")" }` and nested row matrices `matrix_val = { "(" ~ tuple_row ~ ("," ~ tuple_row)* ~ ")" }`.
3. **Lean Path Anchor**:
   - `theorem_decl` matches string literals or qualified Lean paths `theorem_anchor = { string_lit | (ident ~ ("." ~ ident)*) }`.

## Consequences

- Machine-checked guarantee that envelope files conform strictly to the packaging specification.
- Erroneous or malformed envelope headers fail closed prior to spectral calculation.
