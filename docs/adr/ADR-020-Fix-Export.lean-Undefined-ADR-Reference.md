# ADR-020: Fix Export.lean Undefined ADR Reference

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`adr0999` is defined** in `lean/ADR/Examples.lean` as a `Deprecated` prototype ADR.
2. **`Export.lean` references are valid** — both `exportToDocs` and `printAll` successfully reference `adr0999`.
3. **Test coverage added** — `Test.lean` includes `adr0999_cannot_become_accepted` theorem and `adrRegistry` maps `⟨999⟩` to `adr0999`.

## Validation

```bash
$ lake build ADR.Export
Build completed successfully.
$ lake build ADR.Test
Build completed successfully.
```

## Context

`lean/ADR/Export.lean` references `adr0999` in both `exportToDocs` and `printAll`, but `adr0999` is never defined in `ADR/Examples.lean`. Running `lake exe export` will fail with an unknown identifier error.

## Decision

1. **Define `adr0999`** in `ADR/Examples.lean` as a `Deprecated` ADR (or remove the reference from `Export.lean`).
2. **Add `exportToDocs` and `printAll` to the test harness** in `ADR/Test.lean` so undefined references are caught at test time, not just at runtime.

## Consequences

- `lake exe export` produces the expected markdown artifacts.
- The export path is covered by `lake test`.
- Future missing ADR references are caught by the type checker / test suite.
