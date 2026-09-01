# ADR-028: Repair Reconstructible Definition Scope Error

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`Reconstructible` is now an `inductive`** in `lean/ADR/Proofs.lean` (line 150):
   ```lean
   inductive Reconstructible (lookup : ADRId → Option ADR) : ADR → Prop where
     | root (a : ADR) : a.supersedes = none → Reconstructible lookup a
     | step (a : ADR) {targetId : ADRId} {target : ADR} :
         a.supersedes = some targetId →
         lookup targetId = some target →
         Reconstructible lookup target →
         Reconstructible lookup a
   ```
2. **No scope error** — The inductive definition is well-typed and does not reference `hFuelPos` or `getLast` at all.
3. **Traceability proofs discharge without `sorry`** — `accepted_without_supersession_reconstructible` and `accepted_with_supersession_reconstructible` use the inductive constructors directly.

## Validation

```bash
$ lake build ADR.Proofs
Build completed successfully.
```

## Context

`lean/ADR/Proofs.lean` defines:

```lean
def Reconstructible (lookup : ADRId → Option ADR) (a : ADR) : Prop :=
  ∃ fuel,
    let chain := followSupersession lookup a (fuel.succ)
    chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast (by simp)) = none)
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
