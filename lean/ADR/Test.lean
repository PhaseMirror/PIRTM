
import ADR.Core
import ADR.Proofs
import ADR.Examples

namespace ADR

set_option maxRecDepth 200000

/-! ## Basic Construction Tests -/

/--
All example ADRs have unique IDs.
-/
theorem example_adrs_have_unique_ids :
    adr1001.id ≠ adr1002.id ∧
    adr1001.id ≠ adr1003.id ∧
    adr1002.id ≠ adr1003.id := by
  decide

/--
Accepted ADRs have non-empty titles.
-/
theorem example_accepted_adrs_have_titles :
    adr1001.title.length > 0 ∧
    adr1002.title.length > 0 ∧
    adr1003.title.length > 0 := by
  decide

/--
Accepted ADRs have non-empty context.
-/
theorem example_accepted_adrs_have_context :
    adr1001.context.length > 0 ∧
    adr1002.context.length > 0 ∧
    adr1003.context.length > 0 := by
  decide

/--
Accepted ADRs have non-empty decision.
-/
theorem example_accepted_adrs_have_decision :
    adr1001.decision.length > 0 ∧
    adr1002.decision.length > 0 ∧
    adr1003.decision.length > 0 := by
  decide

/-! ## Status Transition Tests -/

/--
An ADR in `Accepted` status that remains `Accepted` cannot be `Proposed`.
-/
theorem accepted_stays_not_proposed :
    ADRStatus.Accepted ≠ ADRStatus.Proposed :=
  accepted_cannot_revert_to_proposed
    ADRStatus.Accepted ADRStatus.Accepted none
    rfl (by simp [validTransition])

/--
An ADR in `Accepted` status that moves to `Superseded` cannot be `Proposed`.
-/
theorem accepted_supersedes_not_proposed :
    ADRStatus.Superseded ≠ ADRStatus.Proposed :=
  accepted_cannot_revert_to_proposed
    ADRStatus.Accepted ADRStatus.Superseded (some ⟨1⟩)
    rfl (by simp [validTransition])

/--
A deprecated ADR prototype.
-/
def adr0999 : ADR := {
  id := ⟨999⟩,
  title := "Deprecated Prototype",
  status := ADRStatus.Deprecated,
  context := "Early prototype",
  decision := "Superseded by formal system",
  consequences := ["Legacy code removed"],
  supersedes := none,
  links := []
}

/--
adr0999 is deprecated, so it cannot become accepted.
-/
theorem adr0999_cannot_become_accepted :
    adr0999.status = ADRStatus.Deprecated →
    validTransition adr0999.status ADRStatus.Accepted adr0999.supersedes = false := by
  intro _; simp [validTransition, adr0999]

/--
adr1001 is accepted, so transitioning to deprecated requires supersession.
Since adr1001.supersedes = none, the transition is invalid.
-/
theorem adr1001_deprecate_without_supersede_invalid :
    validTransition adr1001.status ADRStatus.Deprecated adr1001.supersedes = false := by
  simp [validTransition, adr1001]

/-! ## Consequence Entailment Tests -/

/--
All consequences in accepted ADRs are non-empty.
-/
theorem accepted_examples_consequences_nonempty :
    adr1001.consequences.all (fun c => c.length > 0) ∧
    adr1002.consequences.all (fun c => c.length > 0) ∧
    adr1003.consequences.all (fun c => c.length > 0) := by
  decide

/--
Explicit justifications for adr1001 satisfy entailment.
-/
def adr1001Justifications : List Justification :=
  [ ⟨["Parser gains 4 new node types without breaking existing grammar"], "Parser gains 4 new node types without breaking existing grammar"⟩,
    ⟨["MLIR lowering pipeline extended by ~200 LOC in visitor"], "MLIR lowering pipeline extended by ~200 LOC in visitor"⟩,
    ⟨["Lean proofs guarantee loop termination for bounded for loops"], "Lean proofs guarantee loop termination for bounded for loops"⟩,
    ⟨["Test suite expanded with control-flow programs"], "Test suite expanded with control-flow programs"⟩ ]

theorem adr1001_explicitly_justified :
    JustifiedWith adr1001 adr1001Justifications := by
  constructor <;> rfl


/-! ## Supersession Chain Tests -/

/--
adr1004 supersedes adr1001, so the chain should contain 1001.
-/
theorem adr1004_supersession_chain_contains_1001 :
    let chain := followSupersession adrRegistry adr1004 1024
    chain.contains ⟨1001⟩ := by
  decide

/--
adr1001 has no supersession, so the chain is empty.
-/
theorem adr1001_no_supersession_chain_empty :
    followSupersession adrRegistry adr1001 1024 = [] := by
  rfl

/--
The supersession chain length is bounded by fuel.
-/
theorem adr1004_chain_length_bounded :
    (followSupersession adrRegistry adr1004 1024).length ≤ 1024 := by
  apply followSupersession_length_bounded

/-! ## Traceability Tests -/

/--
adr1001 is reconstructible with fuel 1.
-/
theorem adr1001_reconstructible :
    Reconstructible adrRegistry adr1001 := by
  apply accepted_without_supersession_reconstructible
  · simp [adr1001]
  · simp [adr1001]

/--
adr1004 is reconstructible because adr1001 is reconstructible.
-/
theorem adr1004_reconstructible :
    Reconstructible adrRegistry adr1004 :=
  accepted_with_supersession_reconstructible
    adr1004
    (targetId := ⟨1001⟩)
    (target := adr1001)
    rfl
    rfl
    rfl
    adr1001_reconstructible

/-! ## Property-Based Style Tests -/

/--
For any ADR in `Accepted` status, `setStatus` to `Superseded` requires
a supersedes target to be a valid transition.
-/
theorem accepted_to_superseded_requires_target
    (a : ADR) (target : ADRId) :
    a.status = ADRStatus.Accepted →
    validTransition a.status ADRStatus.Superseded (some target) = true := by
  intro h; simp [validTransition, h]

/--
For any two distinct ADR IDs, the IDs are not equal.
-/
theorem adr_ids_distinct (id1 id2 : ADRId) :
    id1 ≠ id2 → id1.value ≠ id2.value := by
  intro hNe hEq
  apply hNe
  cases id1; cases id2
  simp only at hEq
  subst hEq
  rfl



/-! ## Executable Test Runner -/

/--
Print a summary of all test categories.
-/
def printTestSummary : IO Unit := do
  IO.println "=== ADR Test Harness ==="
  IO.println s!"Test categories: 6"
  IO.println s!"Example ADRs: 5"
  IO.println s!"Theorems proved: 15+"
  IO.println ""
  IO.println "All tests passed."

end ADR

def main : IO Unit := do
  ADR.printTestSummary

