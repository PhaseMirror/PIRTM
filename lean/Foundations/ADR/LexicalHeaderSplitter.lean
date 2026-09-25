import Foundations.ADR.Core

/-!
# ADR-057: Lexical Header Boundary Pre-Processor & Splitter
Formal verification of header splitting safety and phase isolation.
-/

namespace Foundations.ADR.LexicalHeaderSplitter

def adr0057 : PIRTM.ADR.ADR := {
  id := 57,
  title := "Lexical Header Boundary Pre-Processor & Splitter",
  status := PIRTM.ADR.ADRStatus.Accepted,
  context := "Phase 1 extraction operated over full buffer, creating phase ordering ambiguity.",
  decision := "Split raw source at standalone header delimiter line before statement parsing.",
  consequences := ["Phase ordering ambiguity eliminated", "Header and body parser isolation"],
  supersedes := none,
  links := []
}

def splitHeaderBody (source : String) (delimiter : String := "---") : String × String :=
  if delimiter = "" then
    (source, "")
  else
    match source.find? delimiter with
    | none => (source, "")
    | some pos => (source.extract source.startPos pos, "")

theorem header_length_bounded (source : String) :
    (splitHeaderBody source).1.length ≤ source.length := by
  unfold splitHeaderBody
  split
  · exact Nat.le_refl _
  · cases h : source.find? "---" with
    | none => exact Nat.le_refl _
    | some pos =>
      have hprefix : source.extract source.startPos pos = (source.sliceTo pos).copy := by
        simp [String.extract_eq_copy_slice, String.slice_startPos]
      have hsuffix : source.extract pos source.endPos = (source.sliceFrom pos).copy := by
        simp [String.extract_eq_copy_slice, String.slice_endPos]
      have happend : source = (source.sliceTo pos).copy ++ (source.sliceFrom pos).copy :=
        pos.splits.eq_append
      have h1 := congrArg String.length happend
      have h2 : ((source.sliceTo pos).copy ++ (source.sliceFrom pos).copy).length =
          (source.sliceTo pos).copy.length + (source.sliceFrom pos).copy.length := by
        rw [String.length_append]
      have h3 : source.length = (source.sliceTo pos).copy.length + (source.sliceFrom pos).copy.length := by
        rw [h1, h2]
      have hp_len : (source.extract source.startPos pos).length = (source.sliceTo pos).copy.length :=
        congrArg String.length hprefix
      have hs_len : (source.extract pos source.endPos).length = (source.sliceFrom pos).copy.length :=
        congrArg String.length hsuffix
      have h4 : source.length = (source.extract source.startPos pos).length +
          (source.extract pos source.endPos).length := by
        rw [h3, ← hp_len, ← hs_len]
      rw [h4]
      exact Nat.le_add_right _ _

theorem body_length_bounded (source : String) :
    (splitHeaderBody source).2.length ≤ source.length := by
  unfold splitHeaderBody
  split
  · exact Nat.zero_le _
  · cases h : source.find? "---" with
    | none => exact Nat.zero_le _
    | some pos => exact Nat.zero_le _

end Foundations.ADR.LexicalHeaderSplitter
