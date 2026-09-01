# ADR-020: Fix Export.lean Undefined ADR Reference

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

`lean/ADR/Export.lean` references `adr0999` in both `exportToDocs` and `printAll`, but `adr0999` is never defined in `ADR/Examples.lean`. Running `lake exe export` will fail with an unknown identifier error.

## Decision

1. **Define `adr0999`** in `ADR/Examples.lean` as a `Deprecated` ADR (or remove the reference from `Export.lean`).
2. **Add `exportToDocs` and `printAll` to the test harness** in `ADR/Test.lean` so undefined references are caught at test time, not just at runtime.

## Consequences

- `lake exe export` produces the expected markdown artifacts.
- The export path is covered by `lake test`.
- Future missing ADR references are caught by the type checker / test suite.
