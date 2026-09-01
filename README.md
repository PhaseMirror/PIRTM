# PIRTM/MOC Sovereign Core

[![Sedona Spine CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/sedona_spine_ci.yml/badge.svg)](.github/workflows/sedona_spine_ci.yml)
[![Governed Toolchain CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/governed_toolchain.yml/badge.svg)](.github/workflows/governed_toolchain.yml)
[![Lean 4: Axiom-Clean](https://img.shields.io/badge/Lean%204-Mathlib--Free%20Core-brightgreen.svg)](lean/)
[![Rust Workspace](https://img.shields.io/badge/Rust-1.80%2B%20%7C%2021%20Crates-orange.svg)](rust/)
[![ADR Formal Verification](https://img.shields.io/badge/ADR%20Proofs-ADR--001..046%20100%25-blue.svg)](docs/adr/)
[![License](https://img.shields.io/badge/License-Prime%20Materia%20Commons-blue.svg)](LICENSE)

**PIRTM/MOC** (Phase Mirror / Multiplicity Object Code) is a formally governed, contractive systems programming language and runtime framework. It integrates a verified mathematical kernel (Sedona Spine, prime-indexed tensor sheaf contractions, Lyapunov drift bounding, and Goldilocks prime field arithmetic) with general-purpose language syntax lowered to MLIR and LLVM, exposed through CLI, MCP, WASM, and a Next.js governed web app.

---

## 📖 Table of Contents

- [Architecture Overview](#-architecture-overview)
- [Grounded Status & Verified Capabilities](#-grounded-status--verified-capabilities)
- [Formal ADR Registry (ADR-001 through ADR-046)](#-formal-adr-registry-adr-001-through-adr-046)
- [Locked Governance Constants](#-locked-governance-constants)
- [Repository Layout](#-repository-layout)
- [Installation & Quick Start](#-installation--quick-start)
- [Testing & Machine Attestation](#-testing--machine-attestation)
- [Documentation & Release Synthesis](#-documentation--release-synthesis)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🏛️ Architecture Overview

The PIRTM/MOC architecture enforces structural segregation between formal governance and application-level compute:

```
                       ┌───────────────────────────────────────────────┐
                       │             PIRTM Source Program              │
                       │       `let mut`, `match`, `impl`, `tensor`    │
                       └───────────────────────┬───────────────────────┘
                                               │
                      ┌────────────────────────┴────────────────────────┐
                      ▼                                                 ▼
      ┌───────────────────────────────┐                 ┌───────────────────────────────┐
      │  Kernel & Governance Layer    │                 │   Application Compiler Layer  │
      │  (tree-sitter-pirtm / CSC)    │                 │    (pirtm-parser / mlir)      │
      ├───────────────────────────────┤                 ├───────────────────────────────┤
      │ • Prime Sheaf Contractions    │                 │ • Struct / Enum / Impl AST    │
      │ • Operator Gain Bounds (λ_i)  │                 │ • Stack Alloca / Store / Load │
      │ • Contractivity Assertions    │                 │ • Method Dispatch & FFI       │
      │ • Small-Gain Spectral Radius  │                 │ • scf.if / scf.while / switch │
      │ • Goldilocks Field (F_p)      │                 │ • Poseidon2 ZK Constraints    │
      └───────────────┬───────────────┘                 └───────────────┬───────────────┘
                      │                                                 │
                      │                                                 ▼
                      │                                 ┌───────────────────────────────┐
                      │                                 │       MLIR Text Module        │
                      │                                 │   `examples/json_parser.mlir` │
                      │                                 └───────────────┬───────────────┘
                      │                                                 │
                      └────────────────────────┬────────────────────────┘
                                               ▼
                               ┌───────────────────────────────┐
                               │     Governed Runtime & JIT    │
                               │  (pirtm-monitor / WASM / MCP) │
                               ├───────────────────────────────┤
                               │ • Zeno Damping Attenuation    │
                               │ • Real-time Drift Check (δ)   │
                               │ • SIG_GOV_KILL Enforcement    │
                               │ • Next.js Playground / UI     │
                               └───────────────────────────────┘
```

---

## 🎯 Grounded Status & Verified Capabilities

All core capabilities are physically implemented on-tree and verified by machine-checked test suites:

| Subsystem / Feature | Status | Verification & Test Evidence |
|---|---|---|
| **Lexer & Parser** | ✅ Verified | `pirtm-parser/tests/test_json_parser.rs` (17 top-level constructs) |
| **Mutable State & Structs** | ✅ Verified | Stack `llvm.alloca`, `llvm.store`, `llvm.load` in `pirtm-mlir` |
| **Complex Lowering** | ✅ Verified | End-to-end lowering of `examples/json_parser.pirtm` (397-line MLIR) |
| **WardMonitor Runtime Drift** | ✅ Verified | Zeno damping and `SIG_GOV_KILL` tripwire |
| **Small-Gain Spectral Radius ($\rho < 1.0$)** | ✅ Verified | `pirtm-engine::spectral` computing true $\rho(\|A\|\,\mathrm{diag}(\lambda))$ |
| **Goldilocks Prime Field ($\mathbb{F}_p$)** | ✅ Verified | `pirtm-goldilocks` (12 unit tests, NTT, AVX2/SSE, Kani proof) |
| **Formal ADR Suite (001–046)** | ✅ Verified | Zero `sorry` in Lean 4 (`lake build && lake test` = 42 jobs green) |
| **Rust / Kani Verification Substrate** | ✅ Verified | 42 `adr_rust` tests + 100% pass rate across 21 workspace crates |
| **Governed Web UI App** | ✅ Verified | Next.js WASM compilation, MLIR preview, MCP API routes, Recharts dashboard |

---

## 📜 Formal ADR Registry (ADR-001 through ADR-046)

The framework is governed by 46 machine-checked Architecture Decision Records:
- **ADR-001..030**: Core project setup, grammar authority, toolchain locking, metric unification, and zero-sorry core.
- **ADR-031..033**: Foundry component integration, prime-recursive foundations, and QMHES post-quantum hybrid encryption.
- **ADR-034**: Prime-Indexed Dialectical Semantics & Contestation Fields.
- **ADR-035**: Prime-Encoded Quantum States & Subspace Error Detection.
- **ADR-036**: Prime-Structured Tensor-Network Autoencoder (TN-AE).
- **ADR-037**: Prime-Indexed Phase-Dissonance Functionals.
- **ADR-038**: Phase Mirror Governance Manifold & Fail-Closed Control.
- **ADR-039**: Cognitive Economy & Ethical Projection Substrate.
- **ADR-040**: EchoBraid Quantum Feedback & Recursive Spectrum Coherence.
- **ADR-041**: Multiplicity Floer Differential Operator.
- **ADR-042**: Prime-Constitutional Order & Conscious Sovereignty Layer (CSL).
- **ADR-043**: Lawful Recursion License ($\Xi$-License v1.0).
- **ADR-044**: Phase Mirror Comprehensive ADR Registry Reconciliation.
- **ADR-045**: UI/UX Integration & Governed Toolchain Web Application.
- **ADR-046**: The Goldilocks Prime Field Backend for ZK Circuit Acceleration.

---

## 🔒 Locked Governance Constants (ADR-PML-057)

Runtime governance parameters derive from the contraction decay modulus $\lambda = 0.97$:

| Constant | Value | Role |
|---|---|---|
| $\lambda_{\text{base}}$ | `0.97` | Primary contraction decay modulus |
| $\Delta_{\max}$ | `0.03` | Universal per-step drift rejection threshold ($1 - \lambda_{\text{base}}$) |
| $\tau$ | `1.03` | Maximum Lyapunov growth ceiling |
| $\rho_{\text{warn}}$ | `0.85` | Amber boundary: Zeno controller $\kappa(t) = \kappa_0 e^{-\alpha t}$ |
| $S_{\text{critical}}$ | `0.95` | Red boundary: Critical entropy buffer zone |
| $\rho_{\text{halt}}$ | `1.00` | Non-contractive boundary limit |
| `SIG_GOV_KILL` | `1.05` | Hard phase transition threshold |
| $p_{\text{goldilocks}}$ | `18446744069414584321` | Goldilocks prime modulo $2^{64} - 2^{32} + 1$ |

---

## 📂 Repository Layout

```
PIRTM/
├── CONTRIBUTING.md                    # Open-source contribution guidelines
├── README.md                          # Repository documentation
├── lakefile.lean / lean-toolchain     # Lean 4 pinned build environment
├── .github/workflows/
│   ├── sedona_spine_ci.yml            # Zero-tolerance CI gate
│   └── governed_toolchain.yml         # Lean 4 + Cargo + Next.js build pipeline
├── docs/
│   ├── adr/                           # Markdown ADR specifications (001–046) & registry.json
│   ├── PIRTM_Paper.tex                # Academic paper LaTeX source
│   ├── PIRTM_v3_Release_Synthesis.md  # Technical release synthesis
│   └── PIRTM-axiom-ledger.md          # Proof debt & enforcement ledger
├── lean/
│   └── Foundations/ADR/               # Canonical Lean 4 proof suite (ADR-001..046)
├── rust/
│   ├── adr_rust/                      # Kani proof harnesses & Rust models
│   ├── pirtm-compiler/                # MLIR & bytecode compiler engine
│   ├── pirtm-engine/                  # Sedona Spine engine & spectral validation
│   ├── pirtm-goldilocks/              # Goldilocks prime field arithmetic & NTTs
│   ├── pirtm-mcp/                     # Model Context Protocol governance server
│   └── pirtm-web-sdk/                 # Lean/Rust WASM component builders
├── pirtm-governed-toolchain/          # Next.js 15 Governed Web App & Playground
└── scripts/release_v3.sh              # Automated release verification script
```

---

## 🚀 Installation & Quick Start

### Prerequisites
- **Lean 4**: `v4.33.0-rc2` (via `elan`)
- **Rust**: `1.80+`
- **Node.js**: `20+`

### Build Commands

```bash
# 1. Run Lean 4 formal verification suite
lake build
lake test
lake run generateDocs

# 2. Build & test all 21 Rust workspace crates
cd rust
cargo test --workspace

# 3. Launch Next.js Governed Toolchain Web App
cd ../pirtm-governed-toolchain
npm run dev
```

---

## 🧪 Testing & Machine Attestation

- **Lean 4 Core**: `lake test` runs 14 theorem test cases across all ADR modules.
- **Cargo Workspace**: `cargo test --workspace` passes 100% of unit tests across 21 crates.
- **Model Checking**: `cargo kani` verifies bounded model checking harnesses in `adr_rust` and `pirtm-goldilocks`.

---

## 📄 License

Licensed under the **Prime Materia Open Commons License v1.0** and **Lawful Recursion License ($\Xi$-License v1.0)**.
