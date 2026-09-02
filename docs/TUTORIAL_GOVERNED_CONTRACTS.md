# Developer Tutorial: Governed Smart Contract Development with PIRTM TUI
### Version 1.1.0 · LawfulRecursionVersion 1.0

This tutorial guides you step-by-step through writing, formally verifying, refactoring, and certifying a governed smart contract using the **PIRTM TUI** and **Background Daemon (`pirtmd`)**.

---

## Prerequisites

1. **Rust & Lake/Lean 4 Toolchain**: `cargo` and `lake` installed.
2. **Repository Workspace**: `packages/PIRTM/rust/`.

---

## Step 1: Start the Background Daemon

Open a terminal and start the background daemon service:

```bash
cd packages/PIRTM/rust
cargo run --package pirtm-daemon --bin pirtmd -- serve
```
Output:
```text
🚀 PIRTM Daemon (pirtmd) listening on ws://127.0.0.1:8090
```

---

## Step 2: Launch the Interactive TUI

In a second terminal window, launch `pirtm-tui`:

```bash
cd packages/PIRTM/rust
cargo run --package pirtm-tui --bin pirtm-tui
```

---

## Step 3: Write a Governed Contract

In the Editor Pane, define a PIRTM ensemble contract specifying non-negative exact rational interconnection matrix entries $A_{ij}$ and gain vectors $\lambda_j$:

```pirtm
// PIRTM Governed Contract
import Foundations.ADR.BoundedIteration

ensemble "token_vault" {
  matrix [[(0, 1), (2, 5)], [(2, 5), (0, 1)]]
  lambdas [(4, 5), (4, 5)]
  theorem "Foundations.ADR.BoundedIteration.iterate_non_expansive"

  fn main() -> u64 {
    return 100
  }
}
```

---

## Step 4: Run Formal Governance Slash Commands

Focus the Command Bar by pressing `/` and run the following governance commands:

1. **`/explain`**: Explain Small-Gain 1-norm matrix contractivity:
   $$\|G\|_1 = \max_j \sum_i |A_{ij}| \cdot \lambda_j = \frac{2}{5} \times \frac{4}{5} = \frac{8}{25} = 0.32 < 1.0 \quad (\text{PASS in } \mathbb{Q})$$

2. **`/audit`**: Execute full formal invariant checks (Small-Gain, Zeno Monotonicity, and Fail-Closed boundary limits).

3. **`/simulate`**: Run a 1,000-step Monte Carlo trajectory simulation verifying spectral shift bounds $\Delta_{\max} = 0.021 < 0.030$.

4. **`/proof`**: Generate machine-checkable Lean 4 proof stubs.

5. **`/compile`**: Transpile the program to MLIR and verify exact rational norm bounds.

6. **`/certify`**: Generate Poseidon2 sponge cryptographic `UnifiedWitness` WORM receipts.

---

## Summary

You have successfully developed, analyzed, verified, and certified a contractively sound PIRTM smart contract!
