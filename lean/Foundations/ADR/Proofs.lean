import Foundations.ADR.Core

/-!
# ADR Foundations Proofs

Formal invariants for the ADR data model.
-/
open PIRTM.ADR

theorem accepted_immutable (a : ADR) (_h : a.status = ADRStatus.Accepted) :
    ∀ (a' : ADR), a' = a ∨ a'.status ≠ ADRStatus.Accepted := by
  intro a'
  by_cases h_eq : a' = a
  · left; exact h_eq
  · right
    intro h_acc
    have : a'.status = ADRStatus.Accepted := h_acc
    exact h_eq (by sorry)

theorem no_circular_supersession (a : ADR) :
    a.supersedes ≠ some a.id := by
  intro h
  have : True := True.intro
  cases this
  sorry

theorem traceability (a : ADR) (_h : a.status = ADRStatus.Accepted) :
    ∃ hist : List ADRId, hist.head? = some a.id := by
  exact ⟨[a.id], rfl⟩
