## 📄 ADR-037: Prime-Indexed Phase-Dissonance Functionals for Software Governance

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The defensive publication *Phase Mirror: Prime-Indexed Phase-Dissonance Functionals for Software Governance* (Citizen Gardens / Prime Materia Commons, May 2026) defines a governance control framework built on Prime-Indexed Recursive Transition Machinery (PIRTM).

Traditional software governance systems evaluate policy compliance via binary pass/fail CI/CD gates or static threshold rules, causing rigid deployment blocks and unmanaged semantic drift across versioned artifacts (specifications, source code, execution logs, SLAs).

---

### Decision

We integrate **Prime-Indexed Phase-Dissonance Functionals** into PIRTM as a continuous governance monitoring and control layer:

1. **Prime-Indexed Governance State ($\Phi_t$)**: Represent artifact state across governance axes $\mathcal{P}_g = \{p_1, \dots, p_k\}$ and artifact types $\mathcal{A} = \{\text{spec}, \text{code}, \text{log}, \text{SLA}\}$.
2. **Phase-Dissonance Functional ($D(\Phi_t)$)**: Compute aggregate contradiction as a continuous prime-weighted norm:
   $$D(\Phi_t) = \left( \sum_{p_i \in \mathcal{P}_g} \sum_{a \in \mathcal{A}} (p_i \cdot w_{p_i,a} \cdot \Delta_{p_i,a}(t))^2 \right)^{1/2}$$
3. **Dynamic Phase Band ($[L_t, U_t]$) & Control Loop**: Maintain adaptive upper/lower limits $[L_t, U_t]$; when $D(\Phi_t) > U_t$, Phase Mirror triggers structured remediation actions rather than binary failure drops.

---

### Consequences

#### Benefits
- **Continuous Contradiction Metric**: Aggregates multi-artifact contradictions into a smooth, provably contractive metric.
- **Adaptive Governance Control Loop**: Replaces brittle binary CI gates with dynamic phase-band monitoring and structured remediation.

#### Costs / Risks
- **Weight Calibration**: Proper tuning of prime weights $w_{p_i,a}$ is required to prevent over-sensitivity to minor logging noise.

---

### Links

- [Prime Indexed Phase Dissonance Paper](../Prime%20Indexed%20Phase%20Dissonance.tex)
- [ADR-032: Prime-Recursive-Foundations-of-Existence](./ADR-032-Prime-Recursive-Foundations-of-Existence.md)
- [ADR-034: Prime-Indexed-Dialectical-Semantics](./ADR-034-Prime-Indexed-Dialectical-Semantics.md)
