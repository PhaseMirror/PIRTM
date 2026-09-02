/-!
# WardMonitor & Zeno Controller Formalization

Canonical path: `lean/Foundations/ADR/ZenoController.lean`.
Moved from deprecated `lean/ADR/ZenoController.lean`.
-/

namespace ZenoController

inductive WardState where
  | Green
  | Amber
  | Red
  | Kill
deriving Repr, DecidableEq

def RHO_WARN : Nat := 85
def RHO_HALT : Nat := 100
def KILL_THRESHOLD : Nat := 105
def DELTA_MAX : Nat := 3

theorem threshold_ordering :
    RHO_WARN < RHO_HALT ∧ RHO_HALT < KILL_THRESHOLD := by
  constructor <;> decide

def classifyState (rho_scaled : Nat) : WardState :=
  if rho_scaled ≥ KILL_THRESHOLD then
    WardState.Kill
  else if rho_scaled ≥ RHO_HALT then
    WardState.Red
  else if rho_scaled ≥ RHO_WARN then
    WardState.Amber
  else
    WardState.Green

def zenoDampingStep (kappa : Nat) (rate_percent : Nat) : Nat :=
  (kappa * (100 - rate_percent)) / 100

def zenoDamping (kappa0 : Nat) (rate_percent : Nat) : Nat → Nat
  | 0 => kappa0
  | t + 1 => zenoDampingStep (zenoDamping kappa0 rate_percent t) rate_percent

theorem zeno_step_monotone (kappa : Nat) (rate_percent : Nat) (_hr : rate_percent ≤ 100) :
    zenoDampingStep kappa rate_percent ≤ kappa := by
  dsimp [zenoDampingStep]
  have h1 : 100 - rate_percent ≤ 100 := Nat.sub_le 100 rate_percent
  have h2 : kappa * (100 - rate_percent) ≤ 100 * kappa := by
    have h_mul := Nat.mul_le_mul_left kappa h1
    rw [Nat.mul_comm 100 kappa]
    exact h_mul
  exact Nat.div_le_of_le_mul h2

theorem ward_kill_safe (rho_scaled : Nat) (h : rho_scaled ≥ KILL_THRESHOLD) :
    classifyState rho_scaled = WardState.Kill := by
  dsimp [classifyState]
  split
  · rfl
  · rename_i h_not
    exact False.elim (h_not h)

theorem ward_green_safe (rho_scaled : Nat) (h : rho_scaled < RHO_WARN) :
    classifyState rho_scaled = WardState.Green := by
  dsimp [classifyState]
  split
  · rename_i h_kill
    have h_contra : RHO_WARN < KILL_THRESHOLD := by decide
    have : rho_scaled < KILL_THRESHOLD := Nat.lt_trans h h_contra
    exact False.elim (Nat.not_le_of_gt this h_kill)
  · split
    · rename_i _ h_red
      have h_contra : RHO_WARN < RHO_HALT := by decide
      have : rho_scaled < RHO_HALT := Nat.lt_trans h h_contra
      exact False.elim (Nat.not_le_of_gt this h_red)
    · split
      · rename_i _ _ h_amber
        exact False.elim (Nat.not_le_of_gt h h_amber)
      · rfl

end ZenoController
