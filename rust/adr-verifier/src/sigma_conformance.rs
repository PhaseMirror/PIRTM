//! SigmaKernel Lipschitz Contraction & ACE/SlopeUB Conformance Module
//!
//! Directly mirrors and witnesses `Foundations/PhaseMirror/SigmaKernel.lean`:
//! - Modulus M = 1000 (fixed-point representation of unit Lipschitz constant 1.0)
//! - compute_slope_ub(M, alpha, lf) = (M - min(alpha, M)) + (min(alpha, M) * lf) / M
//! - compute_gap_lb(M, alpha, lf) = M - compute_slope_ub(M, alpha, lf)
//! - AceSlopeUBCondition: alpha > 0 && alpha <= M && lf < M
//! - SpectralState: effective_lipschitz, drift, tau_r
//! - SigmaKernelInvariant: effective_lipschitz < M && drift <= tau_r
//!
//! Validates the Lean theorem statement `ace_slope_ub_implies_sigma_contraction`
//! as an operational runtime invariant.

use serde::{Deserialize, Serialize};

/// Fixed-point scaling modulus representing 1.0 (unit Lipschitz bound).
pub const LIPSCHITZ_MODULUS: u64 = 1000;

/// Prerequisite condition for ACE / SlopeUB contraction:
/// 1. Step size `alpha` > 0
/// 2. Step size `alpha` <= M
/// 3. Underlying operator Lipschitz bound `lf` < M
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AceSlopeUBCondition {
    pub alpha: u64,
    pub lf: u64,
}

impl AceSlopeUBCondition {
    pub fn new(alpha: u64, lf: u64) -> Self {
        Self { alpha, lf }
    }

    pub fn is_valid(&self, m: u64) -> bool {
        self.alpha > 0 && self.alpha <= m && self.lf < m
    }
}

/// Computes the end-to-end SlopeUB (Lipschitz upper bound) from step size `alpha`
/// and component bound `lf` under modulus `M`.
/// Matches `PhaseMirror.computeSlopeUB` in `SigmaKernel.lean`.
pub fn compute_slope_ub(m: u64, alpha: u64, lf: u64) -> u64 {
    let alpha_clamped = alpha.min(m);
    (m - alpha_clamped) + (alpha_clamped * lf) / m
}

/// Computes the spectral gap lower bound: GapLB = M - SlopeUB.
/// Matches `PhaseMirror.computeGapLB` in `SigmaKernel.lean`.
pub fn compute_gap_lb(m: u64, alpha: u64, lf: u64) -> u64 {
    m.saturating_sub(compute_slope_ub(m, alpha, lf))
}

/// Spectral state representation for MOC and ADR-062 compliance.
/// Matches `PhaseMirror.SpectralState` in `SigmaKernel.lean`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpectralState {
    pub effective_lipschitz: u64,
    pub drift: u64,
    pub tau_r: u64,
}

impl SpectralState {
    /// Invariant check: L_eff < LIPSCHITZ_MODULUS (1000) and drift <= tau_r.
    /// Matches `PhaseMirror.SigmaKernelInvariant` in `SigmaKernel.lean`.
    pub fn is_invariant_satisfied(&self) -> bool {
        self.effective_lipschitz < LIPSCHITZ_MODULUS && self.drift <= self.tau_r
    }
}

/// Verifies that a pair (alpha, lf) satisfies the ACE condition and strictly contracts
/// on all given positive sigma-metric distances `d`.
/// Matches the conclusion of `ace_slope_ub_implies_sigma_contraction`.
pub fn verify_slope_ub_contraction(
    m: u64,
    alpha: u64,
    lf: u64,
    test_distances: &[u64],
) -> Result<u64, String> {
    let cond = AceSlopeUBCondition::new(alpha, lf);
    if !cond.is_valid(m) {
        return Err(format!(
            "ACE condition violated: alpha={}, lf={}, m={}",
            alpha, lf, m
        ));
    }

    let l_sigma = compute_slope_ub(m, alpha, lf);
    let gap_lb = compute_gap_lb(m, alpha, lf);

    if l_sigma >= m {
        return Err(format!(
            "Contraction violated: L_sigma={} >= M={}",
            l_sigma, m
        ));
    }
    if gap_lb == 0 {
        return Err(format!("Spectral gap violated: GapLB=0"));
    }

    for &d in test_distances {
        if d > 0 {
            let contracted = (l_sigma * d) / m;
            if contracted >= d {
                return Err(format!(
                    "Metric contraction violated at distance {}: ({} * {}) / {} = {} >= {}",
                    d, l_sigma, d, m, contracted, d
                ));
            }
        }
    }

    Ok(l_sigma)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DISTANCES: [u64; 9] = [1, 2, 5, 10, 50, 100, 500, 1000, 10000];

    #[test]
    fn test_ace_condition_boundaries() {
        let m = LIPSCHITZ_MODULUS;

        // Valid boundary points
        assert!(AceSlopeUBCondition::new(1, 0).is_valid(m));
        assert!(AceSlopeUBCondition::new(1, 999).is_valid(m));
        assert!(AceSlopeUBCondition::new(1000, 0).is_valid(m));
        assert!(AceSlopeUBCondition::new(1000, 999).is_valid(m));
        assert!(AceSlopeUBCondition::new(500, 500).is_valid(m));

        // Invalid: alpha == 0
        assert!(!AceSlopeUBCondition::new(0, 500).is_valid(m));
        // Invalid: alpha > M
        assert!(!AceSlopeUBCondition::new(1001, 500).is_valid(m));
        // Invalid: lf >= M
        assert!(!AceSlopeUBCondition::new(500, 1000).is_valid(m));
        assert!(!AceSlopeUBCondition::new(500, 1200).is_valid(m));
    }

    #[test]
    fn test_slope_ub_formula_values() {
        let m = LIPSCHITZ_MODULUS; // 1000

        // alpha = 100, lf = 500:
        // SlopeUB = (1000 - 100) + (100 * 500) / 1000 = 900 + 50 = 950
        assert_eq!(compute_slope_ub(m, 100, 500), 950);
        assert_eq!(compute_gap_lb(m, 100, 500), 50);

        // alpha = 1000, lf = 999:
        // SlopeUB = (1000 - 1000) + (1000 * 999) / 1000 = 0 + 999 = 999
        assert_eq!(compute_slope_ub(m, 1000, 999), 999);
        assert_eq!(compute_gap_lb(m, 1000, 999), 1);

        // alpha = 1, lf = 0:
        // SlopeUB = (1000 - 1) + (1 * 0) / 1000 = 999 + 0 = 999
        assert_eq!(compute_slope_ub(m, 1, 0), 999);
        assert_eq!(compute_gap_lb(m, 1, 0), 1);

        // alpha = 1000, lf = 0:
        // SlopeUB = 0 + 0 = 0
        assert_eq!(compute_slope_ub(m, 1000, 0), 0);
        assert_eq!(compute_gap_lb(m, 1000, 0), 1000);
    }

    #[test]
    fn test_lean_theorem_conformance_on_grid() {
        let m = LIPSCHITZ_MODULUS;
        // Test grid of (alpha, lf) covering all quadrants of the valid space
        let alphas = [1, 50, 100, 250, 500, 750, 900, 999, 1000];
        let lfs = [0, 1, 100, 250, 500, 750, 900, 998, 999];

        for &alpha in &alphas {
            for &lf in &lfs {
                let res = verify_slope_ub_contraction(m, alpha, lf, &TEST_DISTANCES);
                assert!(
                    res.is_ok(),
                    "Theorem conformance failed for alpha={}, lf={}: {:?}",
                    alpha,
                    lf,
                    res.err()
                );
                let l_sigma = res.unwrap();
                assert!(
                    l_sigma < m,
                    "L_sigma {} must be strictly less than M {}",
                    l_sigma,
                    m
                );

                // Verify induced spectral state satisfies invariant
                let state = SpectralState {
                    effective_lipschitz: l_sigma,
                    drift: 10,
                    tau_r: 20,
                };
                assert!(state.is_invariant_satisfied());
            }
        }
    }

    #[test]
    fn test_drift_exceeded_fails_invariant() {
        let state_pass = SpectralState {
            effective_lipschitz: 950,
            drift: 15,
            tau_r: 20,
        };
        assert!(state_pass.is_invariant_satisfied());

        let state_drift_fail = SpectralState {
            effective_lipschitz: 950,
            drift: 25,
            tau_r: 20,
        };
        assert!(!state_drift_fail.is_invariant_satisfied());

        let state_lip_fail = SpectralState {
            effective_lipschitz: 1000,
            drift: 5,
            tau_r: 20,
        };
        assert!(!state_lip_fail.is_invariant_satisfied());
    }
}
