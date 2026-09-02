use pirtm_engine::governance::Sentinel;
use pirtm_engine::spectral::Ensemble;
use pirtm_monitor::{ManifoldState, ManifoldStateProvider, MonitorConfig};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsensusVote {
    Pass(String),
    Kill(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub quorum_reached: bool,
    pub total_nodes: usize,
    pub pass_votes: usize,
    pub kill_votes: usize,
    pub status: String,
    pub aggregate_receipt: Option<String>,
}

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

    pub fn evaluate_local(&mut self, ensemble: &Ensemble) -> ConsensusVote {
        match self.sentinel.validate_and_seal(ensemble) {
            Ok(receipt) => ConsensusVote::Pass(receipt),
            Err(e) => ConsensusVote::Kill(e),
        }
    }
}

pub struct DistributedGovernanceCluster<P: ManifoldStateProvider> {
    pub nodes: Vec<DistributedSentinelNode<P>>,
    pub quorum_threshold: usize,
}

impl<P: ManifoldStateProvider> DistributedGovernanceCluster<P> {
    pub fn new(nodes: Vec<DistributedSentinelNode<P>>, quorum_threshold: usize) -> Self {
        Self { nodes, quorum_threshold }
    }

    pub fn evaluate_consensus(&mut self, ensemble: &Ensemble) -> ConsensusResult {
        let total_nodes = self.nodes.len();
        let mut pass_votes = 0;
        let mut kill_votes = 0;
        let mut first_receipt = None;
        let mut kill_reason = String::new();

        for node in &mut self.nodes {
            let vote = node.evaluate_local(ensemble);
            match vote {
                ConsensusVote::Pass(receipt) => {
                    pass_votes += 1;
                    if first_receipt.is_none() {
                        first_receipt = Some(receipt);
                    }
                }
                ConsensusVote::Kill(reason) => {
                    kill_votes += 1;
                    kill_reason = reason;
                }
            }
        }

        let quorum_reached = pass_votes >= self.quorum_threshold;
        if quorum_reached {
            ConsensusResult {
                quorum_reached: true,
                total_nodes,
                pass_votes,
                kill_votes,
                status: "CLUSTER_PASS".to_string(),
                aggregate_receipt: first_receipt,
            }
        } else {
            ConsensusResult {
                quorum_reached: false,
                total_nodes,
                pass_votes,
                kill_votes,
                status: format!("CLUSTER_SIG_GOV_KILL: {}", kill_reason),
                aggregate_receipt: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pirtm_monitor::MockStateProvider;

    #[test]
    fn test_distributed_governance_cluster_consensus() {
        let ensemble = Ensemble::new(
            "cluster_ensemble",
            vec![vec![0.0, 0.3], vec![0.3, 0.0]],
<<<<<<< HEAD
            vec![0.9, 0.9],
        )
        .with_theorem_name("author_declared_lambda");
=======
            vec![(9, 10), (9, 10)],
        ).with_theorem_name("author_declared_lambda");
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)

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
}
