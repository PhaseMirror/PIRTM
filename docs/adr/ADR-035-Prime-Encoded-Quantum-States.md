## 📄 ADR-035: Prime-Encoded Quantum States & Subspace Error Detection

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The papers *Prime-Encoded Quantum States for Number-Theoretic Computation* and *Quantum Encoded Quantum States* (Citizen Gardens / Prime Materia Commons, 2026) define a quantum computing framework leveraging prime-encoded computational basis states $\mathcal{H}_{\mathbb{P}}^{(n)} = \text{span}\{|p\rangle : p \text{ prime}, p < 2^n\}$.

Standard quantum architectures lack domain-specific physical invariants for number-theoretic computations, requiring heavy general-purpose error correction. Prime-encoded quantum state space provides built-in subspace syndrome operators $S_{\mathbb{P}} = 2\Pi_{\mathbb{P}} - I$ that act as physical and algorithmic error-detection primitives.

---

### Decision

We integrate **Prime-Encoded Quantum States** as a recognized execution target and formal model within PIRTM's prime-tensor substrate:

1. **Prime Subspace Projection Operator**: Incorporate $\Pi_{\mathbb{P}}$ as a primitive projection inside quantum MLIR lowering and Lean core verification modules.
2. **Subspace Error Detection**: Use prime-subspace syndrome measurements $S_{\mathbb{P}}$ for error mitigation and post-selection, achieving verified state fidelity improvements.
3. **Restricted Grover Factorization Primitive**: Formalize direct amplitude amplification over prime candidate subspaces for semiprime factorization.

---

### Consequences

#### Benefits
- **Domain-Specific Error Mitigation**: Subspace syndrome detection enables early rejection of corrupted quantum states without full surface code overhead.
- **Unified Number-Theoretic Quantum Pipeline**: Bridges classical prime-tensor recursion in Lean/Rust with quantum circuit compilation targets (Qiskit / OpenQASM).

#### Costs / Risks
- **Subspace Dimension Scaling**: Constructing exact prime projection operators for large qubit count $n$ requires sparse QSVT approximations.

---

### Links

- [Quantum Encoded Quantum States Paper](../Quantum%20Encoded%20Quantum%20States.tex)
- [Quantum State Number Theoretics Paper](../Quantum%20State%20Number%20Theoretics.tex)
- [ADR-032: Prime-Recursive-Foundations-of-Existence](./ADR-032-Prime-Recursive-Foundations-of-Existence.md)
