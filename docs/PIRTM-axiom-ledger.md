# PIRTM Formal Axiom & Enforcement Ledger

This ledger records all outstanding proof obligations, unmirrored enforcement mechanisms, heuristic mocks, and unproved claims across the codebase.

## 1. Unmirrored Enforcement & CI Gates

| Identifier | Policy / Gate | Documented Status | Actual Physical Status | Remediation Plan / Verification |
|---|---|---|---|---|
| **ENF-001** | Sedona Spine CI (`sedona_spine_ci.yml`) | "✅ Complete" in ADR-001 | `.github/workflows/sedona_spine_ci.yml` added on-tree | Gate active in CI pipeline |
| **ENF-002** | Matrix Spectral Radius Gate ($\rho < 1.0$) | "Active" in Linker Docs | ✅ Resolved on-tree | Implemented in `pirtm-engine::spectral` & tested in `tests/spectral_tests.rs` |
| **ENF-003** | Zero-Mathlib Enforcement | "Axiom-Clean Core" | Verified self-contained in `lean/` | Checked by CI Gate 1 & `lake build` (7/7 targets) |
| **ENF-004** | Kani Bounded Verification | `adr_rust` Kani harnesses `#[cfg(kani)]` only | Standard `cargo test` does not exercise them | Document `cargo kani` as optional verification gate in CI |
| **ENF-005** | `pirtm-ui` Mock Executor | `KubeExecutor` uses `MOCK:` prefixed stubs | Production deploy/revoke path not real | Replace with `kube-rs` real client or delete crate (see AX-004) |
| **ENF-006** | Prime-arithmetic attribution (ADR-035) | Lean `isPrimeBasis` in `lean/Foundations/ADR/PrimeQuantum.lean` is a *structural* snapshot, not the arithmetic authority | Exact integer primality is verified in Rust/Kani (`rust/adr_rust/src/prime_quantum.rs`, trial-division `is_prime_basis` + `#[kani::proof] firm_ad035_prime_syndrome_invariants`) | Lean syndrome theorems stay sound (relative to predicate); arithmetic ground truth lives in Rust/Kani mirror; regression `test_composite_integer_rejected_as_nonprime` guards 49=7·7 |

## 2. Proof Debts & Mock Closures

| Identifier | Module / Proof | Defect Description | Impact | Target Closure |
|---|---|---|---|---|
| **AX-001** | `sin_lipschitz` | Proof reduces to identity mapping without bound derivation | Local Lipschitz claim unproved | Replace with verified Taylor bound |
| **AX-002** | `TypeChecker.lean` | Exhaustive cases terminated with `exact rfl` on ungrounded terms | False type safety proof | Rigorous inductive verification |
| **AX-003** | `linear_map_is_contractive` | Hides unstated axiom inside proof body using undefined `dist` | Contractivity unproven | Port to standard metric space definition |
| **AX-004** | `pirtm-ui/src/main.rs` | `KubeExecutor` uses `MOCK:` prefixed stubs for Kubernetes deploy/revoke | Production deployment path not real | Replace with `kube-rs` real client or delete crate |
| **AX-005** | `adr_rust/src/euclidean/arithmetic.rs` | `classify` returns `Number` for all non-prime, non-composite integers > 1, but no integer > 1 is ever classified as `Number` | The `IntegerClass::Number` variant is dead code | Audit or remove `Number` variant in future refactor |
| **AX-QMHES-001** | `lean/ADR/QMHESStability.lean` (HKDF block security, E.2) | Cryptographic PRF indistinguishability of the underlying KDF is a layered assumption (NIST/liboqs); only block well-definedness is machine-checked | Adversary-level indistinguishability not proven inside Lean | Recorded ADR-033 non-goal; close via external NIST CAVP evidence + liboqs linkage |
| **AX-QMHES-003** | `lean/ADR/QMHESStability.lean` (frequency map, F.4) | Strong Lipschitz form `|φ(ω₁)−φ(ω₂)| ≤ |ω₁−ω₂|` for φ(ω)=ω/1000 deferred; only quantization-boundedness (`φ(ω) ≤ ω`) is proven | Full noise-amplification bound for key-space lattice mapping unproved | Prove via Euclidean-division remainder case analysis (`Nat.le_div_iff_mul_le` scaffolding ready) |
| **AX-PQ-001** | `lean/Foundations/ADR/PrimeQuantum.lean` (`isPrimeBasis` structural snapshot) | The Lean prime indicator is a simplified decidable snapshot; it may misclassify composites such as 49 as prime | Lean syndrome theorems are sound only *relative to* the predicate, not relative to true integer primality | Arithmetic primality is enforced in the Rust/Kani mirror (`adr_rust::prime_quantum`); close by proving exact primality in Lean or linking the Lean predicate to the Kani-verified Rust oracle |
| **AX-057** | `rust/pirtm-engine/src/spectral.rs` `Ensemble::new` + `validate_and_certify` (commit 0092f80e) | Commit 0092f80e introduced unresolved merge conflict markers (`<<<<<<< HEAD / ======= / >>>>>>> 5318951`). Incoming branch reintroduced `10^6` fixed-point float scaler for adjacency matrix entries, conflicting with Step 5 precision goal ($\mathbb{Q}$ exact rationals). MCP name default `unwrap_or("author_declared_lambda")` also re-introduced Step 4c escape hatch. Invalid ADR-056 document (chat log, not decision) was added. | Merge conflict markers are not valid Rust. CI gate `cargo test --workspace` cannot pass. Incorrect quantization defeats precision architecture. | Reverted commit 0092f80e (commit 8bc3b4b); awaiting clean refactor proposal with ADR authorization |

## 2b. Proof Debts Resolved During ADR-034…043 Audit

The following `sorry` obligations were eliminated (zero-sorry Lean build, `lake build` = 38 jobs green; `grep sorry lean/` empty except docstrings):

| Module / Proof | Prior State | Resolution |
|---|---|---|
| `Foundations/ADR/Proofs.lean` `accepted_immutable` | Unprovable statement (`a` accepted ⇒ no other accepted ADR) + `sorry` | Replaced with honest, provable `accepted_immutable_without_supersession` (mirrors canonical `lean/ADR/`) |
| `Foundations/ADR/Proofs.lean` `no_circular_supersession` | Unprovable statement (`supersedes ≠ some a.id`) + `sorry` | Replaced with fuel-bounded `followSupersession_length_bounded` (well-founded chain, no circular traversal) |
| `Foundations/ADR/DialecticalSemantics.lean` `admissible_implies_invariants` | `sorry` | Proved by `by_cases` on the three actual gate conditions (grounding `< minThreshold`, robustness `>= 100`, dialectical `> maxAllowed` / `branchCount <= 1`) |

## 3. Claim Reconciliation Protocol
No claim in `README.md` or documentation may be designated "Production-Ready" or "Complete" unless:
1. The code physically exists on-tree.
2. The verification tests pass without `sorry`, mocks, or unverified author-declared floats.
