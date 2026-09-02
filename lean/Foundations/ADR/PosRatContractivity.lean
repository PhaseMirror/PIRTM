import Foundations.ADR.Core
import Foundations.ADR.Proofs

/-!
# ADR-055: Exact Rational 1-Norm Matrix Contractivity

This module formalizes the exact rational contractivity predicate over non-negative rationals (PosRat).
Matrix entries A_ij and contraction factors λ_j are stored as reduced exact rationals in Q.
The matrix 1-norm ||G||_1 = max_j sum_i (A_ij * λ_j) strictly bounds the spectral radius ρ(G) <= ||G||_1.
-/

namespace Foundations.ADR.PosRatContractivity

/-- Representation of a non-negative rational pair (p, q) with q >= 1 -/
structure PosRat where
  num : Nat
  den : Nat
  den_pos : den > 0
  deriving Repr

/-- Compute 1-norm contractivity column sum for two component coupling -/
def column_sum_2 (a1 : PosRat) (l1 : PosRat) (a2 : PosRat) (l2 : PosRat) : PosRat :=
  let n1 := a1.num * l1.num * a2.den * l2.den + a2.num * l2.num * a1.den * l1.den
  let d1 := a1.den * l1.den * a2.den * l2.den
  have h1 := Nat.mul_pos a1.den_pos l1.den_pos
  have h2 := Nat.mul_pos h1 a2.den_pos
  have h3 := Nat.mul_pos h2 l2.den_pos
  ⟨n1, d1, h3⟩

/-- Predicate confirming strict 1-norm contractivity ||G||_1 < 1 -/
def is_contractive (s : PosRat) : Bool :=
  decide (s.num < s.den)

/-- Soundness theorem: If is_contractive s is true, s.num < s.den -/
theorem posrat_norm_contractive_sound (s : PosRat) (h : is_contractive s = true) :
  s.num < s.den := by
  unfold is_contractive at h
  exact of_decide_eq_true h

end Foundations.ADR.PosRatContractivity
