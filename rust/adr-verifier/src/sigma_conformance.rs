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

/// Theoretical discretization error bound epsilon = 1.0 / M = 1e-3 (0.001).
pub const DISCRETIZATION_EPSILON: f64 = 1.0 / LIPSCHITZ_MODULUS as f64;

/// Computes the exact continuous floating-point SlopeUB:
/// SlopeUB(alpha, L_F) = (1.0 - alpha) + alpha * L_F
pub fn continuous_slope_ub(alpha: f64, lf: f64) -> f64 {
    let alpha_clamped = alpha.clamp(0.0, 1.0);
    (1.0 - alpha_clamped) + alpha_clamped * lf
}

/// Verifies that the discrete SlopeUB evaluated at scale M=1000 approximates
/// the continuous floating-point SlopeUB within the theoretical error bound
/// epsilon = 1.0 / M (0.001), and proves that continuous SlopeUB violations
/// (L_F >= 1.0 or continuous_slope_ub >= 1.0) remain detectable inside it.
pub fn verify_continuous_discrete_error_bound(
    alpha: f64,
    lf: f64,
) -> Result<f64, String> {
    if alpha <= 0.0 || alpha > 1.0 {
        return Err(format!("Continuous step alpha out of bounds (0, 1]: {}", alpha));
    }
    let cont_slope = continuous_slope_ub(alpha, lf);

    // Convert to discrete representation (fixed-point scaling M = 1000)
    let m = LIPSCHITZ_MODULUS;
    let alpha_disc = (alpha * m as f64).round() as u64;
    let lf_disc = (lf * m as f64).round() as u64;

    let disc_slope = compute_slope_ub(m, alpha_disc, lf_disc);
    let disc_normalized = disc_slope as f64 / m as f64;

    // Theoretical error bound: |continuous - discrete/M| <= 1.0 / M
    let err = (cont_slope - disc_normalized).abs();
    let max_allowed_err = DISCRETIZATION_EPSILON + 1e-9;
    if err > max_allowed_err {
        return Err(format!(
            "Discretization error exceeded bound: |{} - {}| = {} > {}",
            cont_slope, disc_normalized, err, max_allowed_err
        ));
    }

    // SlopeUB violation detection:
    // If continuous L_F >= 1.0, the operator is non-contractive.
    // The discrete check MUST evaluate to >= M (fail-closed detection).
    if lf >= 1.0 || cont_slope >= 1.0 {
        if disc_slope < m {
            return Err(format!(
                "CRITICAL: SlopeUB violation escaped discrete detection! continuous={}, discrete={}",
                cont_slope, disc_slope
            ));
        }
        return Err(format!(
            "SlopeUB violation detected: continuous={}, discrete={}/{}",
            cont_slope, disc_slope, m
        ));
    }

    Ok(cont_slope)
}

/// Sigma bilinear form in prime basis: σ(x, y) = Σ x_i * y_i / ln(p_i)
pub fn continuous_sigma_bilinear_form(x: &[f64], y: &[f64], primes: &[u64]) -> f64 {
    let n = x.len().min(y.len()).min(primes.len());
    let mut sum = 0.0;
    for i in 0..n {
        let log_p = (primes[i] as f64).ln();
        sum += x[i] * y[i] / log_p;
    }
    sum
}

/// Sigma norm: ||x||_σ = sqrt(σ(x, x))
pub fn continuous_sigma_norm(x: &[f64], primes: &[u64]) -> f64 {
    continuous_sigma_bilinear_form(x, x, primes).max(0.0).sqrt()
}

/// Verifies that an operator step on state vectors x and y contracts in the continuous
/// sigma-norm according to the continuous SlopeUB, and verifies that when a non-contractive
/// perturbation occurs (contraction factor >= 1.0), the discrete SlopeUB check detects it.
pub fn verify_continuous_bilinear_contraction(
    x: &[f64],
    y: &[f64],
    fx: &[f64],
    fy: &[f64],
    primes: &[u64],
    alpha: f64,
    lf: f64,
) -> Result<f64, String> {
    let m = LIPSCHITZ_MODULUS;
    let alpha_disc = (alpha * m as f64).round() as u64;
    let lf_disc = (lf * m as f64).round() as u64;

    let cont_bound = verify_continuous_discrete_error_bound(alpha, lf)?;

    let mut diff_in = Vec::with_capacity(x.len());
    for i in 0..x.len().min(y.len()) {
        diff_in.push(x[i] - y[i]);
    }
    let d_in = continuous_sigma_norm(&diff_in, primes);
    if d_in <= 1e-12 {
        return Ok(0.0);
    }

    let mut diff_out = Vec::with_capacity(fx.len());
    for i in 0..fx.len().min(fy.len()) {
        diff_out.push(fx[i] - fy[i]);
    }
    let d_out = continuous_sigma_norm(&diff_out, primes);
    let empirical_ratio = d_out / d_in;

    if empirical_ratio > cont_bound + 1e-9 {
        return Err(format!(
            "Empirical contraction ratio {} exceeds continuous SlopeUB bound {}",
            empirical_ratio, cont_bound
        ));
    }

    // Also verify discrete metric contraction
    let disc_d_in = (d_in * 1000.0).round() as u64;
    if disc_d_in > 0 {
        let disc_slope = compute_slope_ub(m, alpha_disc, lf_disc);
        let disc_d_out = (disc_slope * disc_d_in) / m;
        if disc_d_out >= disc_d_in {
            return Err(format!(
                "Discrete contraction violated: ({} * {}) / {} = {} >= {}",
                disc_slope, disc_d_in, m, disc_d_out, disc_d_in
            ));
        }
    }

    Ok(empirical_ratio)
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

    #[test]
    fn test_continuous_discrete_error_bound_grid() {
        for a_step in 1..=20 {
            let alpha = a_step as f64 * 0.05; // 0.05 to 1.0
            for lf_step in 0..=19 {
                let lf = lf_step as f64 * 0.05; // 0.0 to 0.95
                let res = verify_continuous_discrete_error_bound(alpha, lf);
                assert!(
                    res.is_ok(),
                    "Grid point alpha={}, lf={} failed: {:?}",
                    alpha,
                    lf,
                    res.err()
                );
                let cont_slope = res.unwrap();
                let m = LIPSCHITZ_MODULUS;
                let disc = compute_slope_ub(m, (alpha * m as f64).round() as u64, (lf * m as f64).round() as u64);
                let disc_norm = disc as f64 / m as f64;
                let err = (cont_slope - disc_norm).abs();
                assert!(
                    err <= DISCRETIZATION_EPSILON + 1e-9,
                    "Discretization error {} exceeded epsilon {} at alpha={}, lf={}",
                    err,
                    DISCRETIZATION_EPSILON,
                    alpha,
                    lf
                );
            }
        }
    }

    #[test]
    fn test_slope_ub_violation_detectable_by_discrete_guard() {
        let violations = [1.0, 1.001, 1.05, 1.1, 1.5, 2.0];
        for &lf in &violations {
            for a_step in 1..=10 {
                let alpha = a_step as f64 * 0.1;
                let res = verify_continuous_discrete_error_bound(alpha, lf);
                assert!(
                    res.is_err(),
                    "Non-contractive lf={} must be detected by discrete guard at alpha={}",
                    lf,
                    alpha
                );
                let err_msg = res.unwrap_err();
                assert!(
                    err_msg.contains("SlopeUB violation detected"),
                    "Expected SlopeUB violation detection, got: {}",
                    err_msg
                );
            }
        }
    }

    #[test]
    fn test_runtime_fails_outside_error_bound() {
        let primes = [2u64, 3, 5, 7, 11];
        let x = vec![1.0, 0.5, 0.2, 0.1, 0.05];
        let y = vec![0.0, 0.0, 0.0, 0.0, 0.0];

        // Expansive step where fx - fy has greater norm than allowed by SlopeUB
        let fx = vec![2.0, 1.0, 0.4, 0.2, 0.1]; // doubled distance
        let fy = vec![0.0, 0.0, 0.0, 0.0, 0.0];

        let alpha = 0.5;
        let lf = 0.8; // SlopeUB = 0.5 + 0.4 = 0.9 < 1.0
        let res = verify_continuous_bilinear_contraction(&x, &y, &fx, &fy, &primes, alpha, lf);
        assert!(
            res.is_err(),
            "Expansive step must fail continuous-discrete contraction verification"
        );
        let err = res.unwrap_err();
        assert!(
            err.contains("exceeds continuous SlopeUB bound"),
            "Expected bound excess error, got: {}",
            err
        );
    }

    #[test]
    fn test_continuous_bilinear_norm_conformance() {
        let primes = [2u64, 3, 5, 7, 11];
        let x = vec![1.0, 0.8, 0.6, 0.4, 0.2];
        let y = vec![0.2, 0.4, 0.6, 0.8, 1.0];

        // Contractive step: F(v) = 0.7 * v
        let fx: Vec<f64> = x.iter().map(|&v| 0.7 * v).collect();
        let fy: Vec<f64> = y.iter().map(|&v| 0.7 * v).collect();

        let alpha = 0.6;
        let lf = 0.5; // SlopeUB = 0.4 + 0.3 = 0.7
        let res = verify_continuous_bilinear_contraction(&x, &y, &fx, &fy, &primes, alpha, lf);
        assert!(
            res.is_ok(),
            "Contractive step should pass continuous verification: {:?}",
            res.err()
        );
        let ratio = res.unwrap();
        assert!((ratio - 0.7).abs() < 1e-9, "Contraction ratio should equal 0.7: {}", ratio);
    }
}
