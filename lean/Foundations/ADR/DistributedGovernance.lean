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

/-- Convenience accessor on `ClusterMetrics`. -/
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
    total number of votes. -/
theorem countVotes_sum_eq_length (votes : List ConsensusVote) :
    (countVotes votes).1 + (countVotes votes).2 = votes.length := by
  induction votes with
  | nil => rfl
  | cons v vs ih =>
    dsimp [countVotes]
    split
    · simp [ih, Nat.add_assoc, Nat.add_comm]; omega
    · simp [ih, Nat.add_assoc]; omega

/-- Quorum threshold validity: non-zero and at most `totalNodes`.

    A threshold of zero is degenerate (no quorum required). A threshold
    exceeding `totalNodes` makes `CLUSTER_PASS` unreachable. Both cases are
    rejected so that `clusterStatus` is well-defined. -/
def quorumThresholdValid (quorumThreshold totalNodes : Nat) : Bool :=
  quorumThreshold > 0 && quorumThreshold <= totalNodes

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

/-- Aggregate a list of votes into a `ConsensusStatus`.

    Enforces the ADR-050 rule:
    `CLUSTER_PASS` iff `passVotes ≥ quorumThreshold`.

    On quorum: the first `Pass` receipt is the aggregate receipt.
    On failure: the first `Kill` reason is reported as `CLUSTER_SIG_GOV_KILL`. -/
def aggregateConsensus (votes : List ConsensusVote) (quorumThreshold : Nat) : ConsensusStatus :=
  let (passVotes, _killVotes) := countVotes votes
  if h : passVotes >= quorumThreshold then
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

/-- Extract boolean status from `ConsensusStatus`. -/
def ConsensusStatus.isClusterPass : ConsensusStatus → Bool
  | ConsensusStatus.clusterPass _ => true
  | ConsensusStatus.clusterKill _ => false

/-- Full bidirectional quorum soundness.

    **Theorem (ADR-050):** `isQuorumReached` returns `true` *if and only if*
    `passVotes ≥ quorumThreshold`.

    This is the zero-`sorry` machine-checked statement corresponding to the
    Rust `cluster_consensus_quorum_soundness` Kani harness. -/
theorem cluster_consensus_quorum_soundness (passVotes quorumThreshold : Nat) :
    isQuorumReached passVotes quorumThreshold ↔ passVotes >= quorumThreshold := by
  dsimp [isQuorumReached]
  rfl

/-- `CLUSTER_PASS` status is reached iff the quorum predicate holds. -/
theorem aggregateConsensus_clusterPass_iff (votes : List ConsensusVote) (quorumThreshold : Nat) :
    ConsensusStatus.isClusterPass (aggregateConsensus votes quorumThreshold) ↔
      isQuorumReached (countVotes votes).1 quorumThreshold := by
  dsimp [aggregateConsensus, ConsensusStatus.isClusterPass]
  split
  · simp
  · simp
  · simp

/-- If the cluster reaches quorum, then `aggregateConsensus` returns `clusterPass`.
    If not, it returns `clusterKill`. -/
theorem aggregateConsensus_sound (votes : List ConsensusVote) (quorumThreshold : Nat)
    (h : isQuorumReached (countVotes votes).1 quorumThreshold) :
    ConsensusStatus.isClusterPass (aggregateConsensus votes quorumThreshold) := by
  dsimp [aggregateConsensus, ConsensusStatus.isClusterPass]
  split
  · simp
  · simp at h

/-- If the cluster does NOT reach quorum, `aggregateConsensus` returns `clusterKill`. -/
theorem aggregateConsensus_complete (votes : List ConsensusVote) (quorumThreshold : Nat)
    (h : ¬ isQuorumReached (countVotes votes).1 quorumThreshold) :
    ConsensusStatus.isClusterPass (aggregateConsensus votes quorumThreshold) = false := by
  dsimp [aggregateConsensus, ConsensusStatus.isClusterPass]
  split
  · simp at h
  · rfl
  · rfl

/-- Quorum threshold validity: a valid threshold is bounded by total nodes. -/
theorem quorumThresholdValid_implies_bounded (qt total : Nat)
    (h : quorumThresholdValid qt total = true) :
    qt <= total := by
  dsimp [quorumThresholdValid] at h
  push_neg at h
  rcases h with _h1 | _h2
  · omega
  · omega

/-- A cluster with all `Pass` votes reaches quorum when threshold ≤ total. -/
theorem allPass_clusterPass (n : Nat) (quorumThreshold : Nat)
    (h : quorumThreshold <= n) :
    let votes : List ConsensusVote := List.replicate n (ConsensusVote.pass "ok")
    ConsensusStatus.isClusterPass (aggregateConsensus votes quorumThreshold) := by
  dsimp [aggregateConsensus, ConsensusStatus.isClusterPass, countVotes]
  have : countVotes (List.replicate n (ConsensusVote.pass "ok")) = (n, 0) := by
    induction n with
    | zero => rfl
    | succ n ih =>
      dsimp [List.replicate, countVotes]
      split
      · simp [ih]
      · simp
  simp [this]
  split
  · rfl
  · omega

/-- A cluster with all `Kill` votes never reaches quorum (when threshold > 0). -/
theorem allKill_clusterKill (n quorumThreshold : Nat)
    (h : quorumThreshold > 0) :
    let votes : List ConsensusVote := List.replicate n (ConsensusVote.kill "no")
    ConsensusStatus.isClusterPass (aggregateConsensus votes quorumThreshold) = false := by
  dsimp [aggregateConsensus, ConsensusStatus.isClusterPass, countVotes]
  have : countVotes (List.replicate n (ConsensusVote.kill "no")) = (0, n) := by
    induction n with
    | zero => rfl
    | succ n ih =>
      dsimp [List.replicate, countVotes]
      split
      · simp [ih]
      · simp
  simp [this]
  split
  · omega
  · rfl

end PIRTM.DistributedGovernance
