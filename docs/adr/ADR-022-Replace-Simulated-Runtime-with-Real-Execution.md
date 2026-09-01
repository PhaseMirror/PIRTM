# ADR-022: Replace Simulated Runtime with Real LLVM IR Execution

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **Real execution path implemented** in `rust/pirtm-engine/src/lib.rs`:
   - `mlir-translate --mlir-to-llvmir` converts `.mlir` to LLVM IR
   - `llc -filetype=obj` compiles to object file
   - `clang` links to native binary
   - Binary is executed with captured stdout, stderr, and return code
 2. **Simulation retained only for `--dry-run`** — `simulate_telemetry_collection` is used exclusively in the `dry_run` branch. Real execution uses `collect_execution_metrics` derived from actual process I/O and return code.
3. **`ExecutionReceipt` contains genuine process metrics** — `return_code`, `stdout`, and `stderr` come from actual process execution.

## Validation

```rust
// rust/pirtm-engine/src/lib.rs:72-151
pub fn run(&mut self) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
    // ... dry_run early return with simulated metrics ...
    let mlir_status = Command::new("mlir-translate")
        .arg("--mlir-to-llvmir")
        .arg(mlir_path)
        .arg("-o")
        .arg(&ll_path)
        .status()?;
    // ... llc, clang, execute ...
}
```

## Context

`rust/pirtm-engine/src/lib.rs` implements `Runtime::run` as a telemetry simulator:

```rust
pub fn run(&mut self) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
    let metrics = telemetry::simulate_telemetry_collection();
    let mut stdout_buf = String::new();
    if !self.config.input_args.is_empty() {
        stdout_buf.push_str(&format!("Simulated output for input: {}\n", ...));
    }
    ...
}
```

The README architecture diagram labels this subsystem "Governed Runtime & JIT" and the claim table lists "End-to-End JSON Parser Execution" as "✅ Complete."

## Hidden Assumption

That `simulate_telemetry_collection()` and string-formatted "simulated output" constitute a governed runtime. In reality, no LLVM IR is compiled, linked, or executed.

## Decision

1. **Replace `run`** with a real execution path:
   - Invoke `mlir-translate` to convert `.mlir` to LLVM IR.
   - Invoke `llc` and `clang` (or `lld`) to produce a native binary.
   - Execute the binary and capture stdout/stderr/return code.
2. **Retain `simulate_telemetry_collection`** only as a `--dry-run` flag explicitly labeled as simulation.
3. **Update the claim table** to mark "Governed Runtime & JIT" as "In Progress" until the real execution path is on-tree and tested.

## Consequences

- The runtime executes actual compiled code, not simulated strings.
- `ExecutionReceipt` contains genuine process metrics.
- The claim table reflects physical reality per ADR-015.
