import Foundations.ADR.Core
import Foundations.ADR.Proofs

/-!
# ADR-055: Exact rational 1-norm column sums

Defines PosRat, entry product, list column sum, and ||G||_1 as the max of
precomputed column sums in Q over exact integer tuples (p, q).
Machine-checked proof suite for exact rational contractivity gate.
-/

namespace Foundations.ADR.PosRatContractivity

/-- Positive exact rational representation (p, q) with q > 0. -/
structure PosRat where
  num : Nat
  den : Nat
  den_pos : den > 0
  deriving Repr

/-- Multiplication of exact rationals (a_n * b_n) / (a_d * b_d). -/
def mul (a b : PosRat) : PosRat :=
  ⟨a.num * b.num, a.den * b.den, Nat.mul_pos a.den_pos b.den_pos⟩

/-- Addition of exact rationals (a_n * b_d + b_n * a_d) / (a_d * b_d). -/
def add (a b : PosRat) : PosRat :=
  ⟨a.num * b.den + b.num * a.den, a.den * b.den, Nat.mul_pos a.den_pos b.den_pos⟩

/-- Exact zero element 0/1. -/
def zero : PosRat :=
  ⟨0, 1, Nat.succ_pos 0⟩

/-- Column sum of exact rational entries. -/
def col_sum : List PosRat → PosRat
  | [] => zero
  | x :: xs => add x (col_sum xs)

/-- Column gain evaluation: sum_i (A_ij * lambda_j). -/
def g_column (Acol : List PosRat) (lam : PosRat) : PosRat :=
  col_sum (Acol.map (fun a => mul a lam))

/-- Maximum rational element over a list in Q. -/
def max_q : List PosRat → PosRat
  | [] => zero
  | x :: xs =>
    let m := max_q xs
    if x.num * m.den ≥ m.num * x.den then x else m

/-- ||G||_1 from already-formed column sums in Q. -/
def norm1 (column_sums : List PosRat) : PosRat :=
  max_q column_sums

/-- Boolean predicate for exact contractivity norm1 < 1. -/
def is_contractive (s : PosRat) : Bool :=
  decide (s.num < s.den)

/-- **Theorem (ADR-055-IFF): Exact Rational Contractivity Soundness**

    `is_contractive s = true ↔ s.num < s.den`.

    Machine-checked in Lean 4 core with zero Mathlib axioms. -/
theorem is_contractive_iff (s : PosRat) :
    is_contractive s = true ↔ s.num < s.den := by
  dsimp [is_contractive]
  exact decide_eq_true_iff

/-- **Theorem (ADR-055-MUL-ZERO-L): Zero Multiplication Left Identity** -/
theorem mul_left_zero (a : PosRat) :
    (mul zero a).num = 0 := Nat.zero_mul a.num

/-- **Theorem (ADR-055-MUL-ZERO-R): Zero Multiplication Right Identity** -/
theorem mul_right_zero (a : PosRat) :
    (mul a zero).num = 0 := Nat.mul_zero a.num

/-- **Theorem (ADR-055-ADD-COMM): Addition Value Commutativity in Q** -/
theorem add_comm_val (a b : PosRat) :
    (add a b).num * (add b a).den = (add b a).num * (add a b).den := by
  dsimp [add]
  have h1 : a.num * b.den + b.num * a.den = b.num * a.den + a.num * b.den := Nat.add_comm (a.num * b.den) (b.num * a.den)
  have h2 : b.den * a.den = a.den * b.den := Nat.mul_comm b.den a.den
  rw [h1, h2]

end Foundations.ADR.PosRatContractivity
