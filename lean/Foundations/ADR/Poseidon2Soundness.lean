import Foundations.ADR.Core
import Foundations.ADR.Proofs

/-!
# ADR-049 receipt flag conjunction (not Poseidon2 soundness)

This module does not define a sponge, a field, a constraint system,
or knowledge soundness. It records two Boolean flags.
-/

namespace PIRTM.Poseidon2Soundness

structure Poseidon2Receipt where
  constraintCount : Nat
  isValid : Bool
  deriving Repr

/-- Conjunction of author-set flags. Not a ZK verifier. -/
def receipt_flag_conjunction (receipt : Poseidon2Receipt) : Bool :=
  receipt.isValid && receipt.constraintCount <= 5087

theorem receipt_flag_conjunction_of_hyps (receipt : Poseidon2Receipt)
    (h_valid : receipt.isValid = true)
    (h_bound : receipt.constraintCount <= 5087) :
    receipt_flag_conjunction receipt = true := by
  dsimp [receipt_flag_conjunction]
  simp [h_valid, h_bound]

end PIRTM.Poseidon2Soundness
