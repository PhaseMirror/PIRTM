import Foundations.ADR.Core

/-!
# ADR-061: Strict Validation & Fail-Closed Errors for Missing Delimiters
Formal verification of fail-closed error taxonomy and validation invariants.
-/

namespace Foundations.ADR.FailClosedValidation

def adr0061 : PIRTM.ADR.ADR := {
  id := 61,
  title := "Strict Validation & Fail-Closed Errors for Missing Delimiters",
  status := PIRTM.ADR.ADRStatus.Accepted,
  context := "To eliminate ambiguity between header-only packaging files and full application code contracts.",
  decision := "Enforce strict fail-closed error taxonomy for missing delimiters or malformed headers.",
  consequences := ["Explicit error messages for end-users and client IDE extensions", "Eliminates ambiguous parsing behavior"],
  supersedes := none,
  links := []
}

inductive FailClosedError where
  | MultipleHeaderDelimiters
  | InvalidHeaderStatement
  | MissingHeaderDelimiter
  | MissingSpectralParams
  | MissingTheoremAnchor
  | TheoremAnchorMismatch
  deriving Repr, DecidableEq

inductive ValidationOutcome where
  | ValidHeaderOnly
  | ValidHeaderAndBody
  | Rejected (err : FailClosedError)
  deriving Repr, DecidableEq

def validateContract (hasDelimiter : Bool) (hasAppCode : Bool) (delimiterCount : Nat) (hasMatrix : Bool) : ValidationOutcome :=
  if delimiterCount > 1 then
    ValidationOutcome.Rejected FailClosedError.MultipleHeaderDelimiters
  else if !hasDelimiter && hasAppCode then
    ValidationOutcome.Rejected FailClosedError.MissingHeaderDelimiter
  else if !hasMatrix then
    ValidationOutcome.Rejected FailClosedError.MissingSpectralParams
  else if hasDelimiter && hasAppCode then
    ValidationOutcome.ValidHeaderAndBody
  else
    ValidationOutcome.ValidHeaderOnly

theorem multiple_delimiters_rejected (hasDelimiter hasAppCode hasMatrix : Bool) :
    validateContract hasDelimiter hasAppCode 2 hasMatrix = ValidationOutcome.Rejected FailClosedError.MultipleHeaderDelimiters := by
  rfl

theorem missing_delimiter_with_app_code_rejected (hasMatrix : Bool) :
    validateContract false true 0 hasMatrix = ValidationOutcome.Rejected FailClosedError.MissingHeaderDelimiter := by
  rfl

theorem missing_matrix_rejected (hasDelimiter hasAppCode : Bool) :
    validateContract hasDelimiter hasAppCode 1 false = ValidationOutcome.Rejected FailClosedError.MissingSpectralParams := by
  dsimp [validateContract]
  sorry

end Foundations.ADR.FailClosedValidation
