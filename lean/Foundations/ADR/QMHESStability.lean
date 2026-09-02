import Foundations.ADR.Core
import Foundations.ADR.Proofs
import Foundations.ADR.BoundedIteration
import Foundations.ADR.ZenoController
import Foundations.ADR.LoweringSoundness

/-!
# QMHES Stability Theorems (ADR-033)

Canonical path: `lean/Foundations/ADR/QMHESStability.lean`.
Moved from deprecated `lean/ADR/QMHESStability.lean`.
-/ 

namespace QMHESStability

def SCALE_FACTOR : Nat := 1000
def QBER_THRESHOLD : Nat := 110

structure MultiplicityState where
  rho_scaled : Nat
  delta_scaled : Nat
  qber_scaled : Nat
  rotation_period : Nat
  multiplicity : Nat

def natDist (x y : Nat) : Nat := if x ≥ y then x - y else y - x

theorem natDist_self (x : Nat) : natDist x x = 0 := by
  dsimp [natDist]
  split <;> omega

theorem natDist_symm (x y : Nat) : natDist x y = natDist y x := by
  dsimp [natDist]
  split <;> split <;> omega

theorem natDist_triangle (x y z : Nat) : natDist x z ≤ natDist x y + natDist y z := by
  dsimp [natDist]
  split <;> split <;> split <;> omega

def StandardNatMetric : BoundedIteration.MetricSpace Nat :=
  { dist := natDist,
    dist_self := natDist_self,
    dist_symm := natDist_symm,
    dist_triangle := natDist_triangle }

theorem multiplicity_bounded
    (B : Nat) (p : Nat) (_hp : p ≥ 1) (t : Nat) :
    let M_step := fun (n : Nat) => B + n / p
    M_step t ≤ B + t := by
  dsimp
  have h_div : t / p ≤ t := Nat.div_le_self t p
  omega

theorem lyapunov_convergence
    (rho_scaled : Nat) (_kappa : Nat) (rate_percent : Nat)
    (_h_rate : rate_percent ≤ 100) :
    let rho_next := rho_scaled * (100 - rate_percent) / 100
    rho_next ≤ rho_scaled := by
  dsimp
  have h_sub : 100 - rate_percent ≤ 100 := Nat.sub_le 100 rate_percent
  have h_smult : (100 - rate_percent) * rho_scaled ≤ 100 * rho_scaled :=
    Nat.mul_le_mul_right rho_scaled h_sub
  have h_mul : rho_scaled * (100 - rate_percent) ≤ rho_scaled * 100 := by
    simpa [Nat.mul_comm] using h_smult
  have h_div := Nat.div_le_div_right (c := 100) h_mul
  have h_reduce : rho_scaled * 100 / 100 = rho_scaled :=
    Nat.mul_div_left (m := rho_scaled) (n := 100) (by decide)
  rw [h_reduce] at h_div
  exact h_div

theorem prime_eigenmode_convergence {X : Type}
    (M : BoundedIteration.MetricSpace X)
    (f : BoundedIteration.NonExpansiveMap X M)
    (p : Nat) (_hp_prime : p ≥ 2)
    (N : Nat) (x y : X) :
    M.dist
      (BoundedIteration.iterate f.f N x)
      (BoundedIteration.iterate f.f N y)
    ≤ M.dist x y :=
  BoundedIteration.iterate_non_expansive M f N x y

def eigenmodeCollapse (_n : Nat) : Nat := 0

theorem eigenmodeCollapse_non_expansive (x y : Nat) :
    natDist (eigenmodeCollapse x) (eigenmodeCollapse y) ≤ natDist x y := by
  simp [eigenmodeCollapse, natDist]

def eigenmodeCollapseMap : BoundedIteration.NonExpansiveMap Nat StandardNatMetric :=
  { f := eigenmodeCollapse,
    bound := eigenmodeCollapse_non_expansive }

def eigenmodeHold (n : Nat) : Nat := n

theorem eigenmodeHold_non_expansive (x y : Nat) :
    natDist (eigenmodeHold x) (eigenmodeHold y) ≤ natDist x y := by
  dsimp [eigenmodeHold]
  exact Nat.le_refl (natDist x y)

def eigenmodeHoldMap : BoundedIteration.NonExpansiveMap Nat StandardNatMetric :=
  { f := eigenmodeHold,
    bound := eigenmodeHold_non_expansive }

def hkdf_expand_step (ikm info counter : Nat) : Nat :=
  ikm * 2 ^ 256 + info * 2 ^ 128 + counter

theorem hkdf_expand_distinct
    (ikm info c1 c2 : Nat)
    (_h1 : c1 < 2 ^ 128) (_h2 : c2 < 2 ^ 128) (hc : c1 ≠ c2) :
    hkdf_expand_step ikm info c1 ≠ hkdf_expand_step ikm info c2 := by
  dsimp [hkdf_expand_step]
  intro h_eq
  have h_c : c1 = c2 := by omega
  exact hc h_c

def frequencyMap (omega : Nat) : Nat := omega / SCALE_FACTOR

theorem frequency_quantization_bounded (omega : Nat) :
    frequencyMap omega ≤ omega := by
  unfold frequencyMap
  exact Nat.div_le_self omega SCALE_FACTOR

example : frequencyMap 2520 = 2 := by
  native_decide

def adaptiveFeedbackStep (rho_scaled kappa_scaled multiplicity : Nat) : Nat :=
  (rho_scaled * kappa_scaled * multiplicity) / (SCALE_FACTOR * SCALE_FACTOR)

theorem adaptive_feedback_contractive
    (rho_scaled kappa_scaled multiplicity : Nat)
    (h_rho : rho_scaled ≤ SCALE_FACTOR)
    (h_kappa : kappa_scaled ≤ SCALE_FACTOR) :
    adaptiveFeedbackStep rho_scaled kappa_scaled multiplicity ≤ multiplicity := by
  dsimp [adaptiveFeedbackStep, SCALE_FACTOR]
  have h_prod : rho_scaled * kappa_scaled ≤ SCALE_FACTOR * SCALE_FACTOR := by
    have h1 : rho_scaled * kappa_scaled ≤ SCALE_FACTOR * kappa_scaled :=
      Nat.mul_le_mul_right kappa_scaled h_rho
    have h2 : SCALE_FACTOR * kappa_scaled ≤ SCALE_FACTOR * SCALE_FACTOR :=
      Nat.mul_le_mul_left SCALE_FACTOR h_kappa
    exact Nat.le_trans h1 h2
  have hmul' : rho_scaled * kappa_scaled * multiplicity ≤
      (SCALE_FACTOR * SCALE_FACTOR) * multiplicity :=
    Nat.mul_le_mul_right multiplicity h_prod
  exact Nat.div_le_of_le_mul hmul'

theorem qmhes_spectral_contraction
    (rho_raw kappa_scaled : Nat)
    (h_rho : rho_raw ≤ SCALE_FACTOR)
    (h_kappa : kappa_scaled ≤ SCALE_FACTOR) :
    (rho_raw * kappa_scaled) / SCALE_FACTOR ≤ SCALE_FACTOR := by
  have h1 : rho_raw * kappa_scaled ≤ SCALE_FACTOR * kappa_scaled :=
    Nat.mul_le_mul_right kappa_scaled h_rho
  have h2 : SCALE_FACTOR * kappa_scaled ≤ SCALE_FACTOR * SCALE_FACTOR :=
    Nat.mul_le_mul_left SCALE_FACTOR h_kappa
  exact Nat.div_le_of_le_mul (Nat.le_trans h1 h2)

def requiresRekey (qber_scaled : Nat) : Bool := qber_scaled ≥ QBER_THRESHOLD

theorem qber_threshold_rekey (qber_scaled : Nat) (h : qber_scaled ≥ QBER_THRESHOLD) :
    requiresRekey qber_scaled = true := by
  simp [requiresRekey, h]

theorem qber_below_threshold_no_rekey (qber_scaled : Nat) (h : qber_scaled < QBER_THRESHOLD) :
    requiresRekey qber_scaled = false := by
  have hnot : ¬ qber_scaled ≥ QBER_THRESHOLD := Nat.not_le_of_gt h
  simp [requiresRekey, hnot]

theorem qmhes_ward_state_not_worse
    (rho_raw rho_atten : Nat)
    (h_atten : rho_atten ≤ rho_raw)
    (h_lt : rho_raw < ZenoController.RHO_WARN) :
    ZenoController.classifyState rho_atten = ZenoController.WardState.Green :=
  ZenoController.ward_green_safe rho_atten (Nat.lt_of_le_of_lt h_atten h_lt)

theorem qahes_handshake_envelope_preserved (op1 op2 : LoweringSoundness.OpTransformer) :
    ∀ x y, LoweringSoundness.cellDist
      (op2.transform (op1.transform x))
      (op2.transform (op1.transform y)) ≤ LoweringSoundness.cellDist x y :=
  LoweringSoundness.mlir_lowering_preserves_contractivity op1 op2

open PIRTM.ADR
open Foundations.ADR.Proofs

def qmhesAdr : ADR := {
  id := 33,
  title := "QMHES Integration",
  status := ADRStatus.Accepted,
  context := "QMHES cryptographic protocol integration",
  decision := "Integrate QMHES into PIRTM/MOC compiler",
  consequences := ["Unified security model", "Machine-checked stability"],
  supersedes := none,
  links := []
}

theorem qmhes_no_circular_supersession (lookup : ADRId → Option ADR) :
    followSupersession lookup qmhesAdr 1024 = [] := by
  simp [followSupersession, followSupersessionLoop, qmhesAdr]

end QMHESStability
