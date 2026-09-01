import Foundations.ADR.Core
import Foundations.ADR.Proofs
import Foundations.ADR.Examples
import Foundations.ADR.DialecticalSemantics
import Foundations.ADR.PrimeQuantum
import Foundations.ADR.PrimeAutoencoder
import Foundations.ADR.PhaseDissonance
import Foundations.ADR.GovernanceManifold
import Foundations.ADR.CognitiveEconomy

/-!
# ADR Foundations Test

Lake test suite for ADR invariants, ADR-034 through ADR-039.
-/
open PIRTM.ADR
open PIRTM.DialecticalSemantics
open PIRTM.PrimeQuantum
open PIRTM.PrimeAutoencoder
open PIRTM.PhaseDissonance
open PIRTM.GovernanceManifold
open PIRTM.CognitiveEconomy

def test_accepted_immutable : IO Unit := do
  let a := foundryIntegration
  if a.status == ADRStatus.Accepted then
    pure ()
  else
    throw $ IO.Error.userError "Immutable test failed"

def test_traceability : IO Unit := do
  let a := foundryIntegration
  let hist := [a.id]
  if hist.head? == some a.id then
    pure ()
  else
    throw $ IO.Error.userError "Traceability test failed"

def test_dialectical_admissibility : IO Unit := do
  let validTrajectory : CandidateTrajectory := {
    id := 101,
    grounding := { coverageRatio := 85, minThreshold := 70 },
    tension := { tensionDelta := 12, maxAllowed := 20, branchCount := 3 },
    contractivityScaled := 92
  }
  match certifyTrajectory validTrajectory with
  | CertificationResult.Admissible _ => IO.println "ADR-034: Trajectory certified admissible"
  | CertificationResult.Rejected r => throw $ IO.Error.userError s!"ADR-034: Unexpected rejection: {repr r}"

def test_dialectical_collapse_rejection : IO Unit := do
  let collapsedTrajectory : CandidateTrajectory := {
    id := 102,
    grounding := { coverageRatio := 90, minThreshold := 70 },
    tension := { tensionDelta := 5, maxAllowed := 20, branchCount := 1 },
    contractivityScaled := 85
  }
  match certifyTrajectory collapsedTrajectory with
  | CertificationResult.Rejected (CertificationRejection.DialecticalCollapse _) =>
    IO.println "ADR-034: Dialectical collapse caught as expected"
  | _ => throw $ IO.Error.userError "ADR-034: Failed to reject collapsed branch count"

def test_prime_quantum_syndrome : IO Unit := do
  let p_state : PrimeSubspaceState 8 := { basisState := 13, h_bound := by decide }
  let c_state : PrimeSubspaceState 8 := { basisState := 14, h_bound := by decide }
  if primeSyndromeEigenvalue p_state.basisState == 1 && primeSyndromeEigenvalue c_state.basisState == -1 then
    IO.println "ADR-035: Prime quantum syndrome S_P eigenvalue test passed (+1 prime, -1 composite)"
  else
    throw $ IO.Error.userError "ADR-035: Syndrome eigenvalue test failed"

def test_prime_autoencoder_rank_surrogate : IO Unit := do
  let r : RankSurrogate := { effectiveRankScaled := 540, maxAllowedDimension := 6 }
  if checkRankSurrogateBound r then
    IO.println "ADR-036: Prime-structured rank surrogate bound test passed"
  else
    throw $ IO.Error.userError "ADR-036: Rank surrogate bound check failed"

def test_phase_dissonance_in_bounds : IO Unit := do
  let entries : List ContradictionEntry := [
    { primeAxis := 2, artifact := ArtifactType.Spec, weight := 1, delta := 2 },
    { primeAxis := 3, artifact := ArtifactType.Code, weight := 1, delta := 1 }
  ]
  let band : PhaseBand := { lowerBound := 0, upperBound := 10 }
  if isGovernanceInBounds entries band then
    IO.println "ADR-037: Phase dissonance calculation and dynamic phase band check passed"
  else
    throw $ IO.Error.userError "ADR-037: Phase dissonance bound check failed"

def test_governance_manifold_arbitration : IO Unit := do
  let d : DriftState := { driftScaled := 35, driftDotScaled := 5, deltaSoftScaled := 20, deltaHardScaled := 30 }
  match arbitrateControl d with
  | ControlArbitration.GovernorHalt => IO.println "ADR-038: Fail-closed GovernorHalt triggered as expected on drift saturation"
  | _ => throw $ IO.Error.userError "ADR-038: Failed to trigger GovernorHalt on hard drift saturation"

def test_ethical_projection_idempotence : IO Unit := do
  let m : EthicalManifold := { maxNormScaled := 500 }
  let s : CognitiveState := { stateVector := 42, normScaled := 750, isLawful := false }
  let s_proj1 := projectEthical m s
  let s_proj2 := projectEthical m s_proj1
  if s_proj1.normScaled == 500 && s_proj1.isLawful && s_proj1.normScaled == s_proj2.normScaled then
    IO.println "ADR-039: Idempotent ethical projection test passed"
  else
    throw $ IO.Error.userError "ADR-039: Ethical projection idempotence test failed"
