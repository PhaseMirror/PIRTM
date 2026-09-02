# PIRTM/MOC Governed Platform v1.1.0

[![Sedona Spine CI](https://github.com/PhaseMirror/PIRTM/actions/workflows/sedona_spine_ci.yml/badge.svg)](.github/workflows/sedona_spine_ci.yml)
[![Lean 4 Proofs](https://img.shields.io/badge/Lean%204-100%25%20Verified%20%7C%20Zero%20Sorry-brightgreen.svg)](lean/)
[![Rust Workspace](https://img.shields.io/badge/Rust-1.80%2B%20%7C%2024%20crates-orange.svg)](rust/)
[![ADR Status](https://img.shields.io/badge/ADR%20Proofs-ADR--034%20to%20ADR--056%20Accepted-blue.svg)](docs/PIRTM-README-Claim-Table.md)
[![License](https://img.shields.io/badge/License-Prime%20Materia%20Commons-blue.svg)](LICENSE)

**PIRTM/MOC** (Phase Mirror / Multiplicity Object Code) is a formally verified, contractive L0 governed development platform, compiler, and interactive terminal environment governed by **Citizen Gardens UNA d/b/a The Prime Materia Commons**.

A program session executes only after transpile-time receipts and link-time spectral small-gain contractivity:

$$
\|G\|_1 = \max_j \sum_{i} |A_{ij}| \cdot \lambda_j < 1.0 \quad \text{in } \mathbb{Q}
$$

are formally verified over exact reduced rational tuples $\mathrm{PosRat} \in \mathbb{Q}$ and anchored to machine-checked Lean 4 theorem declarations.

---

## Table of Contents

- [Architecture](#architecture)
- [Grounded Status & Feature Matrix](#grounded-status--feature-matrix)
- [PIRTM TUI & Daemon (`pirtmd`)](#pirtm-tui--daemon-pirtmd)
- [ADR Formal Proof Suite (ADR-034 to ADR-056)](#adr-formal-proof-suite-adr-034-to-adr-056)
- [Repository Layout](#repository-layout)
- [Build & Verification](#build--verification)
- [License & Legal Entity](#license--legal-entity)

---

## Architecture

PIRTM enforces strict structural segregation between the kernel substrate, formal proof layer, compiler pipeline, interactive daemon service, and client editors.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                      PIRTM Governed Substrate v1.1.0                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  PIRTM Source (.pirtm) ──► pirtm-compiler ──► MLIR lowering             │
│                                │                                        │
│                                ▼                                        │
│  Lean 4 Proof Core ───► Sentinel Gate ───► Exact Rational ||G||₁ < 1.0  │
│  (Zero Mathlib)         (spectral.rs)       Constructor (PosRat in Q)   │
│                                │                                        │
│                                ▼                                        │
│  Client Editors ◄───── WebSocket IPC ◄─── pirtmd Daemon (Port 8090)     │
│  (TUI / VSCode / Nvim)  (JSON-RPC 2.0)    (WardMonitor + MCP Agent)   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Grounded Status & Feature Matrix

| Subsystem | Status | Evidence & Verification Path |
|---|---|---|
| **Lean 4 Proof Core** | ✅ Verified | 25 ADR proof modules (`lean/Foundations/ADR/*.lean`), zero `sorry` |
| **Exact Small-Gain Gate** | ✅ Verified | `spectral.rs` evaluates $\|G\|_1 < 1$ in $\mathbb{Q}$ via `PosRat` (ADR-055) |
| **Background Daemon (`pirtmd`)** | ✅ Implemented | Tokio WebSocket service (`ws://127.0.0.1:8090`) hosting compiler & Sentinel |
| **Interactive Editor (`pirtm-tui`)** | ✅ Implemented | Split-pane Ratatui editor with 14 slash commands & LSP diagnostic feeds |
| **Editor Extensions** | ✅ Implemented | VS Code (`editors/vscode/`) and Neovim (`editors/neovim/`) IPC extensions |
| **Rust Workspace** | ✅ Verified | 24 workspace member crates (`cargo test --workspace` 100% green) |
| **Kani Model Checking** | ✅ Verified | Formal harnesses (`#[cfg(kani)]`) across `adr_rust` proof modules |
| **Poseidon2 ZK Receipts** | ✅ Verified | `Poseidon2Soundness.lean` flag conjunction & receipt verifier (ADR-049) |
| **WardMonitor / Lyapunov** | ✅ Verified | `WardMonitorStability.lean` Lyapunov stability tripwire (ADR-048) |
| **Multi-Node Consensus** | ✅ Verified | Quorum soundness $passVotes \ge quorumThreshold \iff CLUSTER\_PASS$ (ADR-050) |
| **Collaborative CRDT Engine** | ✅ Verified | Vector clock merge convergence & contractivity $\|G\|_1 < 1$ preservation (ADR-056) |

---

## PIRTM TUI & Daemon (`pirtmd`)

Launch the background daemon and interactive TUI development environment:

```bash
# Start background compiler and governance daemon
cargo run --bin pirtmd

# Launch interactive split-pane TUI editor
cargo run --bin pirtm-tui
```

### Governance Slash Commands

- **Governance & Verification**: `/audit`, `/simulate`, `/certify`, `/validate`, `/status`
- **Formal Analysis**: `/explain`, `/proof`, `/refactor`
- **Execution & Tools**: `/benchmark`, `/profile`, `/deploy`, `/compile`, `/ask <question>`, `/clear`, `/quit`

---

## ADR Formal Proof Suite (ADR-034 to ADR-056)

All 23 active Architecture Decision Records are fully accepted, implemented, and verified in Lean 4 and Rust/Kani:

- **ADR-034–ADR-048**: Dialectical admissibility, prime quantum syndrome, phase dissonance bands, WardMonitor Lyapunov stability, CSL constitution gate, and Goldilocks prime field contractivity.
- **ADR-049**: Poseidon2 ZK receipt flag conjunction soundness.
- **ADR-050**: Multi-node Sentinel cluster consensus quorum soundness.
- **ADR-051**: Local PC environment installation protocol soundness.
- **ADR-052**: PETC prime valuation additive homomorphism $v_p(e_1 + e_2) = v_p(e_1) + v_p(e_2)$ and ACE weighted-$\ell_1$ soft-thresholding non-expansiveness.
- **ADR-053**: Universal Multiplicity Constant $\Lambda_m$ fail-closed precedence & PMRO $2\sqrt{N}$ associator defect upper bound.
- **ADR-054**: Prime-indexed NCG-CDT unified action density operator norm bound & spectral dimension proxy bounds ($1.2 \le D_s(t) \le 2.0$).
- **ADR-055**: Elimination of $10^6$ float scaling membranes in favor of exact rational constructor `Ensemble::from_rationals` over reduced `PosRat` in $\mathbb{Q}$.
- **ADR-056**: Collaborative CRDT vector clock state merge convergence and contractivity $\|G\|_1 < 1$ preservation under merged edits.

---

## Repository Layout

```text
PIRTM/
├── README.md                         # Platform overview & grounded status
├── CHANGELOG.md                      # Version history (v1.1.0 release)
├── lakefile.lean / lean-toolchain    # Lean 4 toolchain lock (v4.33.0-rc2)
├── docs/                             # ADR documents, guides & tutorials
│   ├── adr/accepted/                 # Accepted ADRs 034 through 056
│   ├── TUI_USER_GUIDE.md             # TUI keybindings & slash command guide
│   ├── TUTORIAL_GOVERNED_CONTRACTS.md# Developer contract creation tutorial
│   └── RELEASE_NOTES_v1.1.0.md       # v1.1.0 official release notes
├── editors/                          # External editor IPC plugins
│   ├── vscode/                       # VS Code extension manifest & client
│   └── neovim/                       # Neovim lua plugin initializer
├── lean/                             # Canonical Lean 4 proof suite
│   ├── Foundations/ADR/              # 25 ADR formal proof modules
│   └── TestDriver/Main.lean          # Lean test runner executable
├── rust/                             # 28 Cargo workspace member crates
│   ├── pirtm-daemon/                 # pirtmd WebSocket IPC background daemon
│   ├── pirtm-tui/                    # pirtm-tui Ratatui interactive editor
│   ├── pirtm-engine/                 # Spectral gate & execution engine
│   └── adr_rust/                     # Kani formal proof harness suite
└── docker-compose.yml                # Multi-container orchestration stack
```

---

## Build & Verification

### Prerequisites

- Lean 4 `v4.33.0-rc2` via `elan` (must match `lean-toolchain`)
- Rust 1.80+ (`cargo`, `rustc`)

### Execution Commands

```bash
# 1. Run full Lean 4 formal proof test suite (25 modules)
lake test

# 2. Run full Rust Cargo workspace test suite (24 crates)
cd rust && cargo test --workspace

# 3. Launch containerized orchestration stack
docker-compose up -d
```

---

## License & Legal Entity

- **Legal Entity**: **Citizen Gardens UNA** (Wyoming W.S. 17-22)
- **Doing Business As (DBA)**: **The Prime Materia Commons**
- **Official Legal Designation**: **Citizen Gardens UNA d/b/a The Prime Materia Commons**
- **Licensing**: Prime Materia Open Commons License v1.0 and Lawful Recursion License ($\Xi$-License v1.0). See `LICENSE` and `Ξ-LICENSE`.
