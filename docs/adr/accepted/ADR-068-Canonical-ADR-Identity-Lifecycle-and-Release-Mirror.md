# ADR-068: Canonical ADR Identity, Lifecycle, and Release Mirror

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: ADR-030 and ADR-044 where they permit duplicate or detached registries

## Context

The current tree contains multiple authorities for the same ADR identity:

- ADR-052, ADR-053, and ADR-054 have conflicting copies in `docs/` and `artifacts/`.
- ADR-056 and ADR-057 through ADR-061 exist in both `docs/adr/completed/` and lifecycle directories.
- `docs/adr/registry.json`, `lean/Foundations/ADR/Examples.lean`, and `lean/Foundations/ADR/Export.lean` stop at ADR-054 while Lean modules, tests, and source code continue through ADR-067.
- `docs/adr/generated/` is treated as both derived output and a source of truth.

The hidden assumption is that directory placement or a matching numeric prefix establishes identity. It does not. Identity requires one canonical document, one lifecycle state, and one machine-checkable registry entry.

## Decision

1. `docs/adr/proposed/`, `docs/adr/accepted/`, and `docs/adr/deprecated/` are the canonical lifecycle directories. `docs/adr/completed/` and `docs/adr/generated/` are legacy or derived views and cannot override a canonical file.
2. An ADR number identifies exactly one decision. Duplicate IDs with different titles are invalid and fail the ADR verifier.
3. `docs/adr/registry.json`, `lean/Foundations/ADR/Examples.lean`, and `lean/Foundations/ADR/Export.lean` must describe the same canonical ADR set. A missing entry is an enforcement gap, not an implicit acceptance.
4. `artifacts/` is a byte-identical release snapshot of canonical accepted ADRs, the claim table, the axiom ledger, and their checksums. It is not an independent editorial branch.
5. Code existence does not promote an ADR. ADR-064 and ADR-067 remain Proposed until their proof, test, and artifact obligations are closed.
6. The current canonical identities are the `docs/adr/accepted/` copies of ADR-052, ADR-053, and ADR-054. Conflicting legacy copies are superseded by this decision.

## Consequences

- Historical duplicates become detectable drift instead of alternate authorities.
- Registry parity can be enforced by CI.
- Release artifacts can be reproduced from the canonical tree.
- Existing historical documents remain readable, but cannot be cited as ground truth without a canonical link.

## Validation

- ADR verifier rejects duplicate IDs and missing required accepted entries.
- `cmp` succeeds for every canonical accepted ADR copied into `artifacts/`.
- Registry, Lean examples, and export lists contain the same canonical ID set.
