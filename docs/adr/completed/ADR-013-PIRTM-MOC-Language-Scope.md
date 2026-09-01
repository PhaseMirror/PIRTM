# ADR-013: Scope Boundary of the PIRTM/MOC Compute Language

- **Status**: Accepted (Updated)
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-08-31

## Context
The PIRTM/MOC architecture integrates formal mathematical governance (Sedona Spine, contraction mappings, spectral radius bounds) with practical general-purpose compute capabilities. A persistent tension has existed between:
1. Treating user-declared floating-point manifests as proof of Small-Gain stability.
2. Blurring the boundary between the formal governance kernel and general-purpose systems programming constructs.

## Decision
1. **L0 Scope Invariant**: The Small-Gain Theorem gate ($\rho(|A|\,\mathrm{diag}(\lambda)) < 1.0$) cannot be satisfied by scalar float summation of declared manifests. Stream R requires explicit extraction of coupling adjacency $A$ and certified gain vector $\lambda$ grounded in kernel receipts.
2. **Layer Segregation**:
   - **Kernel Substrate**: Tensor contractions, prime operators, contractivity certificates, and `UnifiedWitness` generation.
   - **Application Substrate**: General-purpose syntax (`let mut`, `match`, `impl`, loops, method dispatch) lowered to MLIR.
3. **No Sorry Remediation**: Incomplete formal proofs cannot be bypassed with `sorry` or heuristic float mocks in production code paths. All unproved claims must be logged directly into the Axiom Ledger.

## Consequences
- Float mockups in `linker.rs` are formally deprecated in favor of matrix spectral radius verification.
- General-purpose language features do not alter or dilute the formal contractivity invariants of the mathematical kernel.
