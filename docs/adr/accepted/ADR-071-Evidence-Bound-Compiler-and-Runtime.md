# ADR-071: Evidence-Bound Compiler and Runtime

- **Status**: Accepted
- **Date**: 2026-09-24
- **Author**: Phase Mirror Audit
- **Decider**: PIRTM Architectural Review Board
- **Supersedes**: the real-execution and exact-gate completion claims in ADR-021, ADR-022, and ADR-055 where current-tree evidence conflicts

## Context

The compiler CLI can request Lean proof verification but continues with a `"fallback"` receipt when the proof file or Lean executable is unavailable (`rust/pirtm-compiler/src/main.rs:245-256`). The runtime can execute without an ensemble (`rust/pirtm-compiler/src/main.rs:330-348`), and the MCP run tool executes an artifact without a spectral receipt (`rust/pirtm-mcp/src/tools.rs:245-308`). The compiler also contains duplicate admissibility validators, a floating-point margin on the exact gate, a reserved `author_declared_lambda` escape hatch, floating-point prime square roots, and production mock/stub paths.

The hidden assumption is that a receipt-shaped string is evidence. It is not. A receipt must be bound to validated source, AST, MLIR, theorem identity, and execution output, and every real execution path must fail closed when that evidence is absent.

## Decision

1. `--lean-proof` is fail-closed. Missing proofs, missing Lean binaries, failed Lean runs, and fallback hashes are errors; no production path may substitute `"fallback"` or `"no-proof"` for a required proof.
2. Real `Runtime::run` execution requires a validated ensemble receipt. `--dry-run` is the only path allowed to use simulated telemetry or omit execution evidence.
3. The canonical small-gain API uses exact `PosRat` values and has no floating-point margin. Floating-point functions may remain only as explicitly non-governing diagnostic or compatibility surfaces and may never satisfy an L0 claim.
4. `author_declared_lambda` is a reserved invalid theorem anchor. A theorem anchor must be non-empty, non-reserved, and present in the Lake declaration registry before certification.
5. Receipts hash the validated AST/source, MLIR artifact, exact rational gate result, theorem declaration, and real execution result. Metrics alone are insufficient.
6. Prime validation uses an integer-only square-root bound. Duplicate validator implementations are removed in favor of the compiler library validator.
7. Production mock closures and dummy success paths are quarantined behind test-only modules or return an explicit unavailable error.

## Consequences

- A successful compile without a proof is not a governed compile.
- A successful process exit without a spectral receipt is not a governed execution.
- Exact rational contractivity is separated from floating-point numerical diagnostics.
- The compiler and runtime have one authoritative validation implementation.
- Existing paths that do not yet meet this contract remain Partial or Broken in the claim table until repaired.

## Validation

- `pirtm compile --lean-proof` exits nonzero when proof verification is unavailable.
- `pirtm run` and `pirtm_run` reject real execution without a validated ensemble.
- `validate_and_certify` accepts only exact rational inputs and non-reserved theorem anchors.
- Receipt hashes change when source, MLIR, theorem identity, or execution output changes.
- Production mock and fallback scans are empty.
