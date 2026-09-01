# PIRTM/MOC Compute Language

PIRTM/MOC is a formally governed, contractive systems programming language combining mathematical kernel verification (Sedona Spine, prime-indexed tensor sheaf contractions) with general-purpose language syntax lowered to MLIR.

---

## 🎯 Verified On-Tree Capabilities

| Subsystem / Feature | Status | Verification & Evidence |
| :--- | :---: | :--- |
| **Arithmetic, let-bindings, blocks** | ✅ | Unit tests in `pirtm-parser` and `pirtm-mlir` |
| **Prime operators (S, A, R, Π, Δ)** | ✅ | Mathlib-free canonical Lean 4 core in `lean/` |
| **Mutable State (`let mut`, `=`)** | ✅ | Stack alloca/store/load lowered in `pirtm-mlir` |
| **User Types & Methods (`struct`, `enum`, `impl`)** | ✅ | Type lowering and method call dispatch to FFI built-ins |
| **Pattern Matching & Control Flow (`match`, `while`, `loop`)** | ✅ | Lowered to MLIR `scf.switch`, `scf.while`, `scf.if` |
| **Error Handling (`Result<T, E>`, `Option<T>`, `?`)** | ✅ | Option unwrap and return propagation |
| **MLIR Lowering Pipeline** | ✅ | `examples/json_parser.pirtm` → `examples/json_parser.mlir` |
| **WardMonitor Runtime Governance** | ✅ | Zeno damping and `SIG_GOV_KILL` kill-switch |
| **Sedona Spine CI Enforcement Gate** | ✅ | `.github/workflows/sedona_spine_ci.yml` |
| **Small-Gain Spectral Radius Gate ($\rho < 1.0$)** | ⚠️ In Progress | Tracked in [docs/PIRTM-axiom-ledger.md](docs/PIRTM-axiom-ledger.md) |

---

## 📋 Architectural Governance & Axiom Ledger

- **Scope Boundary**: [ADR-013: Scope Boundary of PIRTM/MOC](docs/ADR-013-PIRTM-MOC-Language-Scope.md)
- **Grammar Authority**: [ADR-014: Dual-Grammar Authority & Quarantine](docs/ADR-014-Grammar-Authority.md)
- **Formal Axiom & Enforcement Ledger**: [docs/PIRTM-axiom-ledger.md](docs/PIRTM-axiom-ledger.md)
- **Comprehensive Claim Table**: [docs/PIRTM-README-Claim-Table.md](docs/PIRTM-README-Claim-Table.md)

---

## 🚀 Quick Start

### Compiling PIRTM Source to MLIR
```bash
cargo run -p pirtm-compiler --bin pirtm -- compile examples/json_parser.pirtm --output examples/json_parser.mlir
```

### Running Test Suite
```bash
cargo test --workspace
```
