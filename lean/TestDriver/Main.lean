import Foundations.ADR.Export
import Foundations.ADR.Test

def main : IO UInt32 := do
  test_accepted_immutable
  test_traceability
  test_dialectical_admissibility
  test_dialectical_collapse_rejection
  test_prime_quantum_syndrome
  test_prime_autoencoder_rank_surrogate
  test_phase_dissonance_in_bounds
  test_governance_manifold_arbitration
  test_ethical_projection_idempotence
  exportAll
  IO.println "All Lean ADR tests and documentation exports executed successfully."
  return 0
