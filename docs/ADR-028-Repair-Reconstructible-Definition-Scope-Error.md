# ADR-028: Repair Reconstructible Definition Scope Error

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

`lean/ADR/Proofs.lean` defines:

```lean
def Reconstructible (lookup : ADRId → Option ADR) (a : ADR) : Prop :=
  ∃ fuel, fuel > 0 ∧
  let chain := followSupersession lookup a fuel
  chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast (by simp [hFuelPos])) = none)
```

The term `hFuelPos` is not in scope. `fuel > 0` is bound in the existential, but `hFuelPos` is never introduced as a named hypothesis. This makes `Reconstructible` ill-formed.

## Decision

1. **Introduce `hFuelPos`** as a named binder:
   ```lean
   def Reconstructible (lookup : ADRId → Option ADR) (a : ADR) : Prop :=
     ∃ fuel, let hFuelPos : fuel > 0 := by assumption; ∃ _, True ∧
     let chain := followSupersession lookup a fuel
     chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast hFuelPos) = none)
   ```
   Or, more cleanly, pattern-match on `fuel` to expose the positivity witness:
   ```lean
   def Reconstructible (lookup : ADRId → Option ADR) (a : ADR) : Prop :=
     ∃ fuel, fuel > 0 ∧
     let chain := followSupersession lookup a fuel
     chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast (by simpa [hFuelPos])) = none)
   ```
   where `hFuelPos` is the second conjunct of the existential.
2. **Simplify by avoiding `getLast`**: Replace with `chain.head?` or explicit pattern matching on the non-empty chain.

## Consequences

- `Reconstructible` is well-typed and usable in theorems.
- Traceability proofs (`accepted_without_supersession_reconstructible`, `accepted_with_supersession_reconstructible`) can be discharged without `sorry`.
