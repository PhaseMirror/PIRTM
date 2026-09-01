/-!
# Prime Tensors: Stability

Formal specification of stability conditions for prime-indexed
transition operators under the Sedona Spine governance substrate.

Self-contained in Lean 4 Core (Mathlib-Free).
-/

namespace prime_tensors.Stability

/-! ## Spectral Stability -/

/-- A transition operator is spectrally stable if its repeated application
    does not increase the cardinality of the prime index set. -/
def SpectralStable (op : List Nat → List Nat) (n : Nat) : Prop :=
  ∀ s, (iterate op n s).length ≤ s.length

/-- Stability is preserved under composition with length-preserving maps. -/
theorem stable_under_composition
    (op1 op2 : List Nat → List Nat)
    (h1 : ∀ s, (op1 s).length ≤ s.length)
    (h2 : ∀ s, (op2 s).length ≤ s.length)
    (n m : Nat) :
    SpectralStable (op1 ∘ op2) (n + m) := by
  unfold SpectralStable
  intro s
  have h_comp : (iterate (op1 ∘ op2) (n + m) s).length ≤ s.length := by
    induction n, m with
    | zero, zero =>
      simp [iterate]
    | zero, succ m ih =>
      simp [iterate, Function.comp]
      have h2m := h2 s
      have ihm := ih (op2 s)
      exact Nat.le_trans ihm h2m
    | succ n, zero =>
      simp [iterate, Function.comp]
      have h1n := h1 s
      have ihn := ih (op1 s)
      exact Nat.le_trans ihn h1n
    | succ n, succ m ih =>
      simp [iterate, Function.comp]
      have h1 := h1 (iterate (op1 ∘ op2) (n + m) s)
      have h_ih := ih s
      exact Nat.le_trans h1 h_ih
  exact h_comp

end prime_tensors.Stability
