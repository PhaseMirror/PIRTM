# PIRTM Grounded Status & Claim Table

**Last Audited:** 2026-09-01  
**Audit SHA-256:** `39b37b7d6afaa96b7c807a95b83bdfbfe1dec87fc069ff7e1eccdf1d7f2134b8`

This table reflects the ground-truth status of all PIRTM/MOC components, replacing aspirational statements with verifiable status indicators. Every "✅ Complete" claim must link to an existing, verifiable test or physical artifact on tree.

| Subsystem / Feature | Claimed Status | Verified On-Tree Status | Verifying Test / Artifact |
|---|---|---|---|
| **Lexer & Parser** | Production-Grade | ✅ Complete | `pirtm-parser/tests/test_json_parser.rs` (17/17 top-level constructs) |
| **MLIR Dialect & Lowering** | Production-Grade | ✅ Complete | `examples/json_parser.mlir` generated via `pirtm compile` |
| **Mutable State (`let mut`, `=`)** | Verified | ✅ Complete | `pirtm-mlir/src/pirtm/transpiler/visitor.rs` (Stack alloca/store/load) |
| **Method Calls & Postfix Chaining** | Verified | ✅ Complete | FFI built-in dispatch to `string_len`, `vec_push`, `map_insert` |
| **WardMonitor Runtime Drift** | Verified | ✅ Complete | `pirtm-monitor` unit tests (Zeno damping & kill-switch); unit consistency with Lean formalization documented (ADR-025) |
| **Small-Gain Spectral Radius ($\rho < 1$)** | Formal Invariant | ✅ Complete | `pirtm-engine/tests/spectral_tests.rs` & CLI `--ensemble` validation |
| **Lean Axiom-Clean Core** | Mathlib-Free | ✅ Complete | `lean/` self-contained build; kernel imports repaired (ADR-018); `AdmissibilityValidator` enforced (ADR-021) |
| **Sedona Spine CI Gate** | Continuous Enforcement | ✅ Complete | `.github/workflows/sedona_spine_ci.yml` on-tree |
| **Bounded Iteration Theorems (Phase A)** | Formal Proofs | ✅ Complete | `lean/ADR/BoundedIteration.lean` (`iterate_non_expansive`, zero-sorry); `ZenoController.lean` proves threshold ordering (ADR-025) |
| **MLIR Lowering Soundness (ADR-017)** | Formal Proofs | ✅ Complete | `lean/ADR/LoweringSoundness.lean` (`mlir_lowering_preserves_contractivity`); kernel build verified (ADR-018) |
| **End-to-End JSON Parser Execution** | Governed Runtime | ⏳ In Progress | `pirtm-engine/tests/json_parser_execution.rs` real execution implemented; requires LLVM toolchain in CI |
| **Governed HTTP/1.1 Micro-Server** | Network Application | ⏳ In Progress | `examples/http_server.pirtm`, `std/net.pirtm`; real execution implemented; requires LLVM toolchain in CI |
| **Grammar Quarantine (ADR-014)** | Kernel Purity | ✅ Complete | Separate `pirtm-kernel-lexer` and `pirtm-app-lexer` strictly enforce isolation (ADR-023) |
| **Admissibility Validator** | Governance Gate | ✅ Complete | `AdmissibilityValidator::validate` rejects float literals, unbounded loops, and uncertified primes (ADR-021) |
| **Toolchain Lock** | Zero Drift | ✅ Complete | `fixedToolchain: true` in `lake-manifest.json`; CI verifies `lean --version` matches `lean-toolchain` (ADR-024) |
| **ADR Governance System (Rust/Kani)** | Dual-Verified Governance | ✅ Complete | `cargo test -p adr_rust` (4 tests: immutability, cycle detection, history, examples) |
| **ADR Registry Verifier & Kernel Boundary** | Zero-Drift Gate | ✅ Complete | `cargo test -p adr-verifier` (8 tests: structural + boundary guard) |
| **Governed Ensemble Registry** | Contraction + Resonance Gate | ✅ Complete | `cargo test -p pirtm-registry` (2 tests: publish accept/reject) |
| **Distribution Ledger** | Invariant-Gated Install | ✅ Complete | `cargo test -p pirtm-dist` (2 tests: install valid/unlawful) |
| **CRMF Orchestration** | Hardware-Tensor Binding | ✅ Complete | `cargo test -p pirtm-orchestration` (1 test: CRMF request) |
| **REPL + LSP Developer Tools** | Real-Time Gate Feedback | ✅ Complete | `cargo build -p pirtm-tools`; LSP diagnostics on `scf`/`func` ops |
| **WebAssembly SDK Builder** | WASM Compilation Bridge | ⏳ In Progress | `pirtm-web-sdk` binary compiles via `cargo build`; requires `emcc`+`lake` for `build` subcommand |
| **QMHES Stability Theorems (ADR-033)** | Formal Proofs | ⚠️ Partial | `lean/ADR/QMHESStability.lean` (5 theorems + 6 supporting lemmas, zero-sorry); `lake build` + `lake test` on-tree; open deferred property `AX-QMHES-003` and scoped KDF assumption `AX-QMHES-001` |

## Legend

- ✅ Complete — physically on-tree, tested, no open ADR defects.
- ⚠️ Partial — on-tree but has open defects documented in ADR-018 through ADR-030.
- ⏳ In Progress — implementation exists but requires additional infrastructure (e.g., LLVM toolchain in CI).
- ❌ Broken — claims complete status but has critical defects or is simulated.

## Audit Protocol

No claim may be marked "✅ Complete" unless:
1. The code physically exists on the current tree.
2. Tests pass without `sorry`, mocks, or unverified heuristics.
3. No open ADR (018–030) identifies a blocking defect for that claim.
