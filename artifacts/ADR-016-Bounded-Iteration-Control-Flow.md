# ADR-016: Formal Contractivity Invariants for Bounded Iteration and Control Flow

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-08-31

## Context
Phase A of the PIRTM/MOC language roadmap introduces structured control flow (`if/else`, `while`, `for`, `fn`, `struct`, `enum`). Under the Sedona Spine zero-tolerance governance mandate, no control-flow construct may be admitted into the compiler pipeline unless its metric contractivity properties are formally verified in the Lean 4 core.

Unbounded loops or unverified branch selections could potentially violate the global contraction factor ($\lambda_{\text{eff}} < 1.0$), triggering runtime interlocks (`SIG_GOV_KILL`).

## Decision
1. **Bounded Iteration Theorem (`iterate_non_expansive`)**:
   - Every loop construct (`while`, `for`) is modeled as iterated composition $f^N(x)$ over a finite iteration bound $N \le N_{\max}$.
   - We prove formally in [`lean/ADR/BoundedIteration.lean`](../lean/ADR/BoundedIteration.lean) that if the loop body $f$ is non-expansive ($\text{dist}(f(x), f(y)) \le \text{dist}(x, y)$), then the $N$-fold iteration $f^N$ is strictly non-expansive for any finite $N$.
2. **Conditional Selection Theorem (`conditional_branch_safe`)**:
   - A conditional branch between two contractive/bounded branches $f_{\text{then}}$ and $f_{\text{else}}$ preserves the outer radius envelope $\max(b_1, b_2) \le R_{\text{basin}}$.
3. **Function Composition Theorem (`compose_non_expansive`)**:
   - Function application and composition preserve metric bounds ($\text{dist}(f(g(x)), f(g(y))) \le \text{dist}(x, y)$).
4. **Lowering Guard**:
   - Loops lowered to MLIR `scf.while` or `scf.for` must verify bounded termination criteria.

## Consequences
- Control-flow constructs (`if/else`, `while`, `fn`) are formally admitted into the language syntax with machine-checked mathematical proofs.
- Compiler guarantees that structured control flow cannot induce unmitigated divergence or breach the Zeno envelope.
