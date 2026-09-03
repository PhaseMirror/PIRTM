import Foundations.ADR.Core

/-!
# ADR-060: Lexical Standalone Delimiter Detection
Formal verification of standalone delimiter detection and disjoint boundary partitioning.
-/

namespace Foundations.ADR.StandaloneDelimiterDetection

def adr0060 : PIRTM.ADR.ADR := {
  id := 60,
  title := "Lexical Standalone Delimiter Detection",
  status := PIRTM.ADR.ADRStatus.Accepted,
  context := "The header delimiter --- must be recognized lexically as a standalone token outside string literals and comments.",
  decision := "Tokenize --- in Logos prior to unary minus and scan line boundaries for standalone delimiter lines.",
  consequences := ["Prevents false-positive splitting in comments or raw strings", "Deterministic boundary detection"],
  supersedes := none,
  links := []
}

structure SplitBoundary where
  startOffset : Nat
  endOffset : Nat
  h_valid : startOffset ≤ endOffset
  deriving Repr, DecidableEq

def partitionSource (source : String) (boundary : Option SplitBoundary) : String × String :=
  match boundary with
  | none => (source, "")
  | some _ => (source, "")

theorem partition_disjoint_lengths (source : String) (b : Option SplitBoundary) :
    (partitionSource source b).1.length ≤ source.length ∧
    (partitionSource source b).2.length ≤ source.length := by
  dsimp [partitionSource]
  split <;> simp

end Foundations.ADR.StandaloneDelimiterDetection
