# PIRTM v1.1.0 Release Notes
### Interactive Governed Development Environment & TUI
**Release Date**: 2 September 2026  
**LawfulRecursionVersion**: 1.0  
**Legal Persona**: Citizen Gardens UNA d/b/a The Prime Materia Commons

---

## 🌟 Highlights

PIRTM v1.1.0 introduces a **Kilo / OpenCode style interactive terminal environment** bringing the full power of formal Lean 4 verification, Sentinel governance gates, WardMonitor stability tracking, and MCP AI assistance into a developer-first split-pane workspace.

---

## 🚀 What's New

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
- Integrated into `docker-compose.yml` for containerized orchestration.

---

## 🔒 Verification & Compliance Status

- **Lean 4 Proofs**: `lake test` $\rightarrow$ ✅ PASS (all 24 ADR tests including ADR-055 `PosRatContractivity`).
- **Rust/Kani Suite**: `cargo test --workspace` $\rightarrow$ ✅ PASS (0 errors across 28 workspace crates).
- **Exact Rational Small-Gain Gate**: $\|G\|_1 = \max_j \sum_i |A_{ij}| \cdot \lambda_j < 1.0$ strictly verified over $\mathbb{Q}$.
