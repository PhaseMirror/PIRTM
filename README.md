# PIRTM/MOC Compute Language

[![Sedona Spine CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/sedona_spine_ci.yml/badge.svg)](.github/workflows/sedona_spine_ci.yml)
[![Lean 4: Axiom-Clean](https://img.shields.io/badge/Lean%204-Mathlib--Free%20Core-brightgreen.svg)](lean/)
[![Rust](https://img.shields.io/badge/Rust-1.81%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Prime%20Materia%20Commons-blue.svg)](LICENSE)

**PIRTM/MOC** (Phase Mirror / Multiplicity Object Code) is a formally governed, contractive systems programming language that integrates a verified mathematical kernel (Sedona Spine, prime-indexed tensor sheaf contractions, Lyapunov drift bounding) with general-purpose language syntax lowered to MLIR and LLVM.

---

## 📖 Table of Contents

- [Architecture Overview](#-architecture-overview)
- [Grounded Status & Verified Capabilities](#-grounded-status--verified-capabilities)
- [Locked Governance Constants](#-locked-governance-constants)
- [Repository Layout](#-repository-layout)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage Examples](#-usage-examples)
- [Governance & Axiom Ledger](#-governance--axiom-ledger)
- [Testing](#-testing)
- [Documentation](#-documentation)
- [Contributing](#-contributing)
- [Security](#-security)
- [License](#-license)
- [Contact](#-contact)

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
                               │        (pirtm-monitor)        │
                               ├───────────────────────────────┤
                               │ • Zeno Damping Attenuation    │
                               │ • Real-time Drift Check (δ)   │
                               │ • SIG_GOV_KILL Enforcement    │
                               └───────────────────────────────┘
```

---

## 🎯 Grounded Status & Verified Capabilities

The capabilities below are physically implemented and verified by automated on-tree test suites:

| Subsystem / Feature | Status | Verification & Test Evidence |
|---|---|---|
| **Lexer & Parser** | ✅ Verified | `pirtm-parser/tests/test_json_parser.rs` (17 top-level constructs) |
| **Mutable State (`let mut`, `=`)** | ✅ Verified | Stack `llvm.alloca`, `llvm.store`, `llvm.load` in `pirtm-mlir` |
| **User Data Types (`struct`, `enum`)** | ✅ Verified | `!llvm.struct` and `!llvm.enum` lowering in `pirtm-mlir` |
| **Methods & Chaining (`impl`, `.method()`)** | ✅ Verified | Method dispatch to FFI built-ins (`string_len`, `vec_push`, `map_insert`) |
| **Pattern Matching & Control Flow** | ✅ Verified | Lowered to MLIR `scf.switch`, `scf.while`, `scf.if` |
| **Error Propagation (`Result`, `Option`, `?`)** | ✅ Verified | Direct unwrapping and return-flow propagation |
| **Complex Program Lowering** | ✅ Verified | End-to-end lowering of `examples/json_parser.pirtm` (397-line MLIR) |
| **WardMonitor Runtime Drift** | ✅ Verified | Exponential Zeno damping and `SIG_GOV_KILL` kill-switch |
| **Sedona Spine CI Gate** | ✅ Verified | `.github/workflows/sedona_spine_ci.yml` on-tree |
| **Small-Gain Spectral Radius ($\rho < 1.0$)** | ✅ Verified | `pirtm-engine/src/spectral.rs` computing true $\rho(\|A\|\,\mathrm{diag}(\lambda))$ |

---

## 🔒 Locked Governance Constants (ADR-PML-057)

All runtime governance parameters derive from the foundational **decay modulus** $\lambda = 0.97$:

| Constant | Value | Mathematical Derivation / Role |
|---|---|---|
| $\lambda_{\text{base}}$ | `0.97` | Primary contraction decay modulus (state retention factor) |
| $\Delta_{\max}$ | `0.03` | Universal per-step drift rejection threshold ($1 - \lambda_{\text{base}}$) |
| $\tau$ | `1.03` | Maximum Lyapunov growth ceiling ($1 + \Delta_{\max}$) |
| $\rho_{\text{warn}}$ | `0.85` | Amber boundary: Activates Zeno controller $\kappa(t) = \kappa_0 e^{-\alpha t}$ |
| $S_{\text{critical}}$ | `0.95` | Red boundary: Critical entropy buffer zone |
| $\rho_{\text{halt}}$ | `1.00` | Non-contractive boundary limit |
| `SIG_GOV_KILL` | `1.05` | Hard phase transition threshold ($1 / S_{\text{critical}} \approx 1.0526$) |
| $R$ / `DSE_BUDGET` | `5` | Maximum consecutive micro-drift steps before mandatory recalibration |

---

## 📂 Repository Layout

```
PiLang/
├── AGENTS.md                          # Phase Mirror methodology for agents
├── CHANGELOG.md                       # Version history and release notes
├── CONTRIBUTING.md                    # Contribution guidelines
├── CODE_OF_CONDUCT.md                 # Community standards
├── INSTALL.md                         # Installation and setup guide
├── LICENSE                            # Prime Materia Open Commons License v1.0
├── README.md                          # This file
├── SECURITY.md                        # Security policy and vulnerability reporting
├── USER_GUIDE.md                      # End-user guide for PIRTM/MOC
├── build.sh                           # Validated build entrypoint
├── lakefile.toml                      # Lake build configuration
├── lake-manifest.json                 # Pinned toolchain + dependencies
├── lean-toolchain                     # Lean version pin
├── .github/
│   └── workflows/
│       └── sedona_spine_ci.yml        # Zero-tolerance CI enforcement workflow
├── artifacts/                         # Release mirror of docs/
├── docs/
│   ├── ADR-0XX-*.md                   # Active / resolved Architecture Decision Records
│   ├── PIRTM-README-Claim-Table.md    # Canonical ground-truth status matrix (SHA-256 pinned)
│   └── PIRTM-axiom-ledger.md          # Proof debts (AX-*) and enforcement gaps (ENF-*)
├── examples/
│   ├── json_parser.pirtm               # Full JSON parser in PIRTM
│   └── json_parser.mlir                # Verified compiler output
├── lean/
│   ├── ADR/                           # Verified ADR dependent type core
│   │   ├── Core.lean                  # ADR types, status transitions, ArtifactLink
│   │   ├── Proofs.lean                # Theorems: immutability, entailment, traceability
│   │   ├── Examples.lean              # Realistic example ADRs
│   │   ├── Test.lean                  # Test harness: 15+ theorems
│   │   ├── Export.lean                # Markdown/HTML generation from formal ADRs
│   │   ├── ZenoController.lean        # Governance thresholds & Zeno damping
│   │   └── BoundedIteration.lean      # Contractivity proofs for loops & branches
│   ├── PIRTM.lean                     # Kernel: DivLoop, dynamicScalingFactor, adaptiveLambda
│   └── prime_tensors/                 # Prime tensor library
│       ├── Transition.lean
│       ├── Stability.lean
│       ├── CPIRTM.lean
│       └── DRMM.lean
└── rust/
    ├── pirtm-kernel-lexer/            # Kernel-only tokens (tensor, Ap, assert_contractive)
    ├── pirtm-app-lexer/               # Application tokens (let, mut, if, while, fn, struct)
    ├── pirtm-parser/                  # Recursive descent parser & EBNF decoder
    ├── pirtm-mlir/                    # MLIR dialect operations & AST visitor
    ├── pirtm-compiler/                # CLI, linker, Lean wrappers, AdmissibilityValidator
    ├── pirtm-engine/                  # Runtime: real execution, spectral validation, telemetry
    ├── pirtm-monitor/                 # WardMonitor drift detection & Zeno controller
    ├── pirtm-mcp/                     # Model Context Protocol server & tools
    └── pirtm-stdlib/                  # Verified standard library primitives
```

---

## 🚀 Installation

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.81.0+ | Compiler toolchain for `rust/` crates |
| **Lean 4** | v4.33.0-rc2 | Formal verification kernel (managed by `elan`) |
| **Lake** | (bundled with Lean) | Build system for Lean modules |
| **mlir-translate / llc / clang** | LLVM 17+ | Real execution path in `pirtm-engine` |
| **Git** | 2.40+ | Version control |

### Quick Setup

```bash
# 1. Clone the repository
git clone https://github.com/PhaseMirror/PiLang.git
cd PiLang

# 2. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install elan (Lean version manager)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
source $HOME/.elan/env

# 4. Install pinned Lean toolchain
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version

# 5. Build Rust workspace
cd rust
cargo build --workspace

# 6. Build Lean kernel
cd ..
./build.sh
```

For detailed installation instructions, see [INSTALL.md](INSTALL.md).

---

## 🏁 Quick Start

### 1. Build and Run Full Test Suite

```bash
cd rust
cargo test --workspace
```

### 2. Compile PIRTM Source to MLIR

```bash
cd rust
cargo run -p pirtm-compiler --bin pirtm -- compile ../examples/json_parser.pirtm --output ../examples/json_parser.mlir
```

### 3. Verify Lean 4 Axiom-Clean Core

```bash
cd ..
./build.sh
```

**Note:** Run from the `PiLang/` root directory. `lakefile.toml` is at `PiLang/lakefile.toml`, not `PiLang/lean/lakefile.toml`.

---

## 📚 Usage Examples

### Compiling a PIRTM Program

```bash
# Compile json_parser.pirtm to MLIR
cargo run -p pirtm-compiler --bin pirtm -- compile examples/json_parser.pirtm --output examples/json_parser.mlir

# Inspect the generated MLIR
cat examples/json_parser.mlir
```

### Running the Runtime

```bash
# Run with real execution (requires LLVM toolchain)
cargo run -p pirtm-engine --bin pirtm-engine -- run --mlir examples/json_parser.mlir --input "test.json"

# Run in dry-run mode (simulated telemetry)
cargo run -p pirtm-engine --bin pirtm-engine -- run --mlir examples/json_parser.mlir --dry-run
```

### Validating Spectral Contractivity

```bash
# Validate ensemble spectral radius
cargo run -p pirtm-engine --bin pirtm-engine -- validate --ensemble examples/ensemble.json
```

For comprehensive usage examples, see [USER_GUIDE.md](USER_GUIDE.md).

---

## 📋 Governance & Axiom Ledger

PIRTM/MOC operates under the **Phase Mirror** methodology, which treats Architecture Decision Records (ADRs), formal proofs, and compiler artifacts as ground-truth, machine-checkable artifacts.

- **[ADR-013: Scope Boundary of PIRTM/MOC](docs/ADR-013-PIRTM-MOC-Language-Scope.md)**: Rejects author-declared float sums as stability proofs.
- **[ADR-014: Dual-Grammar Authority](docs/ADR-014-Grammar-Authority.md)**: Establishes kernel/app lexer quarantine.
- **[ADR-015: Reject False Delivery Packs](docs/ADR-015-Reject-False-Delivery-Pack.md)**: Mandates strict on-tree physical verification.
- **[ADR-016: Bounded Iteration & Control Flow](docs/ADR-016-Bounded-Iteration-Control-Flow.md)**: Formal proofs for loops, branches, and function composition.
- **[ADR-017: MLIR Lowering Soundness](docs/ADR-017-Lowering-Soundness.md)**: Machine-checked metric preservation proofs.
- **[ADR-018–030: Phase Mirror Audit](docs/)**: All 13 audit ADRs resolved; see `PIRTM-README-Claim-Table.md` for ground-truth status.
- **[Defensive Publication Whitepaper](docs/DEFENSIVE_PUBLICATION_GOVERNANCE_AS_COMPILATION.md)**: Formal prior art disclosure for Governance-as-Compilation.
- **[Axiom & Enforcement Ledger](docs/PIRTM-axiom-ledger.md)**: Complete tracking of proof obligations (`AX-*`) and enforcement mechanisms (`ENF-*`).
- **[Grounded Claim Table](docs/PIRTM-README-Claim-Table.md)**: Full claim-by-claim verification table. **Last audited 2026-09-01.**

---

## 🧪 Testing

### Rust Test Suites

```bash
# Run all Rust tests
cargo test --workspace

# Run specific crate tests
cargo test -p pirtm-parser
cargo test -p pirtm-engine
cargo test -p pirtm-compiler
cargo test -p pirtm-monitor
```

### Lean Proof Verification

```bash
# Build all Lean targets (18 jobs)
./build.sh

# Or explicitly:
lake build --rehash
```

### CI Verification

The Sedona Spine CI enforces:
- Zero-drift Lean toolchain locking (`fixedToolchain: true`)
- Full build verification (`lake build`)
- Proof debt detection (`grep -r "sorry" lean/`)
- Rust test suite execution

---

## 📖 Documentation

| Document | Description |
|----------|-------------|
| [README.md](README.md) | Project overview, installation, and quick start |
| [USER_GUIDE.md](USER_GUIDE.md) | Comprehensive end-user guide |
| [INSTALL.md](INSTALL.md) | Detailed installation and setup instructions |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines and development workflow |
| [SECURITY.md](SECURITY.md) | Security policy and vulnerability reporting |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Community standards and expectations |
| [CHANGELOG.md](CHANGELOG.md) | Version history and release notes |
| [AGENTS.md](AGENTS.md) | Phase Mirror methodology for AI agents |
| [docs/PIRTM-README-Claim-Table.md](docs/PIRTM-README-Claim-Table.md) | Ground-truth status matrix |
| [docs/PIRTM-axiom-ledger.md](docs/PIRTM-axiom-ledger.md) | Proof debts and enforcement gaps |
| [docs/ADR-*.md](docs/) | Architecture Decision Records |

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Code of conduct and community principles
- Development environment setup
- Commit message conventions
- Testing requirements
- Security checklist
- Review and merge process

**Quick contribution checklist:**
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and ensure all tests pass
4. Commit with a conventional commit message
5. Push to your fork and open a Pull Request

---

## 🔒 Security

Security is a first-class concern in the Phase Mirror methodology. Please see [SECURITY.md](SECURITY.md) for:

- Supported versions
- How to report vulnerabilities
- Severity classifications and response times
- Security architecture and threat model
- Infrastructure security controls

**Do not** create public GitHub issues for security vulnerabilities. Email **security@phasemirror.com** instead.

---

## 📄 License

This project is licensed under the **Prime Materia Open Commons and Bound Works License v1.0**.

See [LICENSE](LICENSE) for the full legal text. The license consists of three coordinate parts:
- **Part II** — The Prime Materia Covenant (Public Domain Dedication)
- **Part III** — The Operator Atlas Registry Charter
- **Part VII** — The Bound Works Mark License

Key provisions:
- The Constitutional Core belongs to everyone; what any person makes from it belongs to them.
- No deployment, modification, or use is lawful unless Ξ(t+1) = Ψ(Ξ(t)) (lawful recursion).
- No surveillance, profiling, monetization, or behavioral manipulation.

---

## 📬 Contact

- **Project**: [github.com/PhaseMirror/PiLang](https://github.com/PhaseMirror/PiLang)
- **Security**: security@phasemirror.com
- **Issues**: [GitHub Issues](https://github.com/PhaseMirror/PiLang/issues)
- **Discussions**: [GitHub Discussions](https://github.com/PhaseMirror/PiLang/discussions)

---

*Last Updated: 2026-09-01*
