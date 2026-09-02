# ADR-059: Phase-Decoupled Subsystem Pipeline

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

Mixing spectral matrix extraction with application body statement parsing creates cross-contamination between governance evaluation and code generation.

## Decision

1. **Subsystem Isolation**:
   - **Phase 1 (Governance Subsystem)**: Parses packaging envelope header text, extracts spectral parameters $(A, \lambda)$, and evaluates small-gain contractivity $\|A\|_1 < 1$ via `Ensemble::from_rationals`.
   - **Phase 2 (Code Generation Subsystem)**: Receives body text after `---`, parses function definitions and application logic via `pirtm-parser` (Pratt parser), and emits MLIR text.
2. **Strict Phase Gate**:
   - If Phase 1 spectral evaluation fails or returns an error, Phase 2 code generation is aborted immediately without building an AST or emitting MLIR.

## Consequences

- Clean architectural decoupling between governance verification and code generation.
- Unlawful code cannot trigger MLIR emission or JIT execution.
