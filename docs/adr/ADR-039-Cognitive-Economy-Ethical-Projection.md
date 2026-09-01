## 📄 ADR-039: Phase Mirror Cognitive Economy & Ethical Projection Substrate

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The defensive publication *Phase Mirror Governance Manifold, Cognitive Economy, Cryptographic Substrate, Topological Governance, and Agent Admission* (Citizen Gardens / Prime Materia Commons, May 2026) specifies an integrated, invariant-bearing governance substrate for cognitive architectures.

Ex-post policy reviews and advisory governance allow unprojected, unlawful states to exist during intermediate execution steps, causing untracked model evolution, unsafe state mutation, and non-reproducible cognitive retrieval.

---

### Decision

We integrate the **Phase Mirror Cognitive Economy & Ethical Projection Substrate** into PIRTM as a mandatory execution layer:

1. **Immutable Idea Snapshots & Novelty Function**: Enforce immutable idea nodes $e_i$ evaluated against Euclidean separation threshold $\nu(i,j) = \|e_i - e_j\|_2 > \tau_v$.
2. **Idempotent Ethical Projection ($\Pi_E$)**: Replace soft penalties with a proximal projection operator $\Pi_E$ mapping candidate state $x$ to lawful state $\hat{x} = \Pi_E(x)$ satisfying:
   - Lawful preservation: $x \in E \Rightarrow \Pi_E(x) = x$
   - Idempotence: $\Pi_E(\Pi_E(x)) = \Pi_E(x)$
3. **Cryptographic State Attestation ($S(t)$)**: Anchor path-dependent execution traces via hash chain $S(t) = H(S(t-1) \parallel p_i \parallel |A_p T(t)| \parallel M(t))$. If multiplicity norm $|A_p T(t)|_{\text{mult}} > \Xi_{\max}$, trigger immediate fail-closed `L0_HALT`.

---

### Consequences

#### Benefits
- **Idempotent State Safety**: Intermediate computational states are guaranteed lawful by construction through proximal projection.
- **Cryptographic Traceability**: Path-dependent hash chaining binds execution history to prime-indexed multiplicity bounds.

#### Costs / Risks
- **Projection Computation**: Real-time manifold projection $\Pi_E$ adds computational overhead to every transition step.

---

### Links

- [Cognitive Economy Paper](../PM-Cognitive%20Economy.tex)
- [ADR-032: Prime-Recursive-Foundations-of-Existence](./ADR-032-Prime-Recursive-Foundations-of-Existence.md)
- [ADR-038: Governance-Manifold-Fail-Closed-Control](./ADR-038-Governance-Manifold-Fail-Closed-Control.md)
