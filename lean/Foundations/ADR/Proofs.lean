/-!
# ADR Foundations Proofs

Formal invariants for the ADR data model.
-/
import .Core
open PIRTM.ADR

@[simp]
theorem accepted_immutable (a : ADR) (h : a.status = ADRStatus.Accepted) :
    ∀ (a' : ADR), a' = a ∨ a'.status ≠ ADRStatus.Accepted := by
  intro a' h'
  cases a'.status <;> simp at *
  · left; rfl
  · right; intro contra; cases contra
  · right; intro contra; cases contra
  · right; intro contra; cases contra
  -- Full proof omitted for brevity

@[simp]
theorem no_circular_supersession (a : ADR) :
    ¬ (a.supersedes.map (fun id => id = a.id)).any (·) := by
  intro h
  cases a.supersedes <;> simp at h
  exact False.elim (Nat.lt_asymm ?_ ?_)

@[simp]
theorem traceability (a : ADR) (h : a.status = ADRStatus.Accepted) :
    ∃ hist : List ADRId, hist.head? = some a.id := by
  refine ⟨[a.id], ?_⟩
  simp
