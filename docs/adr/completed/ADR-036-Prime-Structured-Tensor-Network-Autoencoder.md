## 📄 ADR-036: Prime-Structured Tensor-Network Autoencoder (TN-AE)

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The defensive paper *Prime-Structured Tensor-Network Autoencoder: Architecture, Regularization, and Prior Art Defensive Publication* (Citizen Gardens / Prime Materia Commons, May 2026) formalizes a framework for tensor-network autoencoders (MPS/TTN-based) whose bond dimensions and effective entanglement ranks are constrained to a prime-structured lattice of allowed dimensions.

Conventional tensor networks utilize continuous or arbitrary integer bond dimensions without multiplicative structure, leading to unconstrained entanglement scaling, difficulty in interpretability, and lack of alignment with prime-tensor multiplicity models.

---

### Decision

We integrate **Prime-Structured Tensor-Network Autoencoders** into PIRTM/MOC's tensor representation and MLIR lowering engine:

1. **Prime-Structured Bond Lattice**: Constrain allowed bond dimensions $D \in \mathcal{S}_{\mathbb{P}}$ where $\mathcal{S}_{\mathbb{P}}$ is a set of prime-factored integers.
2. **Differentiable Rank Surrogates & Prime Regularization**: Enforce softmin/sigmoid rank surrogates that penalize deviations of effective rank $\hat{\chi}$ from the prime lattice $\mathcal{S}_{\mathbb{P}}$.
3. **Prime-Exponent Vector Penalties**: In strong form, extract and regularize approximate prime-exponent vectors $\mathbf{e} = [e_1, e_2, \dots, e_k]^T$ against target prime factorizations.

---

### Consequences

#### Benefits
- **Multiplicity-Aware Entanglement Compression**: Direct alignment between tensor autoencoder bottlenecks and prime-factor multiplicity modules.
- **Differentiable Multiplicity Model Selection**: Provides continuous loss functions that drive discrete bond dimension optimization.

#### Costs / Risks
- **SVD Gradient Overhead**: Computing singular value decomposition surrogates during backward passes increases autograd memory and execution time.

---

### Links

- [Prime Tensor Network Autoencoder Paper](../Prime%20Tensor%20Network%20Autoencoder.tex)
- [ADR-032: Prime-Recursive-Foundations-of-Existence](./ADR-032-Prime-Recursive-Foundations-of-Existence.md)
- [ADR-035: Prime-Encoded-Quantum-States](./ADR-035-Prime-Encoded-Quantum-States.md)
