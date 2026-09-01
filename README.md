# PIRTM/MOC Compute Language

[![Sedona Spine CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/sedona_spine_ci.yml/badge.svg)](.github/workflows/sedona_spine_ci.yml)
[![Lean 4: Axiom-Clean](https://img.shields.io/badge/Lean%204-Mathlib--Free%20Core-brightgreen.svg)](lean/)

**PIRTM/MOC** is a formally governed, contractive systems programming language that integrates a verified mathematical kernel (Sedona Spine, prime-indexed tensor sheaf contractions, Lyapunov drift bounding) with general-purpose language syntax lowered to MLIR and LLVM.

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
|---|:---:|---|
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
|---|:---:|---|
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
├── .github/
│   └── workflows/
│       └── sedona_spine_ci.yml        # Zero-tolerance CI enforcement workflow
├── docs/
│   ├── ADR-013-PIRTM-MOC-Language-Scope.md    # Small-gain scope boundary
│   ├── ADR-014-Grammar-Authority.md           # Tree-sitter authority & quarantine
│   ├── ADR-015-Reject-False-Delivery-Pack.md  # Rejection of speculative off-tree packs
│   ├── PIRTM-axiom-ledger.md                  # Proof debt & unmirrored enforcement log
│   └── PIRTM-README-Claim-Table.md            # Comprehensive claim verification matrix
├── examples/
│   ├── json_parser.pirtm                      # Full JSON parser in PIRTM
│   └── json_parser.mlir                       # Verified compiler output (397 lines)
├── lean/
│   ├── ADR/                                   # Verified ADR dependent type core
│   ├── lakefile.toml                          # Mathlib-free, self-contained Lean config
│   └── PIRTM.lean                             # Canonical mathematical kernel definitions
└── rust/
    ├── pirtm-compiler/                        # Compiler CLI, linker, and Lean wrappers
    ├── pirtm-engine/                          # FFI runtime built-ins & execution harness
    ├── pirtm-lexer/                           # Logos-based high-performance tokenizer
    ├── pirtm-mlir/                            # MLIR dialect operations & AST visitor
    ├── pirtm-monitor/                         # WardMonitor drift & Zeno controller
    ├── pirtm-parser/                          # Recursive descent parser & EBNF decoder
    ├── pirtm-rs/                              # Kani/Jury-Schur stability verification
    └── pirtm-stdlib/                          # Verified standard library primitives
```

---

## 📋 Governance & Axiom Ledger Cross-References

- **[ADR-013: Scope Boundary of PIRTM/MOC](docs/ADR-013-PIRTM-MOC-Language-Scope.md)**: Rejects author-declared float sums as stability proofs.
- **[ADR-014: Dual-Grammar Authority](docs/ADR-014-Grammar-Authority.md)**: Establishes `tree-sitter-pirtm` as sole kernel authority and quarantines control flow.
- **[ADR-015: Reject False Delivery Packs](docs/ADR-015-Reject-False-Delivery-Pack.md)**: Mandates strict on-tree physical verification.
- **[Axiom & Enforcement Ledger](docs/PIRTM-axiom-ledger.md)**: Complete tracking of proof obligations (`AX-001`–`AX-003`) and enforcement mechanisms (`ENF-001`–`ENF-003`).
- **[Grounded Claim Table](docs/PIRTM-README-Claim-Table.md)**: Full claim-by-claim verification table.

---

## 🚀 Quick Start & Verification

### 1. Build and Run Full Test Suite
```bash
cd rust
cargo test --workspace
```

### 2. Compile PIRTM Source to MLIR
```bash
cargo run -p pirtm-compiler --bin pirtm -- compile ../examples/json_parser.pirtm --output ../examples/json_parser.mlir
```

### 3. Verify Lean 4 Axiom-Clean Core
```bash
cd lean
lake build
```
