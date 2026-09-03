import Foundations.ADR.Core

/-!
# ADR-058: Formal Header Envelope Grammar & Scope Isolation
Formal verification of header envelope statement classification and quarantine.
-/

namespace Foundations.ADR.FormalHeaderEnvelopeGrammar

def adr0058 : PIRTM.ADR.ADR := {
  id := 58,
  title := "Formal Header Envelope Grammar & Scope Isolation",
  status := PIRTM.ADR.ADRStatus.Accepted,
  context := "Application control flow tokens must be quarantined from header envelope grammar.",
  decision := "Restrict pirtm.pest to packaging envelope declarations only.",
  consequences := ["Envelope files conform strictly to specification", "Malformed headers fail closed"],
  supersedes := none,
  links := []
}

inductive EnvelopeStmtKind where
  | Matrix
  | Lambdas
  | Theorem
  | Import
  | Ensemble
  | InvalidAppCode
  deriving Repr, DecidableEq

def isAllowedHeaderStmt : EnvelopeStmtKind → Bool
  | EnvelopeStmtKind.Matrix => true
  | EnvelopeStmtKind.Lambdas => true
  | EnvelopeStmtKind.Theorem => true
  | EnvelopeStmtKind.Import => true
  | EnvelopeStmtKind.Ensemble => true
  | EnvelopeStmtKind.InvalidAppCode => false

theorem invalid_app_code_quarantined :
    isAllowedHeaderStmt EnvelopeStmtKind.InvalidAppCode = false := by
  rfl

theorem allowed_header_stmts_verified (k : EnvelopeStmtKind)
    (h : k ≠ EnvelopeStmtKind.InvalidAppCode) :
    isAllowedHeaderStmt k = true := by
  cases k <;> try rfl
  · contradiction

end Foundations.ADR.FormalHeaderEnvelopeGrammar
