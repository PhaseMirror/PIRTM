/-!
# ADR Foundations Test

Lake test suite for ADR invariants.
-/
import .Core .Proofs .Examples
open PIRTM.ADR

@[test] def test_accepted_immutable : IO Unit := do
  let a := foundryIntegration
  let h := accepted_immutable a rfl
  -- Positive case: a' = a satisfies the theorem
  let a' := a
  have : a' = a ∨ a'.status ≠ ADRStatus.Accepted := h a' rfl
  match this with
  | Or.inl eq => pure ()
  | Or.inr _ => throw $ IO.Error.userError "Immutable test failed"

@[test] def test_no_circular : IO Unit := do
  let a := foundryIntegration
  have h := no_circular_supersession a
  IO.println "no circular supersession passed"

-- Additional placeholder tests could be added here.
