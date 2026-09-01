# Contributing to PIRTM Sovereign Core

Thank you for your interest in contributing to the **Multiplicity Sovereign Core (PIRTM)**. This project implements a formally verified, governance-as-compilation architecture.

---

## 📜 Principles & Rules

1. **Zero Proof Debt**: No `sorry` warnings are permitted in the canonical Lean 4 proof suite (`lean/`).
2. **Zero Mathlib Core**: The canonical `Foundations.ADR` module must remain 100% Mathlib-free.
3. **Sedona Spine Compliance**: All spoliation and risk decisions must route strictly through the Rust engine.

---

## 🛠️ Local Development & Setup

### Prerequisites
- Lean 4 (`elan`)
- Rust toolchain (`cargo`)
- Node.js 20+

### Building & Testing

```bash
# 1. Run Lean 4 formal verification
lake build
lake test
lake run generateDocs

# 2. Run Rust workspace tests
cd rust
cargo test --workspace

# 3. Run Web Application
cd ../pirtm-governed-toolchain
npm run dev
```

---

## 🚀 Submitting Pull Requests

1. Ensure all 46 ADR specifications pass machine-checked verification.
2. Verify that `cargo test --workspace` passes 100% across all 21 crates.
3. Include new ADR markdown documents and Lean/Rust modules for architectural additions.
