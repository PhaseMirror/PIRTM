# ADR-052: PIRTM ACE × PETC Safety and Lawfulness Protocol

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

PIRTM state transitions require both structural lawfulness (unambiguous retrospection and prime exponent conservation) and certified stability (strict contraction bounds). Prime-Encoded Tensor Calculus (PETC) provides prime-signature exponent ledgers $e \in \mathbb{Z}^{(\mathbb{P})}$ and multiplicity maps $M(e) = \prod p^{e_p}$. The Arithmetic Control Engine (ACE) enforces a strict weighted-$\ell_1$ budget $\sum_p b_p |w_p| \le \tau < 1$ via exact 1-Lipschitz bisection projection.

## Decision

1. **PETC Prime-Signature Ledger**:
   - Maintain prime signatures $e = (e_p)_{p \in \mathbb{P}}$ as sparse prime-exponent maps.
   - Enforce exact conservation $e(Y) = \sum_{i=1}^r e(X_i)$ for all typed tensor operations, ensuring zero integer factorization during audits via valuation $e_p = v_p(M(e))$.

2. **ACE Budget & Bisection Projection**:
   - Enforce the budget $\sum_{p \in P_N} b_p |w_p| \le \tau < 1-\varepsilon$.
   - Apply 1-Lipschitz soft-thresholding projection $w_p^\star = \text{sign}(w_p) \frac{\max(b_p |w_p| - \theta, 0)}{b_p}$ to guarantee linear convergence to unique fixed point $T_\infty = (I - \mathcal{K})^{-1} F$.

3. **Formal Verification & Reference Implementation**:
   - Machine-check `ace_petc_budget_soundness` and `petc_conservation_soundness` in Lean 4 (`lean/Foundations/ADR/AcePetcIntegration.lean`).
   - Implement `PrimeLedger` and `project_weighted_l1` in `adr_rust::ace_petc_proof`.

## Consequences

- Exact, auditable prime provenance for state iterations with zero factorization cost.
- Guaranteed operator contraction norm $\|K\| \le \tau < 1$ under all parameter variations.
- Fully synchronized across Lean 4, Rust workspace, and `registry.json`.
