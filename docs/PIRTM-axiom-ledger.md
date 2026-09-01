# PIRTM Formal Axiom & Enforcement Ledger

This ledger records all outstanding proof obligations, unmirrored enforcement mechanisms, heuristic mocks, and unproved claims across the codebase.

## 1. Unmirrored Enforcement & CI Gates

| Identifier | Policy / Gate | Documented Status | Actual Physical Status | Remediation Plan / Verification |
|---|---|---|---|---|
| **ENF-001** | Sedona Spine CI (`sedona_spine_ci.yml`) | "✅ Complete" in ADR-001 | `.github/workflows/sedona_spine_ci.yml` added on-tree | Gate active in CI pipeline |
| **ENF-002** | Matrix Spectral Radius Gate ($\rho < 1.0$) | "Active" in Linker Docs | ✅ Resolved on-tree | Implemented in `pirtm-engine::spectral` & tested in `tests/spectral_tests.rs` |
| **ENF-003** | Zero-Mathlib Enforcement | "Axiom-Clean Core" | Verified self-contained in `lean/` | Checked by CI Gate 1 & `lake build` (7/7 targets) |

## 2. Proof Debts & Mock Closures

| Identifier | Module / Proof | Defect Description | Impact | Target Closure |
|---|---|---|---|---|
| **AX-001** | `sin_lipschitz` | Proof reduces to identity mapping without bound derivation | Local Lipschitz claim unproved | Replace with verified Taylor bound |
| **AX-002** | `TypeChecker.lean` | Exhaustive cases terminated with `exact rfl` on ungrounded terms | False type safety proof | Rigorous inductive verification |
| **AX-003** | `linear_map_is_contractive` | Hides unstated axiom inside proof body using undefined `dist` | Contractivity unproven | Port to standard metric space definition |

## 3. Claim Reconciliation Protocol
No claim in `README.md` or documentation may be designated "Production-Ready" or "Complete" unless:
1. The code physically exists on-tree.
2. The verification tests pass without `sorry`, mocks, or unverified author-declared floats.
