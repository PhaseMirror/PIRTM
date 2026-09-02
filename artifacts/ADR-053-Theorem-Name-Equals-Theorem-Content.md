# ADR-053: Theorem Name Equivalence to Engineering Predicate (Ban Tautology Proofs)

- **Status**: Accepted
- **Deciders**: Formal Methods Steward, Principal Verification Engineer
- **Date**: 2026-09-02

## Context
Inspection of Lean theorem signatures revealed proofs where the conclusion was a syntactic identity or tautology of hypotheses (e.g., `verifyPoseidon2Receipt := isValid ∧ count ≤ 5087` proving `P ∧ Q → P ∧ Q`, or `WardMonitor` proving non-increasing behavior on `Nat` instead of exponential Lyapunov decay $\kappa(t) = \kappa_0 e^{-\alpha t}$). Additionally, transcendental contractivity and linear map Lipschitz definitions relied on unproven `axiom` statements.

## Decision
1. **Ban Tautological Soundness Theorems**: A theorem named `*_soundness` or `*_contractivity` must state and prove the actual mathematical engineering predicate (e.g., non-negative matrix small-gain $\rho(|A|\,\mathrm{diag}(\lambda)) < 1$ or true Lyapunov decay), not a logical tautology of its input fields.
2. **Axiom Audit Mandate**: All `axiom` declarations in `lean/` must be explicit residual proof debts tracked in `alp_sorry_manifest.json` with a governor, deadline, and paired witness. No hidden axioms may exist in clinical or core modules.
3. **CI Tautology Gate**: Lean linter rules will flag and reject any theorem where the goal is trivially solved by `simp` of the hypothesis list without non-trivial algebraic or structural reasoning.

## Consequences
- Tautology theorems are formally demoted and removed from claim registries.
- Structural separation between formal specification and verified proof is enforced.
