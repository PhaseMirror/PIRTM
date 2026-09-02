import Foundations.ADR.Core
import Foundations.ADR.Proofs

/-!
# ADR-050: Multi-Node Distributed Governance Consensus

Formal Lean 4 model for ADR-050:
- Multi-node Sentinel consensus arbitration.
- Quorum threshold evaluation: cluster passes iff passVotes ≥ quorumThreshold.
- Vote aggregation: Pass/Kill tally with accounting invariant.

Mirrors the Rust/Kani implementation in
`pirtm-orchestration::distributed_governance` and
`adr_rust::distributed_governance_proof`.
-/

namespace PIRTM.DistributedGovernance

/-- ADR-050: A single node's governance vote on an ensemble. -/
inductive ConsensusVote where
  | pass (receipt : String)
  | kill (reason : String)
  deriving Repr, DecidableEq

/-- Aggregated cluster consensus outcome. -/
inductive ConsensusStatus where
  | clusterPass (receipt : Option String)
  | clusterKill (reason : String)
  deriving Repr, DecidableEq

/-- Extract boolean pass/fail status from `ConsensusStatus`. -/
def ConsensusStatus.isClusterPass : ConsensusStatus → Bool
  | ConsensusStatus.clusterPass _ => true
  | ConsensusStatus.clusterKill _ => false

/-- Cluster consensus metrics computed from vote aggregation. -/
structure ClusterMetrics where
  totalNodes : Nat
  passVotes : Nat
  killVotes : Nat
  quorumThreshold : Nat
  deriving Repr

/-- Pure quorum predicate: true iff `passVotes ≥ quorumThreshold`.

    This is the single source of truth, mirrored exactly in Rust (`is_quorum_reached`)
    and verified under Kani bounded model checking. -/
def isQuorumReached (passVotes quorumThreshold : Nat) : Bool :=
  passVotes >= quorumThreshold

/-- Convenience accessor on `ClusterMetrics` (preserves ADR-047 Test.lean API). -/
def isQuorumReachedMetrics (m : ClusterMetrics) : Bool :=
  isQuorumReached m.passVotes m.quorumThreshold

/-- Count `Pass` and `Kill` votes across a list. Returns `(passVotes, killVotes)`.

    Invariant: `passVotes + killVotes = votes.length` (see `countVotes_sum_eq_length`). -/
def countVotes (votes : List ConsensusVote) : Nat × Nat :=
  match votes with
  | [] => (0, 0)
  | v :: vs =>
    let (p, k) := countVotes vs
    match v with
    | ConsensusVote.pass _ => (p + 1, k)
    | ConsensusVote.kill _ => (p, k + 1)

/-- Vote-accounting invariant: the sum of pass and kill counts equals the
    total number of votes.

    **Theorem (ADR-050-VAI):** `passVotes + killVotes = totalVotes`.

    Machine-checked with zero `sorry`. -/
theorem countVotes_sum_eq_length (votes : List ConsensusVote) :
    (countVotes votes).1 + (countVotes votes).2 = votes.length := by
  induction votes with
  | nil => rfl
  | cons v vs ih =>
    dsimp [countVotes]
    match h : countVotes vs with
    | (p, k) =>
      rw [h] at ih
      cases v <;> (dsimp; omega)

/-- Quorum threshold validity: non-zero and at most `totalNodes`.

    A threshold of zero is degenerate (no quorum required). A threshold
    exceeding `totalNodes` makes `CLUSTER_PASS` unreachable. -/
def quorumThresholdValid (quorumThreshold totalNodes : Nat) : Bool :=
  quorumThreshold > 0 && quorumThreshold <= totalNodes

/-- Quorum threshold validity implies the threshold is bounded by total nodes. -/
theorem quorumThresholdValid_bounded (qt total : Nat)
    (h : quorumThresholdValid qt total = true) :
    qt <= total := by
  dsimp [quorumThresholdValid] at h
  simp at h
  omega

/-- Find the first `Pass` receipt in a vote list, if any. -/
def findFirstPassReceipt (votes : List ConsensusVote) : Option String :=
  match votes with
  | [] => none
  | ConsensusVote.pass r :: _ => some r
  | ConsensusVote.kill _ :: rest => findFirstPassReceipt rest

/-- Find the first `Kill` reason in a vote list, if any. -/
def findFirstKillReason (votes : List ConsensusVote) : Option String :=
  match votes with
  | [] => none
  | ConsensusVote.kill r :: _ => some r
  | ConsensusVote.pass _ :: rest => findFirstKillReason rest

/-- If there is at least one pass vote, `findFirstPassReceipt` returns `some`. -/
theorem findFirstPassReceipt_isSome_of_pass_gt_zero (votes : List ConsensusVote)
    (h : (countVotes votes).1 > 0) :
    ∃ r, findFirstPassReceipt votes = some r := by
  induction votes with
  | nil =>
    dsimp [countVotes] at h
    omega
  | cons v vs ih =>
    cases v with
    | pass r => exact ⟨r, rfl⟩
    | kill _ =>
      dsimp [countVotes] at h
      have ⟨r, hr⟩ := ih h
      exact ⟨r, hr⟩

/-- Aggregate a list of votes into a `ConsensusStatus`.

    Enforces the ADR-050 rule:
    `CLUSTER_PASS` iff `passVotes ≥ quorumThreshold`.

    On quorum: the first `Pass` receipt is the aggregate receipt.
    On failure: the first `Kill` reason is reported as `CLUSTER_SIG_GOV_KILL`. -/
def aggregateConsensus (votes : List ConsensusVote) (quorumThreshold : Nat) : ConsensusStatus :=
  let (passVotes, _killVotes) := countVotes votes
  if isQuorumReached passVotes quorumThreshold then
    match findFirstPassReceipt votes with
    | some receipt => ConsensusStatus.clusterPass (some receipt)
    | none => ConsensusStatus.clusterKill "no-pass-receipt"
  else
    match findFirstKillReason votes with
    | some reason => ConsensusStatus.clusterKill reason
    | none => ConsensusStatus.clusterKill "no-votes"

/-- Cluster status from `ClusterMetrics`. -/
def clusterStatus (m : ClusterMetrics) : ConsensusStatus :=
  if isQuorumReachedMetrics m then
    ConsensusStatus.clusterPass none
  else
    ConsensusStatus.clusterKill
      (if m.passVotes > 0 then "quorum not reached" else "no-pass-votes")

/-- **Theorem (ADR-050-QS): Cluster Consensus Quorum Soundness (iff).**

    `isQuorumReached passVotes quorumThreshold = true` *if and only if*
    `passVotes ≥ quorumThreshold`.

    This is the canonical ADR-050 statement: `CLUSTER_PASS` is reached if and
    only if the pass vote count satisfies or exceeds the quorum threshold.

    Zero `sorry`. Machine-checked in Lean 4 core (zero-Mathlib). -/
theorem cluster_consensus_quorum_soundness (passVotes quorumThreshold : Nat) :
    isQuorumReached passVotes quorumThreshold = true ↔ passVotes >= quorumThreshold := by
  dsimp [isQuorumReached]
  exact decide_eq_true_iff

/-- Corollary: `isQuorumReached` returns `true` when `passVotes ≥ quorumThreshold`. -/
theorem quorum_reached_forward (passVotes quorumThreshold : Nat)
    (h : passVotes >= quorumThreshold) :
    isQuorumReached passVotes quorumThreshold = true := by
  dsimp [isQuorumReached]
  exact decide_eq_true h

/-- Corollary: `isQuorumReached` returning `true` implies `passVotes ≥ quorumThreshold`. -/
theorem quorum_reached_backward (passVotes quorumThreshold : Nat)
    (h : isQuorumReached passVotes quorumThreshold = true) :
    passVotes >= quorumThreshold := by
  dsimp [isQuorumReached] at h
  exact of_decide_eq_true h

/-- When quorum is reached and `quorumThreshold > 0`, `aggregateConsensus` returns `clusterPass`. -/
theorem aggregateConsensus_reaches_quorum (votes : List ConsensusVote) (qt : Nat)
    (hqt : qt > 0)
    (h : isQuorumReached (countVotes votes).1 qt = true) :
    (aggregateConsensus votes qt).isClusterPass = true := by
  dsimp [isQuorumReached] at h
  have hp : (countVotes votes).1 > 0 := by
    have : (countVotes votes).1 ≥ qt := of_decide_eq_true h
    omega
  have ⟨r, hr⟩ := findFirstPassReceipt_isSome_of_pass_gt_zero votes hp
  dsimp [aggregateConsensus]
  have hq : isQuorumReached (countVotes votes).1 qt = true := h
  simp [hq, hr, ConsensusStatus.isClusterPass]

/-- When quorum is NOT reached, `aggregateConsensus` returns `clusterKill`. -/
theorem aggregateConsensus_fails_quorum (votes : List ConsensusVote) (qt : Nat)
    (h : isQuorumReached (countVotes votes).1 qt = false) :
    (aggregateConsensus votes qt).isClusterPass = false := by
  dsimp [aggregateConsensus]
  simp [h]
  cases findFirstKillReason votes <;> rfl

/-- All-pass votes reach quorum when threshold ≤ total nodes. -/
theorem countVotes_replicate_pass (n : Nat) :
    countVotes (List.replicate n (ConsensusVote.pass "ok")) = (n, 0) := by
  induction n with
  | zero => rfl
  | succ n ih => simp [List.replicate, countVotes, ih]

/-- All-kill votes produce zero pass votes. -/
theorem countVotes_replicate_kill (n : Nat) :
    countVotes (List.replicate n (ConsensusVote.kill "no")) = (0, n) := by
  induction n with
  | zero => rfl
  | succ n ih => simp [List.replicate, countVotes, ih]

/-- A cluster with all `Pass` votes reaches quorum when threshold ≤ total and threshold > 0. -/
theorem allPassClusterReachesQuorum (n qt : Nat) (hqt : qt > 0) (h : qt ≤ n) :
    (aggregateConsensus (List.replicate n (ConsensusVote.pass "ok")) qt).isClusterPass = true := by
  have hq : isQuorumReached (countVotes (List.replicate n (ConsensusVote.pass "ok"))).1 qt = true := by
    rw [countVotes_replicate_pass]
    dsimp [isQuorumReached]
    exact decide_eq_true h
  exact aggregateConsensus_reaches_quorum _ qt hqt hq

/-- A cluster with all `Kill` votes never reaches quorum when threshold > 0. -/
theorem allKillClusterFailsQuorum (n qt : Nat) (h : qt > 0) :
    (aggregateConsensus (List.replicate n (ConsensusVote.kill "no")) qt).isClusterPass = false := by
  have hq : isQuorumReached (countVotes (List.replicate n (ConsensusVote.kill "no"))).1 qt = false := by
    rw [countVotes_replicate_kill]
    dsimp [isQuorumReached]
    exact decide_eq_false (by omega)
  exact aggregateConsensus_fails_quorum _ qt hq

end PIRTM.DistributedGovernance
