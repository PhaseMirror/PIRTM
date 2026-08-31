
import ADR.Core

namespace ADR

/-! ## Entailment Logic -/

/--
A minimal justification record for consequence entailment.

`premises` is the set of atomic facts derived from `context` and `decision`.
`conclusion` is the claimed consequence.
-/
structure Justification where
  premises : List String
  conclusion : String
  deriving Repr, Inhabited

namespace Justification

/--
Simple entailment: the conclusion must be a non-empty string that appears
explicitly in the premises.
-/
def entails (j : Justification) : Bool :=
  j.conclusion.length > 0 && j.premises.contains j.conclusion

/--
Build a justification from an ADR's context and decision by treating
every non-empty line as an atomic premise.
-/
def fromADR (a : ADR) : Justification :=
  let ctxLines := (a.context.split (· == '\n')).map (·.toString) |>.filter (· != "") |>.toList
  let decLines := (a.decision.split (· == '\n')).map (·.toString) |>.filter (· != "") |>.toList
  ⟨ctxLines ++ decLines, "dummy"⟩

end Justification

/-! ## Immutability After Acceptance -/

/--
Accepted ADRs can only remain accepted or move to superseded (with a valid
supersession target).  No other transitions are permitted.
-/
theorem accepted_immutable_without_supersession
    (old new : ADRStatus) (sup : Option ADRId) :
    old = ADRStatus.Accepted →
    validTransition old new sup = true →
    new = ADRStatus.Accepted ∨ (new = ADRStatus.Superseded ∧ sup.isSome) := by
  intro hOld hTrans
  unfold validTransition at hTrans
  simp [hOld] at hTrans
  rcases hTrans with hEq | hEq
  · apply Or.inl; exact hEq.symm
  · apply Or.inr; exact hEq

/--
An ADR in `Accepted` status can never revert to `Proposed`.
-/
theorem accepted_cannot_revert_to_proposed
    (old new : ADRStatus) (sup : Option ADRId) :
    old = ADRStatus.Accepted →
    validTransition old new sup = true →
    new ≠ ADRStatus.Proposed := by
  intro hOld hTrans hNew
  unfold validTransition at hTrans
  simp [hOld, hNew] at hTrans

/--
An ADR in `Accepted` status can never move to `Deprecated` without
supersession.
-/
theorem accepted_cannot_deprecate_without_supersede
    (old new : ADRStatus) (sup : Option ADRId) :
    old = ADRStatus.Accepted →
    validTransition old new sup = true →
    new ≠ ADRStatus.Deprecated := by
  intro hOld hTrans hNew
  unfold validTransition at hTrans
  simp [hOld, hNew] at hTrans

/-! ## No Circular Supersession -/

/-
/--
`followSupersession` is bounded by `fuel`.  Therefore the returned chain
has length at most `fuel`, and no cycle can be traversed within that bound.
-/
theorem followSupersession_length_bounded
    (lookup : ADRId → Option ADR) (a : ADR) (fuel : Nat) :
    (followSupersession lookup a fuel).length ≤ fuel := by
  sorry
-/

/-
/--
`followSupersession` terminates at an ADR with `supersedes = none` before
the fuel bound is exhausted, provided the lookup function is well-formed.
-/
theorem followSupersession_terminates_at_root
    (lookup : ADRId → Option ADR) (a : ADR) (fuel : Nat)
    (hFuel : fuel > 0)
    (hLookup : ∀ id, lookup id = none ∨ ∃ adr, lookup id = some adr) :
    let chain := followSupersession lookup a fuel
    chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast (by simp [hFuel])) = none) := by
  sorry

/-! ## Traceability -/

/--
An ADR is reconstructible if its supersession chain terminates at an ADR
with `supersedes = none` before the fuel runs out.
-/
def Reconstructible (lookup : ADRId → Option ADR) (a : ADR) : Prop :=
  ∃ fuel, fuel > 0 ∧
  let chain := followSupersession lookup a fuel
  chain = [] ∨ (chain ≠ [] ∧ lookup (chain.getLast (by simp [hFuelPos])) = none)

/--
If an ADR is `Accepted` and has no supersession target, it is trivially
reconstructible (fuel = 1).
-/
theorem accepted_without_supersession_reconstructible
    (a : ADR) :
    a.status = ADRStatus.Accepted →
    a.supersedes = none →
    Reconstructible (fun _ => none) a := by
  sorry

/--
If an ADR is `Accepted` and supersedes another ADR, and the lookup function
terminates, then the ADR is reconstructible.
-/
theorem accepted_with_supersession_reconstructible
    (lookup : ADRId → Option ADR) (a : ADR)
    (hAcc : a.status = ADRStatus.Accepted)
    (hSup : a.supersedes = some targetId)
    (hTarget : lookup targetId = some target)
    (hTargetRecon : Reconstructible lookup target) :
    Reconstructible lookup a := by
  sorry
-/

/-! ## Consequence Entailment -/

/--
`ConsequencesEntailed a` holds when every consequence listed in `a` is
non-empty and can be derived from `a.context` or `a.decision`.
-/
def ConsequencesEntailed (a : ADR) : Bool :=
  let ctxLines := (a.context.split (· == '\n')).map (·.toString) |>.filter (· != "") |>.toList
  let decLines := (a.decision.split (· == '\n')).map (·.toString) |>.filter (· != "") |>.toList
  let premises := ctxLines ++ decLines
  a.consequences.all (fun c => c.length > 0 && premises.contains c)

/-
/--
If an ADR is `Accepted` and its consequences are entailed, then all
consequences are non-empty strings.
-/
theorem accepted_adr_consequences_nonempty (a : ADR) :
    a.status = ADRStatus.Accepted →
    ConsequencesEntailed a →
    a.consequences.all (fun c => c.length > 0) := by
  sorry

/--
An ADR is `JustifiedWith js` if the explicit justifications `js` match the
consequences one-to-one and each justification satisfies `entails`.
-/
def JustifiedWith (a : ADR) (js : List Justification) : Prop :=
  js.length = a.consequences.length ∧ js.all (·.entails)

/--
If an ADR is `Accepted` and explicitly justified, then its consequences
are entailed by construction.
-/
theorem accepted_adr_explicitly_justified
    (a : ADR) (js : List Justification) :
    a.status = ADRStatus.Accepted →
    JustifiedWith a js →
    js.all (fun j => j.entails → True) := by
  sorry
-/
end ADR
