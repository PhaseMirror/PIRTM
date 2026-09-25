# ADR-068: Canonical Artifact Identity, Lifecycle, and Mirror

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: ADR-030 and the parity claim in ADR-044 where they conflict with this decision

## Context

The current tree contains multiple authority claims for the same ADR identity:

- ADR-050 through ADR-057 appear in more than one lifecycle directory.
- ADR-052, ADR-053, and ADR-054 have different documents under `docs/` and `artifacts/`.
- `docs/adr/registry.json`, `lean/Foundations/ADR/Examples.lean`, and `lean/Foundations/ADR/Export.lean` stop at ADR-054 while Lean modules and tests exist through ADR-064.
- `artifacts/PIRTM-README-Claim-Table.md` is stale relative to the canonical `docs/` table.
- The generated directory is treated both as source and as derived output.

The hidden assumption is that directory location or historical duplication establishes identity. It does not. ADR identity is a stable ID plus one canonical document and one canonical status.

## Decision

1. `docs/adr/accepted/` and `docs/adr/proposed/` are the canonical source locations for active ADR documents.
2. `docs/adr/completed/` is historical and has no authority when it conflicts with the canonical location. `docs/adr/generated/` is derived output and must not be edited by hand.
3. An ADR ID must identify exactly one canonical document. Duplicate IDs, conflicting titles, and conflicting statuses are governance failures and must fail the ADR verifier.
4. `docs/adr/registry.json`, `lean/Foundations/ADR/Examples.lean`, and `lean/Foundations/ADR/Export.lean` must cover the same active ADR ID set. A missing record is an open enforcement gap, not a complete claim.
5. `artifacts/` is a release mirror. It is generated from canonical documents and must be byte-identical for every mirrored file. The mirror includes the claim table, axiom ledger, their SHA-256 sidecars, and canonical ADR documents selected for release.
6. ADR status is evidence-scoped: `Accepted` requires passing on-tree verification; `Proposed` never satisfies a Complete claim; `Resolved` is reserved for a closed repair with its verification recorded.

## Consequences

- Historical duplicates remain available for audit but cannot be cited as ground truth.
- Registry, Lean examples, export output, docs, and artifacts become one reconcilable identity graph.
- New ADRs must update the registry and mirror in the same change.
- Existing claims that relied on duplicate or stale documents must be marked Partial or Defect until reconciled.

## Validation

- ADR verifier rejects duplicate IDs and missing required records.
- `cmp` succeeds for every mirrored file.
- `sha256sum -c` succeeds for claim-table and ledger sidecars.
- `lake build --rehash` and `lake test` pass after the active ADR set is registered.
