# ADR-056: Collaborative CRDT Integration & Governance Preservation

- **Status**: Proposed
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

To support real-time multi-developer editing of PIRTM contracts without central lock contention, the PIRTM development environment integrates a state-based Conflict-Free Replicated Data Type (CRDT) engine into `pirtmd` and `pirtm-tui`. Edits from concurrent sessions must converge deterministically while preserving the exact rational small-gain contractivity gate $\|G\|_1 < 1$ across all merged document states.

## Decision

1. **State-Based CRDT Engine**:
   - Represent document state via causal vector clocks $c \in \mathbb{N}^k$ and rational contractivity parameters $(num, den) \in \mathbb{Q}_{>0}$.
   - Define state-based merge operator $\text{merge}(s_1, s_2)$ taking elementwise maximum vector clock entries and exact maximum rational norm ratio over common denominator $s_1.den \cdot s_2.den$.

2. **Formal Verification Obligations**:
   - Prove state merge commutativity $\text{merge}(s_1, s_2) = \text{merge}(s_2, s_1)$ in Lean 4 (`Foundations.ADR.CollaborativeCRDT.crdt_convergence_sound`).
   - Prove governance preservation (`crdt_governance_preserved`): if $s_1$ and $s_2$ are contractive ($\|G^{(1)}\|_1 < 1$ and $\|G^{(2)}\|_1 < 1$), then $\text{merge}(s_1, s_2)$ is contractive ($\|\text{merge}(s_1, s_2)\|_1 < 1$).
   - Formally verify Rust CRDT merge operator using Kani bounded model checking (`adr_rust::crdt_proof`).

3. **Daemon & Editor Integration**:
   - Extend `pirtmd` JSON-RPC WebSocket protocol with `crdt_sync` and `crdt_delta` message frames.

## Consequences

- Deterministic convergence of concurrent edits across multi-developer sessions.
- Machine-checked guarantee that merged edits cannot violate the spectral contractivity gate.
