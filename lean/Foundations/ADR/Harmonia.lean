/-!
# Harmonia (Φπε) Interface Formalization

Canonical path: `lean/Foundations/ADR/Harmonia.lean`.
Moved from deprecated `lean/ADR/Harmonia.lean`.
-/

namespace Harmonia

inductive Symbol where
  | Phi
  | Pi
  | Epsilon
  deriving Repr, DecidableEq, Inhabited

def primeOfSymbol : Symbol → Nat
  | Symbol.Phi => 2
  | Symbol.Pi => 3
  | Symbol.Epsilon => 5

def isSmallPrime : Nat → Bool
  | 2 => true
  | 3 => true
  | 5 => true
  | 7 => true
  | 11 => true
  | 13 => true
  | _ => false

theorem prime_phi_valid : isSmallPrime (primeOfSymbol Symbol.Phi) = true := by
  rfl

theorem prime_pi_valid : isSmallPrime (primeOfSymbol Symbol.Pi) = true := by
  rfl

theorem prime_epsilon_valid : isSmallPrime (primeOfSymbol Symbol.Epsilon) = true := by
  rfl

structure State where
  k_phi : Nat
  k_pi  : Nat
  k_eps : Nat
  deriving Repr, DecidableEq, Inhabited

def multiplicityNumber (s : State) : Nat :=
  (2 ^ s.k_phi) * (3 ^ s.k_pi) * (5 ^ s.k_eps)

def StateUpdate := State → State

def IsSubdivision (u : StateUpdate) (s : State) : Prop :=
  multiplicityNumber (u s) ≤ multiplicityNumber s

theorem subdivision_preserves_envelope (u : StateUpdate) (s : State) (h : IsSubdivision u s) :
    multiplicityNumber (u s) ≤ multiplicityNumber s := h

end Harmonia
