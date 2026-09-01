# ADR-030: Reconcile README Claim Table with Axiom Ledger

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Documentation
- **Date**: 2026-09-01

## Context

`docs/PIRTM-README-Claim-Table.md` asserts:

| Subsystem / Feature | Claimed Status | Verified On-Tree Status |
|---|---|---|
| **Small-Gain Spectral Radius ($\rho < 1$)** | Formal Invariant | ✅ Complete |
| **Lean Axiom-Clean Core** | Mathlib-Free | ✅ Complete |
| **Sedona Spine CI Gate** | Continuous Enforcement | ✅ Complete |
| **Bounded Iteration Theorems (Phase A)** | Formal Proofs | ✅ Complete |
| **MLIR Lowering Soundness (ADR-017)** | Formal Proofs | ✅ Complete |

However:
- `PIRTM.lean` is broken (missing `prime_tensors` imports — ADR-018).
- `Proofs.lean` contains six `sorry` statements (ADR-019).
- `Export.lean` references undefined `adr0999` (ADR-020).
- `AdmissibilityValidator` is a no-op (ADR-021).
- `Runtime::run` is simulated (ADR-022).

The README claim table is aspirational, not grounded.

## Decision

1. **Audit every claim** in `PIRTM-README-Claim-Table.md` against physical on-tree evidence.
2. **Downgrade claims** from "✅ Complete" to "⏳ In Progress" where the corresponding ADR (018–029) identifies a defect.
3. **Add a "Last Audited" timestamp** and a SHA-256 hash of the claim table to prevent silent re-aspiration.
4. **Require CI to verify** that every "✅ Complete" claim links to a passing test or build artifact.

## Consequences

- The claim table becomes a ground-truth document per ADR-015.
- Future "README pattern" drift is detected automatically.
- Stakeholders can trust the status matrix as a fidelity indicator.
