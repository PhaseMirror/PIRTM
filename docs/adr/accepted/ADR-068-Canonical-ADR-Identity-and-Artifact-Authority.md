# ADR-068: Canonical ADR Identity, Lifecycle, and Artifact Authority

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: ADR-030, ADR-044, and the implicit duplicate-directory convention

## Context

The repository currently has multiple authorities for the same ADR identity:

- `docs/adr/accepted`, `docs/adr/completed`, `docs/adr/proposed`, and `docs/adr/generated` all contain ADR documents.
- ADR-050 through ADR-057 are duplicated across lifecycle directories, while ADR-056 has two Proposed copies.
- `docs/adr/registry.json` stops at ADR-054 even though Lean modules, tests, and documents exist through ADR-067.
- `lean/Foundations/ADR/Examples.lean` and `Export.lean` stop at ADR-054; `Export.lean:46` also emits the non-canonical filename `ADR-0047-...`.
- `artifacts/` is not a mirror: it contains a stale claim table, a different ADR-052/053/054 set, and only a subset of the current ADR corpus.
- The same ADR number is reused for unrelated documents in `docs/` and `artifacts/`.

## Hidden Assumption

A directory name or historical export is treated as proof that an ADR is canonical. This is false: an ADR identity is canonical only when one document, one registry record, one formal record, and one release artifact agree.

## Decision

1. `docs/adr/accepted/ADR-NNN-*.md` is the canonical source for Accepted ADRs; `docs/adr/proposed/ADR-NNN-*.md` is the canonical source for Proposed ADRs. `completed/` and `generated/` are compatibility views and must not introduce a second identity.
2. ADR IDs are globally unique. A duplicate ID with different content is a governance defect and blocks acceptance.
3. `docs/adr/registry.json`, `lean/Foundations/ADR/Examples.lean`, and `lean/Foundations/ADR/Export.lean` must cover the same canonical ADR ID set. Missing records are Partial, not Complete.
4. Exported filenames use the canonical `ADR-NNN-` form; ADR-047 is `ADR-047-...`, not `ADR-0047-...`.
5. `artifacts/` is a byte-identical release mirror of the selected canonical governance set. A deterministic synchronization script and CI comparison are the enforcement mechanism.
6. ADR-065 is not Accepted until its document, registry record, Lean/Rust evidence, and tests exist on the current tree. Implemented code without an ADR is an enforcement gap, not an Accepted decision.

## Consequences

- Historical duplicates become auditable compatibility views instead of competing ground truth.
- Registry, Lean, Rust, documentation, and release artifacts can be checked as one identity set.
- Claims referencing missing or stale ADR records must be marked Partial or Broken until reconciliation completes.

## Validation

- No duplicate canonical ADR IDs with differing content.
- `registry.json`, `Examples.lean`, and `Export.lean` expose the same canonical ID set.
- `cmp docs/PIRTM-README-Claim-Table.md artifacts/PIRTM-README-Claim-Table.md` succeeds.
- The same comparison succeeds for the axiom ledger and every mirrored ADR.
