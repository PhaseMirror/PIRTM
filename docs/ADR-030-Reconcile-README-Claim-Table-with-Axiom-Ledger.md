# ADR-030: Reconcile README Claim Table with Axiom Ledger

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Documentation
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **Audited all claims** in `docs/PIRTM-README-Claim-Table.md` against physical on-tree evidence:
   - Verified test files exist and are referenced correctly.
   - Confirmed open ADR defects are reflected with appropriate status markers (⚠️ Partial, ⏳ In Progress).
   - Downgraded stale "✅ Complete" claims where blocking defects remain (e.g., ADR-022, ADR-025).
2. **Synchronized `artifacts/PIRTM-README-Claim-Table.md`** with the canonical `docs/` version to eliminate divergence.
3. **Pinned audit hash** — Replaced placeholder SHA-256 with actual digest of the claim table:
   `81c67eefda4ba2e985a5c6f674f261d0cd4f75886923b5a980809a29c93a753d`
4. **Legend expanded** — Added explicit `⏳ In Progress` marker for implementations that exist but require additional CI infrastructure.

## Validation

```bash
$ sha256sum docs/PIRTM-README-Claim-Table.md
81c67eefda4ba2e985a5c6f674f261d0cd4f75886923b5a980809a29c93a753d  docs/PIRTM-README-Claim-Table.md
```

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
