# PIRTM Grounded Status & Claim Table

**Last Audited:** 2026-09-02
**Audit SHA-256:** `4725b114c34a6eab3a85968efaf0b43c0d1e6d5fe752570b1fcb0ff2203da09b`
**Last code SHA referenced:** `bc258d499b9f7daa37cb829e1e1639680932e253`

This table reflects the ground-truth status of all PIRTM/MOC components, replacing aspirational statements with verifiable status indicators. Every "✅ Complete" claim must link to an existing, verifiable test or physical artifact on tree.

Step 2 (2026-09-02): two rows added or corrected for defects exposed by README demotion `8c15767` and Lean rename `bc258d49`. Existing data rows below the two Step-2 rows are unchanged in this commit.

| Subsystem / Feature | Claimed Status | Verified On-Tree Status | Verifying Test / Artifact |
|---|---|---|---|
| **Poseidon2 ZK Soundness (ADR-049)** | Formal Invariant | ❌ Defect | Lean identifiers renamed to `receipt_flag_conjunction` / `receipt_flag_conjunction_of_hyps` in `bc258d49`. Module records author-set Bool flags. Not a sponge, field, constraint system, or knowledge soundness. ADR-053 remains Proposed. |
| **ADR Suite 001–050 as a closed set** | 100% Verified | ⚠️ Partial | Markdown and Lean modules exist. Not a closed suite. README unclaimed this set on `8c15767`. Job-count claims in docs disagree. This row does not demote other Complete rows in this table. |
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
| **Dialectical Semantics (ADR-034)** | Gate Firewall | ✅ Complete | `lean/Foundations/ADR/DialecticalSemantics.lean` (`admissible_implies_invariants`, zero-sorry) + `rust/adr_rust/src/dialectical_semantics.rs` (Kani `verify_adr034_*`) + `lean/Foundations/ADR/Test.lean:47` & `Main.lean` runtime |
| **Prime-Encoded Quantum States (ADR-035)** | Subspace Error Detect | ✅ Complete | `lean/Foundations/ADR/PrimeQuantum.lean` (`prime_syndrome_positive/negative`, zero-sorry; arithmetic authority = Rust/Kani `is_prime_basis`) + `rust/adr_rust/src/prime_quantum.rs:84` (incl. `test_composite_integer_rejected_as_nonprime`) |
| **Prime-Structured TN-AE (ADR-036)** | Rank Surrogate Bound | ✅ Complete | `lean/Foundations/ADR/PrimeAutoencoder.lean` (`rank_surrogate_bounded`) + `rust/adr_rust/src/prime_autoencoder.rs` (Kani `verify_adr036_*`) + `lean/Foundations/ADR/Test.lean:78` |
| **Phase-Dissonance Functionals (ADR-037)** | Governance Layer | ✅ Complete | `lean/Foundations/ADR/PhaseDissonance.lean` (`in_bounds_implies_band_satisfied`) + `rust/adr_rust/src/phase_dissonance.rs` (Kani `verify_adr037_*`) + `lean/Foundations/ADR/Test.lean:85` |
| **Phase Mirror Governance Manifold (ADR-038)** | Fail-Closed Control | ✅ Complete | `lean/Foundations/ADR/GovernanceManifold.lean` (`cache_valid_implies_soft_envelope`) + `rust/adr_rust/src/governance_manifold.rs` (Kani `verify_adr038_*`) + `lean/Foundations/ADR/Test.lean:96` |
| **Cognitive Economy & Ethical Projection (ADR-039)** | Idempotent Projection | ✅ Complete | `lean/Foundations/ADR/CognitiveEconomy.lean` (`projection_idempotent`, `lawful_state_preservation`) + `rust/adr_rust/src/cognitive_economy.rs` (Kani `verify_adr039_*`) + `lean/Foundations/ADR/Test.lean:102` |
| **EchoBraid Quantum Feedback (ADR-040)** | Spectral Coherence | ✅ Complete | `lean/Foundations/ADR/EchoBraid.lean` (`prediction_bounded`) + `rust/adr_rust/src/echo_braid.rs` (Kani `verify_adr040_*`) + `lean/Foundations/ADR/Test.lean:112` |
| **Multiplicity Floer Operator (ADR-041)** | Flow Bound | ✅ Complete | `lean/Foundations/ADR/FloerOperator.lean` (`floer_flow_bounded`) + `rust/adr_rust/src/floer_operator.rs` (Kani `verify_adr041_*`) + `lean/Foundations/ADR/Test.lean:119` |
| **Constitutional Order & CSL (ADR-042)** | CSL Gate | ✅ Complete | `lean/Foundations/ADR/Constitution.lean` (`csl_gate_sound`) + `rust/adr_rust/src/constitution.rs` (Kani `verify_adr042_*`) + `lean/Foundations/ADR/Test.lean:127` |
| **Lawful Recursion License (ADR-043)** | Ξ-Certification | ✅ Complete | `lean/Foundations/ADR/License.lean` (`lawful_evolution_sound`) + `rust/adr_rust/src/license.rs` (Kani `verify_adr043_*`) + `rust/adr_rust/tests/integration_test.rs` (`test_lawful_evolution_soundness`) + `lean/Foundations/ADR/Test.lean:134` |
| **ADR Model Invariants (Proofs)** | Immutable / Supersession / Traceability | ✅ Complete | `lean/Foundations/ADR/Proofs.lean` (`accepted_immutable_without_supersession`, `followSupersession_length_bounded`, `traceability`) + `rust/adr_rust` integration test (`test_supersession_cycle_detection`); arithmetic/chain soundness delegated to Rust/Kani (`ENF-006` / `AX-PQ-001`) |
| **Registry Reconciliation (ADR-044)** | Promotion Rule | ✅ Complete | `lean/Foundations/ADR/Reconciliation.lean` (`promotion_requires_proofs`, zero-sorry) + `rust/adr_rust/src/reconciliation.rs` (Kani `verify_adr044_*` + `test_registry_reconciliation_promotion`) + `lean/Foundations/ADR/Test.lean:143` |

## Legend

- ✅ Complete — physically on-tree, tested, no open ADR defects.
- ⚠️ Partial — on-tree but has open defects documented in ADR-018 through ADR-030.
- ⏳ In Progress — implementation exists but requires additional infrastructure (e.g., LLVM toolchain in CI).
- ❌ Defect — named theorem or suite claim does not match on-tree predicate (ADR-053 pending for remaining tautologies).
- ❌ Broken — claims complete status but has critical defects or is simulated.

## Audit Protocol

No claim may be marked "✅ Complete" unless:
1. The code physically exists on the current tree.
2. Tests pass without `sorry`, mocks, or unverified heuristics.
3. No open ADR (018–030) identifies a blocking defect for that claim.

Step 2 does not rewrite historical Complete marks on rows other than the two Step-2 rows above.
