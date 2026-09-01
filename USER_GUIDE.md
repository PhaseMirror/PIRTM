# PIRTM/MOC User Guide

Welcome to the PIRTM/MOC (Phase Mirror / Multiplicity Object Code) User Guide. This document provides comprehensive instructions for installing, configuring, compiling, and executing PIRTM programs.

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Project Structure](#project-structure)
4. [Compiling PIRTM Programs](#compiling-pirtm-programs)
5. [Running Programs](#running-programs)
6. [Validating Contractivity](#validating-contractivity)
7. [Using the Standard Library](#using-the-standard-library)
8. [Debugging and Troubleshooting](#debugging-and-troubleshooting)
9. [Advanced Usage](#advanced-usage)
10. [Appendix](#appendix)

---

## Overview

PIRTM/MOC is a formally governed, contractive systems programming language. Key concepts:

- **Governed Compilation**: Every program is validated against the Sedona Spine contractivity invariants before execution.
- **MLIR Lowering**: PIRTM source is lowered to MLIR, then to LLVM IR, and finally to native code.
- **Prime Operators**: Kernel-level tensor contractions use prime-indexed operators (`Ap(n)`) that must be certified prime.
- **Zeno Damping**: Runtime drift is monitored and attenuated via the Zeno controller.
- **Fail-Closed**: The `SIG_GOV_KILL` tripwire terminates execution if governance thresholds are breached.

---

## Installation

See [INSTALL.md](INSTALL.md) for detailed platform-specific instructions.

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/PhaseMirror/PiLang.git
cd PiLang

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install elan (Lean version manager)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
source $HOME/.elan/env

# Install pinned Lean toolchain
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version

# Build
cd rust && cargo build --workspace
cd .. && ./build.sh
```

---

## Project Structure

```
PiLang/
├── examples/              # Example PIRTM programs and verified outputs
│   ├── json_parser.pirtm  # Full JSON parser in PIRTM
│   └── json_parser.mlir   # Verified compiler output
├── lean/                  # Lean 4 formal verification core
│   ├── ADR/               # Architecture Decision Record proofs
│   ├── PIRTM.lean         # Kernel definitions (DivLoop, scaling factors)
│   └── prime_tensors/     # Prime tensor library
├── rust/                  # Rust compiler and runtime
│   ├── pirtm-compiler/    # CLI and compilation pipeline
│   ├── pirtm-engine/      # Runtime execution and spectral validation
│   ├── pirtm-monitor/     # WardMonitor drift detection
│   ├── pirtm-parser/      # PIRTM parser
│   ├── pirtm-mlir/        # MLIR lowering
│   └── pirtm-stdlib/      # Standard library
├── docs/                  # ADRs, claim tables, axiom ledger
└── build.sh               # Build entrypoint
```

---

## Compiling PIRTM Programs

### Using the CLI

The `pirtm` compiler is provided by `pirtm-compiler`:

```bash
cd rust

# Compile a PIRTM source file to MLIR
cargo run -p pirtm-compiler --bin pirtm -- compile ../examples/json_parser.pirtm --output ../examples/json_parser.mlir

# Compile with contractivity validation
cargo run -p pirtm-compiler --bin pirtm -- compile ../examples/json_parser.pirtm --output ../examples/output.mlir --validate
```

### Program Structure

A PIRTM program consists of:

```pirtm
// Import standard library modules
use std.io;
use std.collections;

// Define a function
fn parse_json(input: String) -> Result<Value, Error> {
    // ... implementation ...
}

// Main entry point
fn main() -> i32 {
    let data = io.read_file("input.json");
    match parse_json(data) {
        Ok(value) => {
            io.print(value.to_string());
            0
        }
        Err(e) => {
            io.print("Error: " + e.message);
            1
        }
    }
}
```

### Supported Constructs

| Construct | Description | MLIR Lowering |
|---|---|---|
| `let mut x = ...` | Mutable binding | `llvm.alloca` + `llvm.store` |
| `if/else` | Conditional | `scf.if` |
| `while` | Loop | `scf.while` |
| `for` | Bounded loop | `scf.for` |
| `fn` | Function definition | `func.func` |
| `struct` | Struct type | `llvm.struct` |
| `enum` | Enum type | `llvm.ptr` + discriminator |
| `impl` | Method block | `func.func` + method dispatch |
| `match` | Pattern matching | `scf.switch` |
| `tensor` | Kernel tensor contraction | `pirtm.tensor` |
| `Ap(n)` | Prime operator | `pirtm.prime_op` |
| `assert_contractive` | Contractivity assertion | `pirtm.assert_contractive` |

---

## Running Programs

### Dry Run (Simulated)

```bash
cargo run -p pirtm-engine --bin pirtm-engine -- run --mlir examples/json_parser.mlir --dry-run
```

Output:
```
Simulated output for input:
ExecutionReceipt { return_code: 0, stdout: "", stderr: "", ... }
```

### Real Execution

Requires LLVM toolchain (`mlir-translate`, `llc`, `clang`) in PATH:

```bash
cargo run -p pirtm-engine --bin pirtm-engine -- run --mlir examples/json_parser.mlir --input "test.json"
```

Output:
```
ExecutionReceipt {
    return_code: 0,
    stdout: "{...}",
    stderr: "",
    metrics: TelemetryMetrics { rho: 0.0, delta: 1e-6, ... },
    contractivity_hash: "sha256:..."
}
```

### Execution Pipeline

1. **Parse**: PIRTM source → AST
2. **Validate**: `AdmissibilityValidator` rejects invalid constructs
3. **Lower**: AST → MLIR text module
4. **Translate**: MLIR → LLVM IR (`mlir-translate`)
5. **Compile**: LLVM IR → object file (`llc`)
6. **Link**: object → native binary (`clang`)
7. **Execute**: binary with captured stdout/stderr/return code
8. **Telemetry**: Metrics collected from execution artifacts

---

## Validating Contractivity

### Spectral Radius Check

```bash
# Validate an ensemble configuration
cargo run -p pirtm-engine --bin pirtm-engine -- validate-and-certify --ensemble examples/ensemble.json
```

Output:
```
EnsembleContractivityReceipt {
    ensemble_name: "json_pipeline",
    dimension: 3,
    spectral_radius: 0.0000000012,
    is_stable: true,
    hash: "sha256:...",
    ...
}
```

### CLI Flags

| Flag | Description |
|------|-------------|
| `--validate` | Run admissibility validation during compilation |
| `--dry-run` | Use simulated telemetry instead of real execution |
| `--ledger` | Enable SHA-256 proof receipt generation |
| `--input <args>` | Pass arguments to the executed binary |
| `--output <file>` | Write MLIR output to file |

---

## Using the Standard Library

The PIRTM standard library (`pirtm-stdlib`) provides verified primitives:

```pirtm
use std.io;        // File I/O, printing
use std.collections; // Vec, Map, Set
use std.net;       // TCP sockets, HTTP
use std.math;      // Transcendental functions with contractivity proofs
```

### FFI Integration

External functions are declared with `extern`:

```pirtm
extern fn string_len(s: String) -> i32;
extern fn vec_push(v: Vec<i32>, x: i32) -> Vec<i32>;
```

All FFI calls are validated by the `AdmissibilityValidator` and require a contractivity proof or documented exception.

---

## Debugging and Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| `mlir-translate: command not found` | Install LLVM 17+ and ensure `mlir-translate`, `llc`, `clang` are in PATH |
| `sorry` found in proofs | Do not use `sorry`; log proof debt in `docs/PIRTM-axiom-ledger.md` as `AX-*` |
| `FloatLit` rejected | Float literals cannot be used as stability proofs; use kernel contractivity receipts instead |
| `unbounded loop` error | Add explicit bound annotation: `for i in 0..N` or `while x > 0 { ... }` with invariant |
| `prime operator not certified` | Ensure `Ap(n)` uses a certified prime index; `AdmissibilityValidator::validate_prime` checks primality |
| Toolchain drift detected | Run `elan run leanprover/lean4:v4.33.0-rc2 -- lean --version` and ensure it matches `lean-toolchain` |
| Build fails on `ZenoController.lean` | Do not rewrite the file; restore with `git checkout HEAD -- lean/ADR/ZenoController.lean` |

### Inspecting MLIR Output

```bash
# Generate MLIR
cargo run -p pirtm-compiler --bin pirtm -- compile examples/json_parser.pirtm --output /tmp/output.mlir

# Inspect
cat /tmp/output.mlir
```

### Inspecting Proof Receipts

```bash
# Run with ledger enabled
cargo run -p pirtm-engine --bin pirtm-engine -- run --mlir examples/json_parser.mlir --ledger

# Output includes contractivity_hash
```

---

## Advanced Usage

### Custom Admissibility Rules

The `AdmissibilityValidator` can be extended with custom rules. See `rust/pirtm-compiler/src/lib.rs` for the implementation.

### Embedding in Rust Projects

```rust
use pirtm_compiler::PhaseMirrorCompiler;
use pirtm_engine::{Runtime, RuntimeConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile
    let compiler = PhaseMirrorCompiler::new();
    let module = compiler.compile(include_str!("../program.pirtm"))?;

    // Execute
    let mut runtime = Runtime::new(RuntimeConfig::default());
    runtime.load("program.mlir")?;
    let receipt = runtime.run()?;

    println!("Exit code: {}", receipt.return_code);
    Ok(())
}
```

### Extending the Standard Library

Add new verified primitives to `rust/pirtm-stdlib/src/lib.rs`. Each public function must have:
- A Rust implementation
- A corresponding Lean contractivity proof in `lean/ADR/BoundedIteration.lean`
- An entry in `docs/PIRTM-README-Claim-Table.md`

---

## Appendix

### Glossary

| Term | Definition |
|------|------------|
| **ADR** | Architecture Decision Record — a formal document capturing a significant decision |
| **Contractivity** | The property that an operator reduces distance between states ($\|T(x) - T(y)\| < \|x - y\|$) |
| **DivLoop** | A type class axiomatizing division with cancellation (`mul`, `div`, `zero`, `div_cancel`) |
| **Kernel** | The mathematically governed core of PIRTM (tensor contractions, prime operators) |
| **Small-Gain Theorem** | A control-theoretic result: a feedback system is stable if the spectral radius $\rho < 1$ |
| **Spectral Radius** | The largest eigenvalue magnitude of a matrix; must be $< 1.0$ for stability |
| **Zeno Damping** | An attenuation controller that reduces the gain bound $\kappa(t)$ over time |
| **Phase Mirror** | The methodology enforcing on-tree ground truth and zero-tolerance proof standards |

### References

- [Sedona Spine Governance](docs/DEFENSIVE_PUBLICATION_GOVERNANCE_AS_COMPILATION.md)
- [ADR-013: Scope Boundary](docs/ADR-013-PIRTM-MOC-Language-Scope.md)
- [ADR-014: Grammar Authority](docs/ADR-014-Grammar-Authority.md)
- [ADR-015: Reject False Delivery Packs](docs/ADR-015-Reject-False-Delivery-Pack.md)
- [Lean 4 Manual](https://leanprover.github.io/lean4/doc/)
- [MLIR Documentation](https://mlir.llvm.org/docs/)

---

*Last Updated: 2026-09-01*
