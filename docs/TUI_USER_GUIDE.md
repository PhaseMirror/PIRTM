# PIRTM Interactive TUI & Governed Environment User Guide
### Version 1.1.0 · LawfulRecursionVersion 1.0

The **PIRTM Interactive Development Environment** brings the power of the governed compiler, Sentinel gate, WardMonitor, and MCP AI assistant directly into a keyboard-first terminal user interface (TUI), modeled after Kilo and OpenCode.

---

## 1. System Architecture

```text
+-------------------+      WebSocket IPC        +---------------------+
|                   | <-----------------------> |                     |
|   pirtm-tui       |    (ws://127.0.0.1:8090)  |   pirtmd (Daemon)   |
|   (Ratatui Client)|                           |   (Compiler/MCP)    |
+-------------------+                           +---------------------+
                                                           |
                                                           v
                                                 +---------------------+
                                                 | PIRTM Engine & LSP  |
                                                 | PosRat Small-Gain   |
                                                 +---------------------+
```

---

## 2. Launch Instructions

1. **Start the Background Daemon (`pirtmd`)**:
   ```bash
   cargo run --package pirtm-daemon --bin pirtmd -- serve
   ```

2. **Launch the Interactive TUI (`pirtm-tui`)**:
   ```bash
   cargo run --package pirtm-tui --bin pirtm-tui
   ```

---

## 3. Keyboard Navigation & Keybindings

| Key | Action |
|---|---|
| `Tab` | Cycle focus between panes (Explorer $\rightarrow$ Editor $\rightarrow$ Terminal $\rightarrow$ Command REPL) |
| `/` | Immediately focus the Command REPL and insert `/` for slash commands |
| `Up` / `Down` | Navigate project file list in the Explorer pane |
| `Enter` | Submit slash command or select active file |
| `Esc` / `/quit` | Exit the TUI application cleanly |

---

## 4. Interactive Slash Commands

- `/explain` — Analyzes the active PIRTM file and prints a step-by-step mathematical breakdown of the Small-Gain matrix 1-norm dominance $\|G\|_1 = \max_j \sum_i |A_{ij}| \cdot \lambda_j < 1.0$ over exact rationals $\mathbb{Q}$.
- `/proof` — Generates a machine-checkable Lean 4 theorem stub anchoring the ensemble's contractivity in `Foundations.ADR.PosRatContractivity`.
- `/refactor` — Calculates optimal component gains $\lambda_j$ to maximize mathematical safety margin without violating contractivity.
- `/compile` — Transpiles PIRTM source code to MLIR, verifies exact rational 1-norm contractivity, and emits a Poseidon2-sealed WORM audit receipt.
- `/validate` — Invokes the Sentinel gate to evaluate manifold drift against the hard limit $\rho < 1.05$.
- `/status` — Displays real-time WardMonitor health metrics, daemon IPC state, and the sovereign entity name (**Citizen Gardens UNA d/b/a The Prime Materia Commons**).
- `/ask <question>` — Queries the MCP AI governance agent for real-time compliance and formal verification guidance.
- `/clear` — Clears the integrated terminal log output.
- `/quit` — Exits the application.

---

## 5. LSP Diagnostics & Syntax Highlighting

- **Syntax Highlighting**: Keywords (`ensemble`, `matrix`, `lambdas`, `theorem`, `fn`, `return`), types (`u64`, `PosRat`), string literals, numbers, and comments are highlighted in distinct cyan, yellow, green, and magenta themes.
- **LSP Diagnostics Pane**: Real-time diagnostic overlays from `pirtm-lsp` display theorem anchor verification state and rational matrix reduction hints.

---

## 6. External Editor Integration (VS Code & Neovim Blueprint)

External editors connect to `pirtmd` via WebSocket or stdio JSON-RPC (`ws://127.0.0.1:8090`).

### Sample WebSocket JSON-RPC Payload:
```json
{
  "id": 1,
  "method": "compile",
  "params": {
    "name": "calculator",
    "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive",
    "source": "ensemble \"calculator\" { ... }"
  }
}
```

### Response Frame:
```json
{
  "id": 1,
  "result": {
    "status": "COMPILED",
    "mlir": "// MLIR generated for calculator\nmodule { ... }",
    "receipt": {
      "exact_rational_norm_1": "9/25",
      "is_norm_contractive": true,
      "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
    }
  },
  "error": null
}
```
