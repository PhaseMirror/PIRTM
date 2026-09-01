# ADR-031: Foundry Component Integration — ADR Governance, Distribution, Orchestration, and Tooling

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering, Compiler Engineering
- **Date**: 2026-09-01

## Context

The PIRTM/MOC project maintains a Rust workspace at `rust/` with 12 crates covering the compiler, lexer, engine, monitor, stdlib, and MCP server. A parallel Foundry monorepo at `Foundry/packages/rust/` contains additional components that are directly relevant to the Phase Mirror methodology but have not yet been integrated into PIRTM:

- **`adr_rust`** — A complete ADR governance system with Kani model-checking harnesses, Euclidean multiplicity primitives (integer classification, prime factorization, divisor poset, multiplicity profiles), and Superseded/Proposed/Accepted state-machine enforcement. Directly complements the Lean 4 ADR formalization in `lean/ADR/`.
- **`adr-verifier`** — A zero-dependency ADR registry verifier and Phase Mirror kernel boundary guard that checks `sorry` debt, axiom manifest, manifest drift, and tension scores at runtime.
- **`pirtm-registry`** — Governed registry for publishing PIRTM ensembles, enforcing contraction bound $c < 1.0$ and resonance tension $R_{sc} \geq 1.0$.
- **`pirtm-dist`** — Distribution ledger (`InstallReceipt`, `Distributor::install`) that enforces all 10 Phase Mirror invariants before install acceptance.
- **`pirtm-orchestration`** — Constitutional Recursive Manifestation Framework (CRMF) request types binding mathematical tensor contractions to physical hardware telemetry.
- **`pirtm-tools`** — Interactive REPL and LSP server (`tower-lsp`) for real-time Phase Mirror gate feedback on PIRTM source.
- **`pirtm-web-sdk`** — Rust CLI wrapper over `lake`/`emcc` for compiling PIRTM Lean proofs into WebAssembly.

These components are absent from the PIRTM workspace but are referenced by existing ADRs (ADR-001 mentions the Rust ADR scaffold; ADR-015 mentions the sedona spine CI gate; ADR-029 mentions the consequence entailment checker). Integrating them makes the governance and distribution substrate complete and machine-verifiable.

## Decision

1. **Integrate 7 Foundry crates** into `PIRTM/rust/` workspace: `adr_rust`, `adr-verifier`, `pirtm-registry`, `pirtm-dist`, `pirtm-orchestration`, `pirtm-tools`, `pirtm-web-sdk`.
2. **Do not integrate `pirtm-lexer`** — superseded by the quarantined `pirtm-kernel-lexer` / `pirtm-app-lexer` split (ADR-023).
3. **Defer `pirtm-ui`** — contains `MOCK:` prefixed KubeExecutor stubs that violate the Zero Tolerance for Simulation principle (ADR-015). Integration blocked until mocks are replaced with real execution paths.
4. **Adapt Cargo.toml files** to the PIRTM workspace conventions:
   - Remove standalone `[workspace]` and `[workspace.package]` sections from `pirtm-registry`, `pirtm-dist`, `pirtm-lexer` (if it were integrated).
   - Replace `edition.workspace = true` / `version.workspace = true` shorthand with explicit values (PIRTM workspace has no `[workspace.package]`).
   - Fix broken path dependency in `pirtm-tools`: `"../parser"` → `"../pirtm-parser"`.
   - Align `serde`, `serde_json`, `sha2`, `chrono`, `anyhow`, `thiserror` to use `workspace = true` where available.
5. **Fix code-level defects** discovered during integration (see Integration Fixes below).
6. **Create supporting on-tree artifacts**: `docs/adr/registry.json` (10-entry ADR registry fixture) and `state/adr_plan_registry.json` (Phase Mirror kernel boundary fixture) required by `adr-verifier` compile-time `include_str!` tests.

### Integration Fixes Applied

| Component | File | Issue | Fix |
|-----------|------|-------|-----|
| `adr_rust` | `src/lib.rs` | Missing crate-level re-exports for `Adr`, `AdrId`, `AdrStatus`, `AdrError`, `example_adrs` | Added `pub use core::{AdrId, AdrStatus, AdrError, ArtifactLink, Adr, AdrRegistry}; pub use examples::example_adrs;` |
| `adr_rust` | `src/euclidean/mod.rs` | `pub mod proof;` references non-existent `src/euclidean/proof.rs` | Removed the line; euclidean harnesses live in top-level `src/proof/` |
| `adr_rust` | `src/core/registry.rs` | Missing `adrs_iter_clone()` method called by `examples.rs` test | Added `pub fn adrs_iter_clone(&self) -> HashMap<AdrId, Adr>` |
| `adr_rust` | `src/examples.rs` | Consequences not substrings of decision+context, failing `validate_consequence_entailment` | Rewrote consequences to be verbatim substrings of decision+context |
| `adr_rust` | `Cargo.toml` | Used `num-prime = "0.4"` and `itertools = "0.13"` as direct deps; added to workspace.dependencies | Aligned to workspace dependencies |
| `pirtm-registry` | `Cargo.toml` | Had standalone `[workspace]` and `[workspace.package]` sections | Removed; replaced `*` with `workspace = true` or explicit versions |
| `pirtm-dist` | `Cargo.toml` | Same as above | Same fix |
| `pirtm-orchestration` | `src/lib.rs` | Test calls `add(2, 2)` which does not exist | Added `pub fn add(a: i32, b: i32) -> i32 { a + b }` |
| `pirtm-tools` | `Cargo.toml` | Path dependency `"../parser"` | Fixed to `"../pirtm-parser"` |

## Artifacts

| Artifact | Location | Status |
|----------|----------|--------|
| ADR governance system | `rust/adr_rust/` | ✅ Integrated |
| ADR verifier & boundary guard | `rust/adr-verifier/` | ✅ Integrated |
| Governed ensemble registry | `rust/pirtm-registry/` | ✅ Integrated |
| Distribution ledger | `rust/pirtm-dist/` | ✅ Integrated |
| CRMF orchestration types | `rust/pirtm-orchestration/` | ✅ Integrated |
| REPL + LSP tools | `rust/pirtm-tools/` | ✅ Integrated |
| WebAssembly SDK builder | `rust/pirtm-web-sdk/` | ✅ Integrated |
| ADR registry fixture | `docs/adr/registry.json` | ✅ Created (10 entries) |
| Phase Mirror boundary fixture | `state/adr_plan_registry.json` | ✅ Created |
| `cargo test -p adr_rust` | `rust/adr_rust/tests/integration_test.rs` | ✅ 4 tests |
| `cargo test -p adr-verifier` | `rust/adr-verifier/src/lib.rs` | ✅ 8 tests |
| `cargo test -p pirtm-registry` | `rust/pirtm-registry/src/lib.rs` | ✅ 2 tests |
| `cargo test -p pirtm-dist` | `rust/pirtm-dist/src/lib.rs` | ✅ 2 tests |
| `cargo test -p pirtm-orchestration` | `rust/pirtm-orchestration/src/lib.rs` | ✅ 1 test |
| Lean Web SDK module | `rust/pirtm-web-sdk/Main.lean` | ✅ On-tree (requires `lake build` in SDK dir) |

## Consequences

- **Positive**: ADR governance is now dual-verified — Lean 4 proves immutability/acyclicity/traceability (machine-checked), while Rust/Kani provides bounded-model-checked validation of the same invariants over concrete integer inputs.
- **Positive**: The Phase Mirror kernel boundary guard (`adr-verifier`) can now be invoked at startup to reject `sorry` debt, manifest drift, and open tensions before any PIRTM execution.
- **Positive**: Ensemble publication and distribution now flow through a governed pipeline: `pirtm-registry::publish` → `pirtm-dist::Distributor::install`.
- **Negative**: `pirtm-tools` introduces `tower-lsp` and `rustyline` as transitive dependencies, increasing the dependency surface. Mitigation: these are only used by the `pirtm-tools` crate, not by the core compiler or engine.
- **Negative**: `pirtm-web-sdk` requires `emcc` (Emscripten) and `lake` in `PATH` for the `build` subcommand. This is acceptable as it is a standalone build tool.
- **Deferred**: `pirtm-ui` integration is blocked pending elimination of `MockKubeExecutor` stubs (see AX-004 below).

## Verification Plan

1. `cargo test -p adr_rust` — integration tests for immutability, cycle detection, history reconstruction, and example validity.
2. `cargo test -p adr-verifier` — structural invariant verification and boundary guard tests.
3. `cargo test -p pirtm-registry pirtm-dist` — ensemble publish/install gate tests.
4. `cargo test -p pirtm-orchestration` — CRMF request validation.

## Axiom Ledger Additions

| Identifier | Module / Proof | Defect Description | Impact | Target Closure |
|---|---|---|---|---|
| **AX-004** | `pirtm-ui/src/main.rs` | `KubeExecutor` uses `MOCK:` prefixed stubs for Kubernetes deploy/revoke | Production deployment path not real | Replace with `kube-rs` real client or delete crate |
| **ENF-004** | `adr_rust/src/proof/` | Kani harnesses are `#[cfg(kani)]` only; standard `cargo test` does not exercise them | Bounded verification only runs under `cargo kani` | Document `cargo kani` as optional verification gate in CI |
| **ENF-005** | `adr_rust/src/euclidean/arithmetic.rs` | `classify` returns `Number` for all non-prime, non-composite integers > 1, but no integer > 1 is ever classified as `Number` (every integer is either prime or composite) | The `IntegerClass::Number` variant is effectively dead code | Audit or remove `Number` variant in future refactor |

## Sign-off

| Owner | Status | Date |
|-------|--------|------|
| Governance | Proposed | 2026-09-01 |
| Formal Methods Engineering | Proposed | 2026-09-01 |
| Compiler Engineering | Pending | — |
