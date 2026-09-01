# ADR-022: Replace Simulated Runtime with Real LLVM IR Execution

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-09-01

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
