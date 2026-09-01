# PIRTM Grounded Status & Claim Table

**Last Audited:** 2026-09-01  
**Audit SHA-256:** `a7f3b2c1d4e5f6...` (recompute after each edit)

This table reflects the ground-truth status of all PIRTM/MOC components, replacing aspirational statements with verifiable status indicators. Every "✅ Complete" claim must link to an existing, verifiable test or physical artifact on tree.

| Subsystem / Feature | Claimed Status | Verified On-Tree Status | Verifying Test / Artifact |
|---|---|---|---|
| **Lexer & Parser** | Production-Grade | ✅ Complete | `pirtm-parser/tests/test_json_parser.rs` (17/17 top-level constructs) |
| **MLIR Dialect & Lowering** | Production-Grade | ✅ Complete | `examples/json_parser.mlir` generated via `pirtm compile` |
| **Mutable State (`let mut`, `=`)** | Verified | ✅ Complete | `pirtm-mlir/src/pirtm/transpiler/visitor.rs` (Stack alloca/store/load) |
| **Method Calls & Postfix Chaining** | Verified | ✅ Complete | FFI built-in dispatch to `string_len`, `vec_push`, `map_insert` |
| **WardMonitor Runtime Drift** | Verified | ⚠️ Partial | `pirtm-monitor` unit tests (Zeno damping & kill-switch); **unit mismatch with Lean formalization** (ADR-025) |
| **Small-Gain Spectral Radius ($\rho < 1$)** | Formal Invariant | ✅ Complete | `pirtm-engine/tests/spectral_tests.rs` & CLI `--ensemble` validation |
| **Lean Axiom-Clean Core** | Mathlib-Free | ⚠️ Partial | `lean/` self-contained build; **kernel imports broken** (`prime_tensors` missing, ADR-018); **0 `sorry` in Proofs.lean** (ADR-019) |
| **Sedona Spine CI Gate** | Continuous Enforcement | ✅ Complete | `.github/workflows/sedona_spine_ci.yml` on-tree |
| **Bounded Iteration Theorems (Phase A)** | Formal Proofs | ⚠️ Partial | `lean/ADR/BoundedIteration.lean` (`iterate_non_expansive`, zero-sorry); **integer metric may not match f64 runtime** (ADR-025) |
| **MLIR Lowering Soundness (ADR-017)** | Formal Proofs | ⚠️ Partial | `lean/ADR/LoweringSoundness.lean` (`mlir_lowering_preserves_contractivity`); **kernel build broken** (ADR-018) |
| **End-to-End JSON Parser Execution** | Governed Runtime | ❌ Broken | `pirtm-engine/tests/json_parser_execution.rs` **simulates output**; no real LLVM IR execution (ADR-022) |
| **Governed HTTP/1.1 Micro-Server** | Network Application | ❌ Broken | `examples/http_server.pirtm`, `std/net.pirtm`; **runtime is simulated** (ADR-022) |
| **Grammar Quarantine (ADR-014)** | Kernel Purity | ❌ Broken | Single lexer mixes kernel and control-flow tokens (ADR-023) |
| **Admissibility Validator** | Governance Gate | ❌ Broken | `AdmissibilityValidator::validate` is a no-op (ADR-021) |
| **Toolchain Lock** | Zero Drift | ❌ Broken | `fixedToolchain: false` in `lake-manifest.json` (ADR-024) |

## Legend

- ✅ Complete — physically on-tree, tested, no open ADR defects.
- ⚠️ Partial — on-tree but has open defects documented in ADR-018 through ADR-030.
- ❌ Broken — claims complete status but has critical defects or is simulated.

## Audit Protocol

No claim may be marked "✅ Complete" unless:
1. The code physically exists on the current tree.
2. Tests pass without `sorry`, mocks, or unverified heuristics.
3. No open ADR (018–030) identifies a blocking defect for that claim.
