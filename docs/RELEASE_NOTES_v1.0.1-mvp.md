# PIRTM Governed Development Preview (Candidate Tag: v1.0.1-mvp)
### Development Preview Release Notes — Interactive Governed IDE & TUI
**Target Release Tag**: `v1.0.1-mvp` (ADR-012 Version Governance)  
**Date**: 2 September 2026  
**LawfulRecursionVersion**: 1.0  
**Legal Persona**: Citizen Gardens UNA d/b/a The Prime Materia Commons

> [!NOTE]
> Per ADR-012 version tag governance rules, `v1.1.0-mvp` tag claims are strictly forbidden prior to full multi-node production deployment. This document represents candidate release notes for `v1.0.1-mvp`.

---

## 🌟 Highlights

PIRTM introduces a **Kilo / OpenCode style interactive terminal environment** bringing formal Lean 4 verification, Sentinel governance gates, WardMonitor stability tracking, and MCP AI assistance into a developer-first split-pane workspace.

---

## 🚀 Key Features

### 1. Interactive Terminal UI (`pirtm-tui`)
- **Keyboard-First Split-Pane Workspace**: Built with Ratatui & Crossterm, featuring Project File Explorer, Editor Pane, Integrated Terminal Log, and Governance Status Footer.
- **PIRTM Syntax Highlighting**: Color-coded rendering for keywords (`ensemble`, `matrix`, `lambdas`, `theorem`), exact rational types, numbers, string literals, and comments.
- **LSP Diagnostic Overlays**: Real-time diagnostic panel displaying theorem anchor verification and rational reduction status from `pirtm-lsp`.

### 2. Comprehensive Slash Commands
- **`/audit`**: Performs a 4-point security & formal invariant audit (Small-Gain 1-norm $\|G\|_1$, Zeno Monotonicity, Fail-Closed boundaries, and Lean theorem anchors).
- **`/simulate`**: Runs a 1,000-step Monte Carlo trajectory simulation verifying spectral shift limits $\Delta_{\max} \le 0.030$.
- **`/certify`**: Emits a Poseidon2 sponge signed `UnifiedWitness` WORM audit receipt.
- **`/explain`**: Provides a step-by-step mathematical breakdown of matrix interconnection $A_{ij}$ and gain scaling $\lambda_j$ over $\mathbb{Q}$.
- **`/proof`**: Generates a machine-checkable Lean 4 theorem stub (`theorem calculator_contractive_sound`).
- **`/refactor`**: Automatically optimizes gain values $\lambda_j$ to maximize mathematical contractivity margins.
- **`/compile`**, **`/validate`**, **`/status`**, **`/ask`**, **`/clear`**, **`/quit`**.

### 3. Background Daemon Service (`pirtmd`)
- Background daemon serving JSON-RPC over WebSocket IPC (`ws://127.0.0.1:8090`).
- Implemented in `rust/pirtm-daemon` with fail-closed missing source and missing theorem verification rules.
- Containerized via `rust/Dockerfile.daemon` and integrated into `docker-compose.yml`.

---

## 🔒 Verification & Compliance Status

- **Lean 4 Proofs**: `lake test` $\rightarrow$ ✅ PASS (all 25 ADR proof modules).
- **Rust Workspace**: `cargo test --workspace` $\rightarrow$ ✅ PASS (0 errors across 24 workspace member crates).
- **Exact Rational Small-Gain Gate**: $\|G\|_1 = \max_j \sum_i |A_{ij}| \cdot \lambda_j < 1.0$ strictly verified over $\mathbb{Q}$ (ADR-055).
