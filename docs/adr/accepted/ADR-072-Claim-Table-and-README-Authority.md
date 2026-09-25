# ADR-072: Claim Table and README Authority

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the self-pinned hash and status reconciliation claims in ADR-030 where current-tree evidence conflicts

## Context

`docs/PIRTM-README-Claim-Table.md` embeds a SHA-256 of its own contents, which changes whenever the hash field changes. Its current embedded digest does not match the file. The table contains duplicate ADR-049 and ADR-suite rows, stale paths into deprecated `lean/ADR/`, and Complete claims contradicted by executable `sorry`, CI omissions, and the Poseidon2 defect. `README.md` independently asserts Complete/Verified status for several of those same subsystems.

The hidden assumption is that a self-referential hash and a manually duplicated README can remain synchronized by convention. They cannot.

## Decision

1. `docs/PIRTM-README-Claim-Table.md` is the canonical status matrix. `README.md` may summarize it but must not independently assign Complete or Verified status.
2. The claim-table digest is stored in `PIRTM-README-Claim-Table.md.sha256`; the table references the sidecar instead of embedding a self-referential digest.
3. Every Complete row names a physical artifact and a passing command. Every open defect names an ADR or AX/ENF identifier and uses Partial, In Progress, Defect, or Broken.
4. Duplicate rows are removed. Deprecated paths are replaced with canonical `lean/Foundations/ADR/` paths.
5. The table is synchronized to `artifacts/` and its sidecar is verified by CI.
6. README status badges are limited to the canonical table link and must not claim zero-sorry, full workspace verification, or ZK soundness unless their rows support those claims.

## Consequences

- Status is attributable and reproducible rather than aspirational.
- A changed claim automatically invalidates the sidecar until the audit is rerun.
- README and claim table cannot silently diverge.
- The current audit rebaseline explicitly demotes unsupported claims instead of preserving historical Complete markers.

## Validation

- `sha256sum -c docs/PIRTM-README-Claim-Table.md.sha256` passes.
- `cmp docs/PIRTM-README-Claim-Table.md artifacts/PIRTM-README-Claim-Table.md` passes.
- No Complete row references a missing file, deprecated path, or open AX/ENF defect.
- README contains no independent Complete/Verified subsystem assertions.
