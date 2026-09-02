# PIRTM/MOC Kernel Language v1.0.0-mvp

[![Sedona Spine CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/sedona_spine_ci.yml/badge.svg)](.github/workflows/sedona_spine_ci.yml)
[![Lean 4](https://img.shields.io/badge/Lean%204-Mathlib--Free%20Tree-lightgrey.svg)](lean/)
[![Rust Workspace](https://img.shields.io/badge/Rust-1.80%2B%20%7C%2022%20workspace%20members-orange.svg)](rust/)
[![ADR Status](https://img.shields.io/badge/ADR%20Proofs-partial%20%7C%20not%20100%25-yellow.svg)](docs/PIRTM-README-Claim-Table.md)
[![License](https://img.shields.io/badge/License-Prime%20Materia%20Commons-blue.svg)](LICENSE)

**PIRTM/MOC** (Phase Mirror / Multiplicity Object Code) is an L0 kernel language and runtime under construction in this repository. A session is intended to execute only after transpile-time receipts and link-time spectral small-gain

$$
\rho\bigl(|A|\,\mathrm{diag}(\lambda)\bigr) < 1
$$

with each $\lambda_j$ bound to a named theorem or axiom-ledger row. Author-declared floats are manifests, not certificates.

This tree is **not** a general-purpose replacement for Rust, Lean, or C++. Application tokens (`if`, `fn`, `struct`, `let mut`) are not kernel features until a bounded-iteration lowering theorem exists on this tree (ADR-013 original gates; ADR-014 kernel grammar authority is tree-sitter). CHANGELOG records `1.0.0-mvp`. Lake package version is `0.1.0`. There is no v3.0.0-Stable tag on this SHA.

Local ADRs 052–054 remain **Proposed**. This README change does not accept them.

---

## Table of Contents

- [Architecture](#architecture)
- [Grounded status](#grounded-status)
- [ADR registry (claim discipline)](#adr-registry-claim-discipline)
- [Governance constants](#governance-constants)
- [Repository layout](#repository-layout)
- [Build](#build)
- [Testing](#testing)
- [License](#license)

---

## Architecture

Kernel substrate and application substrate are segregated. Segregation is not proof that both layers are complete.

```
PIRTM source
        |
        +-- kernel (tree-sitter / contractivity receipts / small-gain)
        |
        +-- application compiler (pirtm-app-lexer is non-executing for L0
            until Exhibit C lowering exists)
        |
        v
MLIR text (examples/) --> pirtm-engine (spectral.rs present;
                          lambda vectors still unsigned f64)
```

CI that exists on disk: `.github/workflows/sedona_spine_ci.yml` (elan pin + `lake build`). There is no `governed_toolchain.yml` on this tree. Do not badge a missing file.

---

## Grounded status

Status markers: **Present** = file and some tests exist. **Unclaimed** = README must not say Verified. **Partial** = on-tree with a named defect.

| Subsystem | Status | Evidence / defect |
|---|---|---|
| Lexer and parser | Present | `pirtm-parser` tests exist; dual-lexer quarantine is not ADR-014 close |
| MLIR lowering examples | Present | `examples/json_parser.pirtm` / `.mlir` on tree |
| Small-gain engine | Partial | `rust/pirtm-engine/src/spectral.rs` computes $\rho(|A|\mathrm{diag}(\lambda))$; $\lambda$ is author `f64`; receipt has no `theorem_name` |
| WardMonitor / Lyapunov | Unclaimed | `WardMonitorStability.lean` proves Nat scaling of $V=\rho^2$, not runtime Lyapunov |
| Poseidon2 ZK soundness | Unclaimed | `Poseidon2Soundness.lean` is `isValid && count <= 5087`; tautology pending ADR-053 |
| Multi-node cluster consensus | Unclaimed | `pirtm-orchestration` crate exists; quorum soundness not an engineering predicate |
| Formal ADR suite 001–050 | Unclaimed | Documents and Lean modules exist; not 100 percent; job counts in docs disagree (7 / 18 / 38 / 50) |
| Rust workspace | Partial | `rust/Cargo.toml` lists 22 members; `pirtm-clinical` and `pirtm-moc` are on disk and not members |
| Kani substrate | Partial | Harnesses are `#[cfg(kani)]`; `cargo test` does not run them (ledger ENF-004) |
| Governed web UI / WASM / playground | Unclaimed | Out of ADR-013 horizon until kernel CI includes `cargo test --workspace` |
| Lean `sorry` in `lean/` | Partial | `grep sorry lean/` claimed empty; production FFI still has axioms in `pirtm-stdlib/Lean` and `pirtm-clinical` |

Canonical claim matrix: `docs/PIRTM-README-Claim-Table.md`. Where that file still says Complete for an Unclaimed row above, the claim table is stale. Do not treat either file as Layer B.

---

## ADR registry (claim discipline)

Markdown ADR files through ADR-050 exist under `docs/adr/`. Existence is not machine-checked soundness of the title.

Do not list ADR-049 or ADR-050 as verified soundness. Do not list ADR-001 through ADR-050 as a closed suite.

Kernel scope lock remains the original ADR-013 four gates on this repository only. The shortened `docs/adr/completed/ADR-013-PIRTM-MOC-Language-Scope.md` is not authority.

Proposed local artifacts (not on this commit, not Accepted):

- ADR-052 Reject v3 false completeness
- ADR-053 Theorem name equals theorem content
- ADR-054 Single authority trees

---

## Governance constants

Declared runtime numbers (`lambda_base = 0.97`, `C_poseidon2 = 5087`, and others in prior README text) are **policy literals**. They are not Lean theorems. `5087` in `verifyPoseidon2Receipt` is the same class of literal.

---

## Repository layout

```
PIRTM/
├── README.md                         # this file; mvp claim surface
├── CHANGELOG.md                      # 1.0.0-mvp
├── lakefile.lean / lean-toolchain    # Lake package version 0.1.0
├── .github/workflows/
│   └── sedona_spine_ci.yml           # only workflow on disk
├── docs/
│   ├── adr/
│   ├── PIRTM-README-Claim-Table.md
│   └── PIRTM-axiom-ledger.md
├── lean/
│   ├── ADR/                          # one of two ADR trees; ADR-054 unresolved
│   ├── Foundations/ADR/              # the other ADR tree
│   └── prime_tensors/
├── rust/                             # 22 workspace members in Cargo.toml
├── pirtm-governed-toolchain/         # present; not a verified production UI
└── examples/
```

---

## Build

### Prerequisites

- Lean 4 `v4.33.0-rc2` via `elan` (must match `lean-toolchain`)
- Rust 1.80+

### Commands that exist

```bash
lake build
cd rust && cargo test --workspace
```

`lake test` and `cargo kani` are optional until CI runs them. Do not cite them as green on this SHA without a log.

Playground `npm run dev` is not a kernel gate.

---

## Testing

On-disk CI job: toolchain pin plus `lake build`.

Not attested by that job: `cargo test --workspace`, `cargo kani`, Next.js, Poseidon2 knowledge soundness, cluster quorum, 50 ADR proofs.

---

## License

Licensed under the Prime Materia Open Commons License v1.0 and the Lawful Recursion License (Xi-License v1.0). See `LICENSE` and `Ξ-LICENSE`.
