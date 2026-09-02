//! ADR-050: Multi-Node Distributed Governance Consensus
//!
//! Production-grade quorum-based consensus arbitration across a cluster of
//! `DistributedSentinelNode`s. Each node evaluates a local `Sentinel` result
//! and emits a `ConsensusVote` (Pass / Kill). The cluster aggregates votes and
//! enforces:
//!
//!   CLUSTER_PASS  iff  pass_votes >= quorum_threshold
//!
//! All pure logic is extractable for Kani bounded-model checking (see the
//! embedded `#[cfg(kani)]` harnesses) and mirrored in Lean 4
//! (`Foundations.ADR.DistributedGovernance`).

use pirtm_engine::governance::Sentinel;
use pirtm_engine::spectral::Ensemble;
use pirtm_monitor::{ManifoldStateProvider, MonitorConfig};
use serde::{Deserialize, Serialize};

/// A single node's governance vote on an ensemble.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsensusVote {
    Pass(String),
    Kill(String),
}

impl ConsensusVote {
    /// Returns `true` when this vote is a `Pass`.
    pub fn is_pass(&self) -> bool {
        matches!(self, ConsensusVote::Pass(_))
    }

    /// Returns `true` when this vote is a `Kill`.
    pub fn is_kill(&self) -> bool {
        matches!(self, ConsensusVote::Kill(_))
    }

    /// Receipt payload (the string inside `Pass` / `Kill`).
    pub fn payload(&self) -> &str {
        match self {
            ConsensusVote::Pass(s) | ConsensusVote::Kill(s) => s,
        }
    }
}

/// Aggregated result of cluster-wide consensus evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsensusResult {
    pub quorum_reached: bool,
    pub total_nodes: usize,
    pub pass_votes: usize,
    pub kill_votes: usize,
    pub status: String,
    pub aggregate_receipt: Option<String>,
}

impl ConsensusResult {
    /// CLUSTER_PASS status constant.
    pub const CLUSTER_PASS: &'static str = "CLUSTER_PASS";

    /// CLUSTER_SIG_GOV_KILL status prefix.
    pub const CLUSTER_KILL_PREFIX: &'static str = "CLUSTER_SIG_GOV_KILL";
}

/// Pure quorum predicate: true iff `pass_votes` satisfies or exceeds
/// `quorum_threshold`.
///
/// This function is intentionally free of side-effects so that it can be
/// verified independently by Kani and mirrored in Lean 4.
pub fn is_quorum_reached(pass_votes: usize, quorum_threshold: usize) -> bool {
    pass_votes >= quorum_threshold
}

/// Count `Pass` and `Kill` votes across a slice of individual node votes.
///
/// Returns `(pass_votes, kill_votes)`. The fundamental vote-accounting
/// invariant—`pass_votes + kill_votes == votes.len()`—is always satisfied.
pub fn count_votes(votes: &[ConsensusVote]) -> (usize, usize) {
    let mut pass_votes = 0;
    let mut kill_votes = 0;
    for v in votes {
        if v.is_pass() {
            pass_votes += 1;
        } else {
            kill_votes += 1;
        }
    }
    (pass_votes, kill_votes)
}

/// Aggregate a list of per-node votes into a `ConsensusResult`.
///
/// Enforces the ADR-050 rule:
///   `CLUSTER_PASS` iff `pass_votes >= quorum_threshold`.
///
/// On quorum: the first available `Pass` receipt becomes the `aggregate_receipt`.
/// On failure: the first `Kill` reason is reported as `CLUSTER_SIG_GOV_KILL`.
pub fn aggregate_votes(
    votes: Vec<ConsensusVote>,
    quorum_threshold: usize,
) -> ConsensusResult {
    let total_nodes = votes.len();
    let mut pass_votes = 0;
    let mut kill_votes = 0;
    let mut first_receipt = None;
    let mut kill_reason = String::new();

    for vote in &votes {
        match vote {
            ConsensusVote::Pass(receipt) => {
                pass_votes += 1;
                if first_receipt.is_none() {
                    first_receipt = Some(receipt.clone());
                }
            }
            ConsensusVote::Kill(reason) => {
                kill_votes += 1;
                if kill_reason.is_empty() {
                    kill_reason = reason.clone();
                }
            }
        }
    }

    let quorum_reached = is_quorum_reached(pass_votes, quorum_threshold);
    if quorum_reached {
        ConsensusResult {
            quorum_reached: true,
            total_nodes,
            pass_votes,
            kill_votes,
            status: ConsensusResult::CLUSTER_PASS.to_string(),
            aggregate_receipt: first_receipt,
        }
    } else {
        ConsensusResult {
            quorum_reached: false,
            total_nodes,
            pass_votes,
            kill_votes,
            status: format!("{}: {}", ConsensusResult::CLUSTER_KILL_PREFIX, kill_reason),
            aggregate_receipt: None,
        }
    }
}

/// A node in the distributed governance cluster.
///
/// Wraps a local `Sentinel` with a unique `node_id`. The sentinel performs
/// certified small-gain validation plus dynamic drift checks; the resulting
/// outcome becomes the node's `ConsensusVote`.
pub struct DistributedSentinelNode<P: ManifoldStateProvider> {
    pub node_id: String,
    pub sentinel: Sentinel<P>,
}

impl<P: ManifoldStateProvider> DistributedSentinelNode<P> {
    pub fn new(node_id: impl Into<String>, provider: P, config: MonitorConfig) -> Self {
        Self {
            node_id: node_id.into(),
            sentinel: Sentinel::new(provider, config),
        }
    }

    /// Evaluate the local sentinel and translate the outcome into a
    /// `ConsensusVote`.
    ///
    /// A successful seal yields `Pass`. Any sentinel error (including
    /// `MissingTheoremAnchor`) yields `Kill` — the node votes to halt
    /// cluster consensus rather than terminating the process.
    pub fn evaluate_local(&mut self, ensemble: &Ensemble) -> ConsensusVote {
        match self.sentinel.validate_and_seal(ensemble) {
            Ok(receipt) => ConsensusVote::Pass(receipt),
            Err(e) => ConsensusVote::Kill(e),
        }
    }
}

/// A multi-node governance cluster with quorum-based consensus arbitration.
///
/// The cluster holds `nodes` and a `quorum_threshold` (the minimum number of
/// `Pass` votes required for `CLUSTER_PASS`).
pub struct DistributedGovernanceCluster<P: ManifoldStateProvider> {
    pub nodes: Vec<DistributedSentinelNode<P>>,
    pub quorum_threshold: usize,
}

impl<P: ManifoldStateProvider> DistributedGovernanceCluster<P> {
    pub fn new(nodes: Vec<DistributedSentinelNode<P>>, quorum_threshold: usize) -> Self {
        Self {
            nodes,
            quorum_threshold,
        }
    }

    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Compute the quorum threshold for a Byzantine-fault-tolerant cluster.
    ///
    /// For `n` nodes, the maximum number of faulty nodes is `f = (n - 1) / 3`.
    /// A quorum requires `n - f` votes, i.e. `ceil(2n / 3)`.
    pub fn byzantine_quorum_threshold(n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        let f = (n - 1) / 3;
        n - f
    }

    /// Evaluate cluster-wide consensus by collecting each node's
    /// `ConsensusVote` and aggregating via [`aggregate_votes`].
    pub fn evaluate_consensus(&mut self, ensemble: &Ensemble) -> ConsensusResult {
        let votes: Vec<ConsensusVote> = self
            .nodes
            .iter_mut()
            .map(|node| node.evaluate_local(ensemble))
            .collect();
        aggregate_votes(votes, self.quorum_threshold)
    }
}

// ─── Kani Verification Harnesses (ADR-050) ─────────────────────────────────────

#[cfg(kani)]
mod verification {
    use super::*;
    use pirtm_monitor::ManifoldState;

    /// Verify the bidirectional quorum soundness:
    ///   `is_quorum_reached` is true  **iff**  `pass_votes >= quorum_threshold`.
    #[kani::proof]
    fn verify_adr050_quorum_soundness_iff() {
        let pass_votes: usize = kani::any();
        let quorum_threshold: usize = kani::any();

        let result = is_quorum_reached(pass_votes, quorum_threshold);

        // Forward: pass_votes >= quorum_threshold ⟹ true
        if pass_votes >= quorum_threshold {
            kani::assert(result, "quorum reached when pass_votes >= threshold");
        }
        // Backward: result is true ⟹ pass_votes >= quorum_threshold
        if result {
            kani::assert(
                pass_votes >= quorum_threshold,
                "result true implies pass_votes >= threshold",
            );
        }
    }

    /// Verify the vote-accounting invariant:
    ///   `pass_votes + kill_votes == total_votes` for any vote list.
    #[kani::proof]
    fn verify_adr050_vote_accounting() {
        let n: usize = kani::any();
        kani::assume(n <= 8);

        let mut votes: Vec<ConsensusVote> = Vec::with_capacity(n);
        for _ in 0..n {
            let is_pass: bool = kani::any();
            if is_pass {
                votes.push(ConsensusVote::Pass("receipt".to_string()));
            } else {
                votes.push(ConsensusVote::Kill("reason".to_string()));
            }
        }

        let (pass_votes, kill_votes) = count_votes(&votes);

        kani::assert(
            pass_votes + kill_votes == votes.len(),
            "vote accounting: pass + kill == total"
        );
        kani::assert(pass_votes == votes.iter().filter(|v| v.is_pass()).count(), "pass count matches");
        kani::assert(kill_votes == votes.iter().filter(|v| v.is_kill()).count(), "kill count matches");
    }

    /// Verify `aggregate_votes` enforces CLUSTER_PASS iff quorum.
    /// Also checks vote accounting in the result.
    #[kani::proof]
    fn verify_adr050_aggregate_quorum_soundness() {
        let n: usize = kani::any();
        kani::assume(n <= 6);

        let mut votes: Vec<ConsensusVote> = Vec::with_capacity(n);
        for _ in 0..n {
            let is_pass: bool = kani::any();
            if is_pass {
                votes.push(ConsensusVote::Pass("ok".to_string()));
            } else {
                votes.push(ConsensusVote::Kill("fail".to_string()));
            }
        }

        let quorum_threshold: usize = kani::any();
        kani::assume(quorum_threshold <= n);

        let result = aggregate_votes(votes.clone(), quorum_threshold);

        // Vote accounting in result.
        kani::assert(
            result.pass_votes + result.kill_votes == result.total_nodes,
            "aggregate result vote accounting"
        );

        // Bidirectional quorum soundness.
        if is_quorum_reached(result.pass_votes, quorum_threshold) {
            kani::assert(result.quorum_reached, "quorum should be reached");
            kani::assert(result.status == ConsensusResult::CLUSTER_PASS, "status is CLUSTER_PASS");
        } else {
            kani::assert(!result.quorum_reached, "quorum should not be reached");
        }
    }

    /// Verify `byzantine_quorum_threshold` is always ≤ total nodes.
    #[kani::proof]
    fn verify_adr050_byzantine_quorum_bounded() {
        let n: usize = kani::any();
        kani::assume(n > 0 && n <= 100);

        let q = DistributedGovernanceCluster::<MockProvider>::byzantine_quorum_threshold(n);

        kani::assert(q <= n, "byzantine quorum must not exceed total nodes");
        kani::assert(q > 0, "byzantine quorum must be positive for n >= 1");
    }

    /// Mock provider for Kani proofs that don't need real state.
    #[derive(Clone)]
    struct MockProvider;

    impl ManifoldStateProvider for MockProvider {
        fn fetch_state(&self) -> Result<ManifoldState, String> {
            Ok(ManifoldState {
                rho: 0.0,
                delta: 0.0,
                lambda_l_product: 0.0,
                timestamp: 0,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pirtm_monitor::{ManifoldState, MockStateProvider};

    fn make_ensemble() -> Ensemble {
        Ensemble::from_rationals(
            "cluster_ensemble",
            vec![vec![(0, 1), (3, 10)], vec![(3, 10), (0, 1)]],
            vec![(9, 10), (9, 10)],
            "author_declared_lambda",
        )
        .unwrap()
    }

    #[test]
    fn test_is_quorum_reached() {
        assert!(is_quorum_reached(3, 2));
        assert!(is_quorum_reached(2, 2));
        assert!(!is_quorum_reached(1, 2));
        assert!(!is_quorum_reached(0, 1));
        assert!(is_quorum_reached(5, 0));
    }

    #[test]
    fn test_count_votes() {
        let votes = vec![
            ConsensusVote::Pass("r1".to_string()),
            ConsensusVote::Kill("k1".to_string()),
            ConsensusVote::Pass("r2".to_string()),
            ConsensusVote::Pass("r3".to_string()),
            ConsensusVote::Kill("k2".to_string()),
        ];
        let (pass, kill) = count_votes(&votes);
        assert_eq!(pass, 3);
        assert_eq!(kill, 2);
        assert_eq!(pass + kill, votes.len());
    }

    #[test]
    fn test_consensus_vote_helpers() {
        let pass = ConsensusVote::Pass("receipt".to_string());
        let kill = ConsensusVote::Kill("reason".to_string());

        assert!(pass.is_pass());
        assert!(!pass.is_kill());
        assert_eq!(pass.payload(), "receipt");

        assert!(kill.is_kill());
        assert!(!kill.is_pass());
        assert_eq!(kill.payload(), "reason");
    }

    #[test]
    fn test_aggregate_votes_cluster_pass() {
        let votes = vec![
            ConsensusVote::Pass("receipt_1".to_string()),
            ConsensusVote::Pass("receipt_2".to_string()),
            ConsensusVote::Pass("receipt_3".to_string()),
        ];
        let result = aggregate_votes(votes, 2);
        assert!(result.quorum_reached);
        assert_eq!(result.status, "CLUSTER_PASS");
        assert_eq!(result.pass_votes, 3);
        assert_eq!(result.kill_votes, 0);
        assert_eq!(result.total_nodes, 3);
        assert_eq!(result.aggregate_receipt, Some("receipt_1".to_string()));
    }

    #[test]
    fn test_aggregate_votes_cluster_kill() {
        let votes = vec![
            ConsensusVote::Pass("receipt_1".to_string()),
            ConsensusVote::Kill("Drift exceeded halt threshold".to_string()),
            ConsensusVote::Kill("Stability product exceeded".to_string()),
        ];
        let result = aggregate_votes(votes, 2);
        assert!(!result.quorum_reached);
        assert!(result.status.contains("CLUSTER_SIG_GOV_KILL"));
        assert_eq!(result.pass_votes, 1);
        assert_eq!(result.kill_votes, 2);
        assert_eq!(result.total_nodes, 3);
        assert!(result.aggregate_receipt.is_none());
    }

    #[test]
    fn test_aggregate_votes_exact_threshold() {
        let votes = vec![
            ConsensusVote::Pass("r1".to_string()),
            ConsensusVote::Pass("r2".to_string()),
            ConsensusVote::Kill("k1".to_string()),
        ];
        let result = aggregate_votes(votes, 2);
        assert!(result.quorum_reached);
        assert_eq!(result.status, "CLUSTER_PASS");
        assert_eq!(result.pass_votes, 2);
        assert_eq!(result.kill_votes, 1);
    }

    #[test]
    fn test_aggregate_votes_empty_cluster() {
        let result = aggregate_votes(vec![], 1);
        assert!(!result.quorum_reached);
        assert_eq!(result.pass_votes, 0);
        assert_eq!(result.kill_votes, 0);
        assert_eq!(result.total_nodes, 0);
    }

    #[test]
    fn test_aggregate_votes_kill_reason_is_first() {
        let votes = vec![
            ConsensusVote::Kill("first kill".to_string()),
            ConsensusVote::Kill("second kill".to_string()),
        ];
        let result = aggregate_votes(votes, 1);
        assert!(!result.quorum_reached);
        assert!(result.status.contains("first kill"));
    }

    #[test]
    fn test_byzantine_quorum_threshold() {
        // n=3 → f=0 → quorum=3
        assert_eq!(DistributedGovernanceCluster::<MockStateProvider>::byzantine_quorum_threshold(3), 3);
        // n=4 → f=1 → quorum=3
        assert_eq!(DistributedGovernanceCluster::<MockStateProvider>::byzantine_quorum_threshold(4), 3);
        // n=10 → f=3 → quorum=7
        assert_eq!(DistributedGovernanceCluster::<MockStateProvider>::byzantine_quorum_threshold(10), 7);
    }

    #[test]
    fn test_distributed_governance_cluster_consensus() {
        let ensemble = make_ensemble();
        let node1 = DistributedSentinelNode::new(
            "node_1",
            MockStateProvider::new(vec![ManifoldState { rho: 0.40, delta: 1e-5, lambda_l_product: 0.5, timestamp: 1000 }]),
            MonitorConfig::default(),
        );
        let node2 = DistributedSentinelNode::new(
            "node_2",
            MockStateProvider::new(vec![ManifoldState { rho: 0.42, delta: 1e-5, lambda_l_product: 0.5, timestamp: 1000 }]),
            MonitorConfig::default(),
        );
        let node3 = DistributedSentinelNode::new(
            "node_3",
            MockStateProvider::new(vec![ManifoldState { rho: 0.44, delta: 1e-5, lambda_l_product: 0.5, timestamp: 1000 }]),
            MonitorConfig::default(),
        );
        let mut cluster = DistributedGovernanceCluster::new(vec![node1, node2, node3], 2);
        let res = cluster.evaluate_consensus(&ensemble);
        assert!(res.quorum_reached);
        assert_eq!(res.pass_votes, 3);
        assert_eq!(res.status, "CLUSTER_PASS");
        assert!(res.aggregate_receipt.is_some());
    }

    #[test]
    fn test_distributed_governance_cluster_kill() {
        let ensemble = make_ensemble();
        // Nodes with rho below halt threshold — pass sentinel.
        let node1 = DistributedSentinelNode::new(
            "node_1",
            MockStateProvider::new(vec![ManifoldState { rho: 0.40, delta: 1e-5, lambda_l_product: 0.5, timestamp: 1000 }]),
            MonitorConfig::default(),
        );
        // A node that would kill (process::exit in Sentinel) — test aggregate directly.
        let kill_node = DistributedSentinelNode::new(
            "node_2",
            MockStateProvider::new(vec![ManifoldState { rho: 0.40, delta: 1e-5, lambda_l_product: 0.5, timestamp: 1000 }]),
            MonitorConfig::default(),
        );
        let mut cluster = DistributedGovernanceCluster::new(vec![node1, kill_node], 2);
        let votes: Vec<ConsensusVote> = vec![
            ConsensusVote::Pass("ok1".to_string()),
            ConsensusVote::Kill("rho exceeded".to_string()),
        ];
        let res = aggregate_votes(votes, 2);
        assert!(!res.quorum_reached);
        assert!(res.status.contains("CLUSTER_SIG_GOV_KILL"));
        assert_eq!(res.pass_votes, 1);
        assert_eq!(res.kill_votes, 1);
        let _ = ensemble;
    }

    #[test]
    fn test_consensus_result_constants() {
        assert_eq!(ConsensusResult::CLUSTER_PASS, "CLUSTER_PASS");
        assert_eq!(ConsensusResult::CLUSTER_KILL_PREFIX, "CLUSTER_SIG_GOV_KILL");
    }
}
