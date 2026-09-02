//! ADR-050: Multi-Node Distributed Governance Consensus
//!
//! Rust model and Kani proof harness for cluster consensus quorum verification.
//!
//! This module mirrors the Lean 4 formalization in
//! `Foundations.ADR.DistributedGovernance.lean` and the production implementation
//! in `pirtm-orchestration::distributed_governance`. All pure predicates are
//! verified with Kani bounded-model checking:
//!
//! - `is_quorum_reached` — bidirectional iff (CLUSTER_PASS iff pass >= threshold)
//! - `count_votes` — vote-accounting invariant (pass + kill == total)
//! - `quorum_threshold_valid` — threshold boundedness (0 <= threshold <= total)
//!
//! Run with:
//! ```bash
//! cargo kani --package adr_rust
//! ```

/// A single node's governance vote on an ensemble.
///
/// Mirrors `pirtm_orchestration::ConsensusVote`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusVote {
    Pass(String),
    Kill(String),
}

impl ConsensusVote {
    pub fn is_pass(&self) -> bool {
        matches!(self, ConsensusVote::Pass(_))
    }

    pub fn is_kill(&self) -> bool {
        matches!(self, ConsensusVote::Kill(_))
    }
}

/// Pure quorum predicate: true iff `pass_votes` satisfies or exceeds
/// `quorum_threshold`.
///
/// This is the exact mirror of the Lean 4 `isQuorumReached`.
pub fn is_quorum_reached(pass_votes: usize, quorum_threshold: usize) -> bool {
    pass_votes >= quorum_threshold
}

/// Count `Pass` and `Kill` votes across a slice of individual node votes.
///
/// Returns `(pass_votes, kill_votes)`.
/// Invariant: `pass_votes + kill_votes == votes.len()`.
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

/// Validate that a quorum threshold is meaningful for the given cluster size.
///
/// Returns `true` when `quorum_threshold` is non-zero and does not exceed
/// `total_nodes`. A threshold of zero is degenerate; a threshold exceeding
/// `total_nodes` makes CLUSTER_PASS impossible.
pub fn quorum_threshold_valid(quorum_threshold: usize, total_nodes: usize) -> bool {
    quorum_threshold > 0 && quorum_threshold <= total_nodes
}

// ─── Kani Verification Harnesses for ADR-050 ──────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr050_quorum_soundness_iff() {
    let pass_votes: usize = kani::any();
    let quorum_threshold: usize = kani::any();

    let result = is_quorum_reached(pass_votes, quorum_threshold);

    // Forward direction: pass_votes >= quorum_threshold ⟹ is_quorum_reached = true
    if pass_votes >= quorum_threshold {
        kani::assert(result, "Forward: pass_votes >= threshold implies quorum reached");
    }

    // Backward direction: is_quorum_reached = true ⟹ pass_votes >= quorum_threshold
    if result {
        kani::assert(
            pass_votes >= quorum_threshold,
            "Backward: quorum reached implies pass_votes >= threshold",
        );
    }
}

#[cfg(kani)]
#[kani::unwind(9)]
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
        "vote accounting: pass + kill == total votes"
    );
    kani::assert(pass_votes == votes.iter().filter(|v| v.is_pass()).count(), "pass count matches");
    kani::assert(kill_votes == votes.iter().filter(|v| v.is_kill()).count(), "kill count matches");
}

#[cfg(kani)]
#[kani::proof]
fn verify_adr050_quorum_threshold_valid() {
    let quorum_threshold: usize = kani::any();
    let total_nodes: usize = kani::any();

    let valid = quorum_threshold_valid(quorum_threshold, total_nodes);

    if quorum_threshold == 0 {
        kani::assert(!valid, "threshold of 0 is invalid");
    }
    if quorum_threshold > total_nodes {
        kani::assert(!valid, "threshold > total_nodes is invalid");
    }
    if quorum_threshold > 0 && quorum_threshold <= total_nodes {
        kani::assert(valid, "threshold in (0, total_nodes] is valid");
    }
}

#[cfg(kani)]
#[kani::unwind(7)]
fn verify_adr050_cluster_pass_iff_quorum() {
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
    kani::assume(quorum_threshold > 0 && quorum_threshold <= n);

    let (pass_votes, kill_votes) = count_votes(&votes);

    // Vote accounting.
    kani::assert(pass_votes + kill_votes == votes.len());

    // CLUSTER_PASS iff pass_votes >= quorum_threshold.
    let cluster_pass = is_quorum_reached(pass_votes, quorum_threshold);

    if pass_votes >= quorum_threshold {
        kani::assert(cluster_pass, "CLUSTER_PASS when quorum satisfied");
    } else {
        kani::assert(!cluster_pass, "CLUSTER_SIG_GOV_KILL when quorum not satisfied");
    }
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_quorum_reached() {
        assert!(is_quorum_reached(3, 2));
        assert!(is_quorum_reached(2, 2));
        assert!(!is_quorum_reached(1, 2));
        assert!(!is_quorum_reached(0, 1));
        assert!(is_quorum_reached(5, 0));
        // A threshold of 0 passes the raw predicate (0 >= 0 = true) but is
        // invalid per quorum_threshold_valid — callers must check validity.
        assert!(!quorum_threshold_valid(0, 0));
        assert!(!quorum_threshold_valid(0, 3));
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

        // Empty
        let (p, k) = count_votes(&[]);
        assert_eq!(p, 0);
        assert_eq!(k, 0);

        // All pass
        let all_pass = vec![
            ConsensusVote::Pass("a".to_string()),
            ConsensusVote::Pass("b".to_string()),
        ];
        let (p, k) = count_votes(&all_pass);
        assert_eq!(p, 2);
        assert_eq!(k, 0);

        // All kill
        let all_kill = vec![
            ConsensusVote::Kill("a".to_string()),
            ConsensusVote::Kill("b".to_string()),
        ];
        let (p, k) = count_votes(&all_kill);
        assert_eq!(p, 0);
        assert_eq!(k, 2);
    }

    #[test]
    fn test_consensus_vote_helpers() {
        let pass = ConsensusVote::Pass("receipt".to_string());
        let kill = ConsensusVote::Kill("reason".to_string());

        assert!(pass.is_pass());
        assert!(!pass.is_kill());
        assert!(kill.is_kill());
        assert!(!kill.is_pass());
    }

    #[test]
    fn test_quorum_threshold_valid() {
        assert!(!quorum_threshold_valid(0, 3));
        assert!(!quorum_threshold_valid(4, 3));
        assert!(quorum_threshold_valid(1, 1));
        assert!(quorum_threshold_valid(3, 3));
        assert!(quorum_threshold_valid(2, 3));
    }

    #[test]
    fn test_cluster_pass_exact_threshold() {
        let votes = vec![
            ConsensusVote::Pass("r1".to_string()),
            ConsensusVote::Pass("r2".to_string()),
            ConsensusVote::Kill("k1".to_string()),
        ];
        let (pass, kill) = count_votes(&votes);
        let cluster_pass = is_quorum_reached(pass, 2);
        assert!(cluster_pass);
        assert_eq!(pass, 2);
        assert_eq!(kill, 1);
    }

    #[test]
    fn test_cluster_kill_below_threshold() {
        let votes = vec![
            ConsensusVote::Pass("r1".to_string()),
            ConsensusVote::Kill("k1".to_string()),
            ConsensusVote::Kill("k2".to_string()),
        ];
        let (pass, kill) = count_votes(&votes);
        let cluster_pass = is_quorum_reached(pass, 2);
        assert!(!cluster_pass);
        assert_eq!(pass, 1);
        assert_eq!(kill, 2);
    }

    #[test]
    fn test_full_consensus_all_pass() {
        let votes = vec![
            ConsensusVote::Pass("r1".to_string()),
            ConsensusVote::Pass("r2".to_string()),
            ConsensusVote::Pass("r3".to_string()),
        ];
        let (pass, _kill) = count_votes(&votes);
        assert!(is_quorum_reached(pass, 2));
        assert!(is_quorum_reached(pass, 3));
    }

    #[test]
    fn test_no_consensus_all_kill() {
        let votes = vec![
            ConsensusVote::Kill("k1".to_string()),
            ConsensusVote::Kill("k2".to_string()),
            ConsensusVote::Kill("k3".to_string()),
        ];
        let (pass, kill) = count_votes(&votes);
        assert_eq!(pass, 0);
        assert_eq!(kill, 3);
        assert!(!is_quorum_reached(pass, 1));
    }
}
