import Foundations.ADR.Core
import Foundations.ADR.Proofs

/-!
# ADR-055: Exact rational 1-norm column sums

Defines PosRat, entry product, list column sum, and ||G||_1 as the max of
precomputed column sums. Does not prove ρ(G) ≤ ||G||_1 in Lean; that inequality
is classical analysis recorded in the ADR prose.
-/

namespace Foundations.ADR.PosRatContractivity

structure PosRat where
  num : Nat
  den : Nat
  den_pos : den > 0
  deriving Repr

def mul (a b : PosRat) : PosRat :=
  ⟨a.num * b.num, a.den * b.den, Nat.mul_pos a.den_pos b.den_pos⟩

def add (a b : PosRat) : PosRat :=
  ⟨a.num * b.den + b.num * a.den, a.den * b.den, Nat.mul_pos a.den_pos b.den_pos⟩

def zero : PosRat :=
  ⟨0, 1, Nat.succ_pos 0⟩

def col_sum : List PosRat → PosRat
  | [] => zero
  | x :: xs => add x (col_sum xs)

def g_column (Acol : List PosRat) (lam : PosRat) : PosRat :=
  col_sum (Acol.map (fun a => mul a lam))

def max_q : List PosRat → PosRat
  | [] => zero
  | x :: xs =>
    let m := max_q xs
    if x.num * m.den ≥ m.num * x.den then x else m

/-- ||G||_1 from already-formed column sums in Q. -/
def norm1 (column_sums : List PosRat) : PosRat :=
  max_q column_sums

def is_contractive (s : PosRat) : Bool :=
  decide (s.num < s.den)

theorem is_contractive_iff (s : PosRat) :
    is_contractive s = true ↔ s.num < s.den := by
  simp [is_contractive]

theorem mul_left_zero (a : PosRat) :
    (mul zero a).num = 0 := by
  simp [mul, zero]

theorem mul_right_zero (a : PosRat) :
    (mul a zero).num = 0 := by
  simp [mul, zero]

end Foundations.ADR.PosRatContractivity
