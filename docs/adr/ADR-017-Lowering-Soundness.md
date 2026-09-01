# ADR-017: Formal MLIR Lowering Soundness and Metric Preservation

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-08-31

## Context
Having formally admitted bounded iteration (ADR-016) and established the runtime Small-Gain interlock (ADR-013, ADR-014), the compilation pipeline translates high-level PIRTM AST constructs into MLIR dialects (`pirtm`, `scf`, `func`, `llvm`, `arith`).

To ensure that the compilation process itself is mathematically certified and introduces zero semantic or metric drift, the lowering rules across memory allocation, load/store mutation, and dialect operations must be proven contractivity-preserving in Lean 4 Core.

## Decision
1. **Memory Isometry Invariant (`stack_alloca_distance_invariant`)**:
   - Proves in [`lean/ADR/LoweringSoundness.lean`](../lean/ADR/LoweringSoundness.lean) that stack frame allocations (`llvm.alloca`) and store operations (`llvm.store`) into allocated slots preserve metric distance on all observed memory addresses:
     $$\text{dist}(s_{\text{updated}}, s'_{\text{updated}}) \le \text{dist}(s, s')$$
2. **MLIR Lowering Soundness Theorem (`mlir_lowering_preserves_contractivity`)**:
   - Proves that the sequential composition and instruction emission of `OpTransformer` operations into MLIR maintains non-expansion ($L \le 1$) across the compilation pipeline.
3. **`scf.while` Lowering Invariant (`scf_while_contractive`)**:
   - Proves that `scf.while` execution with a finite static bound $N_{\max}$ is contractive when its loop body transformer is non-expansive.

## Consequences
- The compiler itself is now anchored to formal machine-checked lowering proofs in Lean 4 Core.
- Ensures end-to-end mathematical soundness from source AST to MLIR execution under Sedona Spine governance.
- Zero-tolerance CI passes with 100% Mathlib-free and 100% `sorry`-free validation.
