//! ADR-034: Prime-Indexed Dialectical Semantics & Contestation Fields
//!
//! Rust implementation and Kani model checking harness for ADR-034:
//! - Gate-based certification pipeline (Grounding, Robustness, Dialectical Non-Collapse).
//! - Typed rejection states.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationRejection {
    GroundingFailure(&'static str),
    RobustnessFailure(&'static str),
    DialecticalCollapse(&'static str),
}

#[derive(Debug, Clone)]
pub struct GroundingMetrics {
    pub coverage_ratio: u32, // 0..100
    pub min_threshold: u32,
}

#[derive(Debug, Clone)]
pub struct DialecticalTension {
    pub tension_delta: u32,
    pub max_allowed: u32,
    pub branch_count: u32,
}

#[derive(Debug, Clone)]
pub struct CandidateTrajectory {
    pub id: u64,
    pub grounding: GroundingMetrics,
    pub tension: DialecticalTension,
    pub contractivity_scaled: u32, // k * 100 < 100
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationResult {
    Admissible,
    Rejected(CertificationRejection),
}

pub fn evaluate_grounding_gate(c: &CandidateTrajectory) -> Option<CertificationRejection> {
    if c.grounding.coverage_ratio >= c.grounding.min_threshold {
        None
    } else {
        Some(CertificationRejection::GroundingFailure("Grounding ratio below minimum threshold"))
    }
}

pub fn evaluate_robustness_gate(c: &CandidateTrajectory) -> Option<CertificationRejection> {
    if c.contractivity_scaled < 100 {
        None
    } else {
        Some(CertificationRejection::RobustnessFailure("Trajectory violates contractivity bound k < 1"))
    }
}

pub fn evaluate_dialectical_gate(c: &CandidateTrajectory) -> Option<CertificationRejection> {
    if c.tension.tension_delta <= c.tension.max_allowed && c.tension.branch_count > 1 {
        None
    } else if c.tension.branch_count <= 1 {
        Some(CertificationRejection::DialecticalCollapse("Dialectical pluralism collapsed to single branch"))
    } else {
        Some(CertificationRejection::DialecticalCollapse("Dialectical tension exceeds stability bound"))
    }
}

pub fn certify_trajectory(c: &CandidateTrajectory) -> CertificationResult {
    if let Some(err) = evaluate_grounding_gate(c) {
        return CertificationResult::Rejected(err);
    }
    if let Some(err) = evaluate_robustness_gate(c) {
        return CertificationResult::Rejected(err);
    }
    if let Some(err) = evaluate_dialectical_gate(c) {
        return CertificationResult::Rejected(err);
    }
    CertificationResult::Admissible
}

// ─── Kani Verification Harnesses for ADR-034 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr034_admissible_invariants() {
    let coverage_ratio: u32 = kani::any();
    let min_threshold: u32 = kani::any();
    let tension_delta: u32 = kani::any();
    let max_allowed: u32 = kani::any();
    let branch_count: u32 = kani::any();
    let contractivity_scaled: u32 = kani::any();

    kani::assume(coverage_ratio <= 100);
    kani::assume(min_threshold <= 100);
    kani::assume(contractivity_scaled <= 200);
    kani::assume(branch_count <= 10);
    kani::assume(tension_delta <= 100);
    kani::assume(max_allowed <= 100);

    let trajectory = CandidateTrajectory {
        id: 1,
        grounding: GroundingMetrics {
            coverage_ratio,
            min_threshold,
        },
        tension: DialecticalTension {
            tension_delta,
            max_allowed,
            branch_count,
        },
        contractivity_scaled,
    };

    let result = certify_trajectory(&trajectory);
    if result == CertificationResult::Admissible {
        assert!(coverage_ratio >= min_threshold);
        assert!(contractivity_scaled < 100);
        assert!(tension_delta <= max_allowed);
        assert!(branch_count > 1);
    }
}

#[cfg(kani)]
#[kani::proof]
fn verify_adr034_branch_collapse_rejected() {
    let branch_count: u32 = kani::any();
    kani::assume(branch_count <= 1);

    let trajectory = CandidateTrajectory {
        id: 2,
        grounding: GroundingMetrics {
            coverage_ratio: 90,
            min_threshold: 70,
        },
        tension: DialecticalTension {
            tension_delta: 10,
            max_allowed: 20,
            branch_count,
        },
        contractivity_scaled: 80,
    };

    let result = certify_trajectory(&trajectory);
    assert_eq!(
        result,
        CertificationResult::Rejected(CertificationRejection::DialecticalCollapse(
            "Dialectical pluralism collapsed to single branch"
        ))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admissible_trajectory() {
        let t = CandidateTrajectory {
            id: 101,
            grounding: GroundingMetrics { coverage_ratio: 85, min_threshold: 70 },
            tension: DialecticalTension { tension_delta: 10, max_allowed: 20, branch_count: 3 },
            contractivity_scaled: 90,
        };
        assert_eq!(certify_trajectory(&t), CertificationResult::Admissible);
    }

    #[test]
    fn test_branch_collapse_rejection() {
        let t = CandidateTrajectory {
            id: 102,
            grounding: GroundingMetrics { coverage_ratio: 85, min_threshold: 70 },
            tension: DialecticalTension { tension_delta: 10, max_allowed: 20, branch_count: 1 },
            contractivity_scaled: 90,
        };
        assert_eq!(
            certify_trajectory(&t),
            CertificationResult::Rejected(CertificationRejection::DialecticalCollapse(
                "Dialectical pluralism collapsed to single branch"
            ))
        );
    }
}
