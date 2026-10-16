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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpectralState {
    pub effective_lipschitz: u64,
    pub drift: u64,
    pub tau_r: u64,
    #[serde(default)]
    pub support_size_n: u64,
    #[serde(default = "default_kappa")]
    pub kappa: f64,
    #[serde(default)]
    pub residual_min: f64,
    #[serde(default = "default_true")]
    pub residual_cert_valid: bool,
}

fn default_kappa() -> f64 {
    1.0
}

fn default_true() -> bool {
    true
}

impl Default for SpectralState {
    fn default() -> Self {
        Self {
            effective_lipschitz: 0,
            drift: 0,
            tau_r: 100,
            support_size_n: 0,
            kappa: 1.0,
            residual_min: 0.0,
            residual_cert_valid: true,
        }
    }
}

impl SpectralState {
    pub fn new(effective_lipschitz: u64, drift: u64, tau_r: u64) -> Self {
        Self {
            effective_lipschitz,
            drift,
            tau_r,
            support_size_n: 0,
            kappa: 1.0,
            residual_min: 0.0,
            residual_cert_valid: true,
        }
    }

    pub fn with_spectral_bound(
        effective_lipschitz: u64,
        drift: u64,
        tau_r: u64,
        support_size_n: u64,
        kappa: f64,
    ) -> Self {
        Self {
            effective_lipschitz,
            drift,
            tau_r,
            support_size_n,
            kappa,
            residual_min: 0.0,
            residual_cert_valid: true,
        }
    }

    pub fn with_certified_residual(
        effective_lipschitz: u64,
        drift: u64,
        tau_r: u64,
        support_size_n: u64,
        kappa: f64,
        residual_min: f64,
        residual_cert_valid: bool,
    ) -> Self {
        Self {
            effective_lipschitz,
            drift,
            tau_r,
            support_size_n,
            kappa,
            residual_min,
            residual_cert_valid,
        }
    }

    pub fn is_margin_safe(&self, m: u64) -> bool {
        if self.support_size_n == 0 {
            return true;
        }
        let margin = m.saturating_sub(self.effective_lipschitz) as f64;
        let gamma_n = compute_gamma_n(self.support_size_n as usize);
        let perturbation = (m as f64) * gamma_n * self.kappa;
        margin > perturbation
    }

    /// Invariant check: L_eff < LIPSCHITZ_MODULUS (1000) and drift <= tau_r,
    /// discrete margin strictly exceeds floating-point perturbation bound,
    /// and power-iteration residual certificate is valid.
    /// Matches `PhaseMirror.SigmaKernelInvariant` in `SigmaKernel.lean`.
    pub fn is_invariant_satisfied(&self) -> bool {
        self.effective_lipschitz < LIPSCHITZ_MODULUS
            && self.drift <= self.tau_r
            && self.is_margin_safe(LIPSCHITZ_MODULUS)
            && self.residual_cert_valid
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

/// IEEE-754 unit roundoff for binary64 (f64): u = 2^-53
pub const UNIT_ROUNDOFF_F64: f64 = 1.1102230246251565e-16;

/// Baseline minimum eigenvalue gap floor: epsilon_0 = u / M = 2^-53 / 1000
pub const BASELINE_EPSILON_GAP: f64 = UNIT_ROUNDOFF_F64 / LIPSCHITZ_MODULUS as f64;

/// Computes the condition-number-scaled rejection threshold:
/// lambda_min(G) < (2^-53 / 1000) * (N * 2^-53 / (1 - N * 2^-53)) * kappa
/// Incorporates stored kappa so ill-conditioned matrices receive proportionally heightened gap floors.
pub fn compute_scaled_rejection_threshold(n: usize, stored_kappa: Option<f64>) -> f64 {
    let k = stored_kappa.unwrap_or(1.0).max(1.0);
    BASELINE_EPSILON_GAP * compute_gamma_n(n) * k
}

/// Default minimum eigenvalue threshold to prevent condition number divergence
pub const MIN_EIGENVALUE_EPSILON: f64 = 1e-8;

/// Residual tolerance for power iteration convergence check
pub const POWER_ITERATION_RESIDUAL_TOL: f64 = 1e-6;

/// Maximum iterations for power iteration
pub const POWER_ITERATION_MAX_ITERS: usize = 200;

/// Computes γ_N = N * u / (1 - N * u) under IEEE-754 f64 rounding
pub fn compute_gamma_n(n: usize) -> f64 {
    let nu = n as f64 * UNIT_ROUNDOFF_F64;
    if nu >= 1.0 {
        f64::INFINITY
    } else {
        nu / (1.0 - nu)
    }
}

/// Computes the Gram matrix G_ij = Σ_p ν_p^(i) * ν_p^(j) * (ln p)^-1
/// for a collection of basis vectors over prime support.
pub fn compute_gram_matrix(vectors: &[Vec<f64>], primes: &[u64]) -> Result<Vec<Vec<f64>>, String> {
    if vectors.is_empty() {
        return Err("Cannot compute Gram matrix: vector set is empty".into());
    }
    let m = vectors.len();
    let n = primes.len();
    for (idx, v) in vectors.iter().enumerate() {
        if v.len() < n {
            return Err(format!(
                "Vector {} length {} < prime support size {}",
                idx, v.len(), n
            ));
        }
    }

    let mut gram = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in i..m {
            let mut sum = 0.0;
            for p_idx in 0..n {
                let log_p = (primes[p_idx] as f64).ln();
                sum += vectors[i][p_idx] * vectors[j][p_idx] / log_p;
            }
            gram[i][j] = sum;
            gram[j][i] = sum;
        }
    }
    Ok(gram)
}

/// Helper: Matrix-vector product for square matrix G and vector v
fn mat_vec_mul(mat: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
    let m = mat.len();
    let mut out = vec![0.0; m];
    for i in 0..m {
        let mut s = 0.0;
        for j in 0..m {
            s += mat[i][j] * v[j];
        }
        out[i] = s;
    }
    out
}

/// Helper: Vector 2-norm
fn vec_norm(v: &[f64]) -> f64 {
    v.iter().map(|&x| x * x).sum::<f64>().sqrt()
}

/// A-posteriori certificate produced by power iteration / shifted power iteration.
/// Formally matched to `PhaseMirror.PowerIterationCertificate` in `SigmaKernel.lean`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PowerIterationCertificate {
    pub lambda_est: f64,
    pub residual: f64,
    pub residual_tol: f64,
    pub epsilon_min: f64,
    pub certified_lambda_lb: f64,
}

impl PowerIterationCertificate {
    pub fn new(lambda_est: f64, residual: f64, residual_tol: f64, epsilon_min: f64) -> Self {
        let certified_lambda_lb = (lambda_est - residual).max(0.0);
        Self {
            lambda_est,
            residual,
            residual_tol,
            epsilon_min,
            certified_lambda_lb,
        }
    }

    /// Verifies the certificate against the Lean decision predicate:
    /// 1. residual <= residual_tol (iteration converged within tolerance)
    /// 2. residual < lambda_est (uncertainty strictly smaller than eigenvalue, keeping spectrum away from zero)
    /// 3. certified_lambda_lb >= epsilon_min (certified true eigenvalue satisfies spectral gap)
    /// 4. epsilon_min > 0.0 (strictly positive gap floor)
    pub fn verify(&self) -> Result<f64, String> {
        if self.epsilon_min <= 0.0 {
            return Err(format!(
                "Invalid epsilon_min <= 0.0: {:.4e}",
                self.epsilon_min
            ));
        }
        if self.residual > self.residual_tol {
            return Err(format!(
                "Power iteration residual {:.4e} > tolerance {:.4e}: iteration failed to converge",
                self.residual, self.residual_tol
            ));
        }
        if self.residual >= self.lambda_est {
            return Err(format!(
                "Power iteration residual {:.4e} >= lambda_est {:.4e}: uncertainty exceeds spectral distance to zero",
                self.residual, self.lambda_est
            ));
        }
        if self.certified_lambda_lb < self.epsilon_min {
            return Err(format!(
                "Certified eigenvalue lower bound {:.4e} < epsilon_min {:.4e}: spectral gap collapse",
                self.certified_lambda_lb, self.epsilon_min
            ));
        }
        Ok(self.certified_lambda_lb)
    }

    pub fn is_valid(&self) -> bool {
        self.verify().is_ok()
    }
}

/// Computes the spectral gap (lambda_min, lambda_max, condition number kappa, PowerIterationCertificate)
/// of a symmetric Gram matrix using power iteration and shifted power iteration, verifying the residual
/// ||G v - λ v|| <= 1e-6. Enforces scaled minimum eigenvalue gap lambda_min >= threshold(N, kappa) = (2^-53 / 1000) * gamma_N * kappa
/// and requires the power-iteration residual to be strictly smaller than the gap.
pub fn compute_gram_spectral_gap_certified(
    gram: &[Vec<f64>],
    support_size_n: Option<usize>,
    stored_kappa: Option<f64>,
    epsilon_min: Option<f64>,
) -> Result<(f64, f64, f64, PowerIterationCertificate), String> {
    let m = gram.len();
    if m == 0 {
        return Err("Gram matrix is empty".into());
    }
    let n = support_size_n.unwrap_or(m);
    let scaled_threshold = compute_scaled_rejection_threshold(n, stored_kappa);
    let eps = epsilon_min.unwrap_or(scaled_threshold);

    if m == 1 {
        let val = gram[0][0];
        let cert = PowerIterationCertificate::new(val, 0.0, POWER_ITERATION_RESIDUAL_TOL, eps);
        cert.verify()?;
        return Ok((val, val, 1.0, cert));
    }

    if m == 2 {
        let a = gram[0][0];
        let b = gram[0][1];
        let d = gram[1][1];
        let trace = a + d;
        let det = a * d - b * b;
        let disc = (trace * trace - 4.0 * det).max(0.0).sqrt();
        let lambda_max = (trace + disc) / 2.0;
        let lambda_min = (trace - disc) / 2.0;

        let cert = PowerIterationCertificate::new(lambda_min, 0.0, POWER_ITERATION_RESIDUAL_TOL, eps);
        cert.verify()?;
        let kappa = lambda_max / cert.certified_lambda_lb;
        return Ok((lambda_min, lambda_max, kappa, cert));
    }

    // General m >= 3: Power iteration for lambda_max
    let mut v = vec![1.0 / (m as f64).sqrt(); m];
    let mut lambda_max = 0.0;
    for _ in 0..POWER_ITERATION_MAX_ITERS {
        let w = mat_vec_mul(gram, &v);
        let norm = vec_norm(&w);
        if norm < 1e-15 {
            return Err("Singular Gram matrix: all eigenvalues vanish".into());
        }
        for i in 0..m {
            v[i] = w[i] / norm;
        }
        let gv = mat_vec_mul(gram, &v);
        lambda_max = v.iter().zip(gv.iter()).map(|(&a, &b)| a * b).sum();
    }
    // Verify residual for lambda_max
    let gv = mat_vec_mul(gram, &v);
    let mut diff_max = vec![0.0; m];
    for i in 0..m {
        diff_max[i] = gv[i] - lambda_max * v[i];
    }
    let res_max = vec_norm(&diff_max);
    if res_max > POWER_ITERATION_RESIDUAL_TOL {
        return Err(format!(
            "Power iteration failed to converge for lambda_max: residual {:.4e} > tol {:.4e}",
            res_max, POWER_ITERATION_RESIDUAL_TOL
        ));
    }

    // Shifted power iteration on B = lambda_max * I - G for lambda_min
    let mut b_mat = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let delta = if i == j { lambda_max } else { 0.0 };
            b_mat[i][j] = delta - gram[i][j];
        }
    }

    let mut u: Vec<f64> = (0..m).map(|i| if i % 2 == 0 { 1.0 } else { -1.0 }).collect();
    let u_norm = vec_norm(&u);
    for x in u.iter_mut() {
        *x /= u_norm;
    }

    let mut mu_max = 0.0;
    for _ in 0..POWER_ITERATION_MAX_ITERS {
        let w_b = mat_vec_mul(&b_mat, &u);
        let norm_b = vec_norm(&w_b);
        if norm_b < 1e-15 {
            // All eigenvalues of G are identical to lambda_max
            mu_max = 0.0;
            break;
        }
        for i in 0..m {
            u[i] = w_b[i] / norm_b;
        }
        let bu = mat_vec_mul(&b_mat, &u);
        mu_max = u.iter().zip(bu.iter()).map(|(&a, &b)| a * b).sum();
    }

    let lambda_min = (lambda_max - mu_max).max(0.0);
    // Verify residual for lambda_min
    let gu = mat_vec_mul(gram, &u);
    let mut diff_min = vec![0.0; m];
    for i in 0..m {
        diff_min[i] = gu[i] - lambda_min * u[i];
    }
    let res_min = vec_norm(&diff_min);

    let cert = PowerIterationCertificate::new(lambda_min, res_min, POWER_ITERATION_RESIDUAL_TOL, eps);
    cert.verify()?;

    let kappa = lambda_max / cert.certified_lambda_lb;
    Ok((lambda_min, lambda_max, kappa, cert))
}

/// Computes the spectral gap (lambda_min, lambda_max, condition number kappa) of a symmetric Gram matrix
/// using power iteration and shifted power iteration, verifying the residual ||G v - λ v|| <= 1e-6.
/// Enforces scaled minimum eigenvalue gap lambda_min >= threshold(N, kappa) = (2^-53 / 1000) * gamma_N * kappa
/// and requires the power-iteration residual to be strictly smaller than the gap.
pub fn compute_gram_spectral_gap(
    gram: &[Vec<f64>],
    support_size_n: Option<usize>,
    stored_kappa: Option<f64>,
    epsilon_min: Option<f64>,
) -> Result<(f64, f64, f64), String> {
    let (lambda_min, lambda_max, kappa, _cert) =
        compute_gram_spectral_gap_certified(gram, support_size_n, stored_kappa, epsilon_min)?;
    Ok((lambda_min, lambda_max, kappa))
}

/// Verifies that the discrete margin (M - SlopeUB) strictly exceeds the floating-point
/// perturbation bound 1000 * γ_N * κ, ensuring that SlopeUB violations remain strictly
/// detectable inside numerical uncertainty. Fails closed when margin <= perturbation.
pub fn verify_discrete_margin_guard(
    m: u64,
    alpha: u64,
    lf: u64,
    support_size_n: usize,
    kappa: f64,
) -> Result<f64, String> {
    if kappa < 1.0 {
        return Err(format!("Invalid condition number kappa < 1.0: {}", kappa));
    }
    let l_sigma = compute_slope_ub(m, alpha, lf);
    let discrete_margin = m.saturating_sub(l_sigma) as f64;

    let gamma_n = compute_gamma_n(support_size_n);
    let perturbation = (m as f64) * gamma_n * kappa;

    if discrete_margin <= perturbation {
        return Err(format!(
            "Discrete margin {:.6e} <= computed perturbation bound {:.6e} (1000 * gamma_{} * {:.2}): SlopeUB violation undetectable inside numerical uncertainty",
            discrete_margin, perturbation, support_size_n, kappa
        ));
    }

    Ok(perturbation)
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
                let state = SpectralState::new(l_sigma, 10, 20);
                assert!(state.is_invariant_satisfied());
            }
        }
    }

    #[test]
    fn test_drift_exceeded_fails_invariant() {
        let state_pass = SpectralState::new(950, 15, 20);
        assert!(state_pass.is_invariant_satisfied());

        let state_drift_fail = SpectralState::new(950, 25, 20);
        assert!(!state_drift_fail.is_invariant_satisfied());

        let state_lip_fail = SpectralState::new(1000, 5, 20);
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

    #[test]
    fn test_gram_matrix_computation() {
        let primes = [2u64, 3, 5];
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![0.5, 1.0, 1.5];
        let gram = compute_gram_matrix(&[v1.clone(), v2.clone()], &primes).unwrap();
        assert_eq!(gram.len(), 2);
        assert_eq!(gram[0].len(), 2);

        // Check symmetry
        assert_eq!(gram[0][1], gram[1][0]);

        // G_00 = v1 · v1 / ln(p)
        let expected_g00 = 1.0 / 2.0f64.ln() + 4.0 / 3.0f64.ln() + 9.0 / 5.0f64.ln();
        assert!((gram[0][0] - expected_g00).abs() < 1e-12);
    }

    #[test]
    fn test_gram_spectral_gap_power_iteration() {
        let primes = [2u64, 3, 5];
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        let gram = compute_gram_matrix(&[v1, v2], &primes).unwrap();

        let (lambda_min, lambda_max, kappa) = compute_gram_spectral_gap(&gram, Some(3), None, None).unwrap();
        assert!(lambda_min > 0.0);
        assert!(lambda_max >= lambda_min);
        assert!(kappa >= 1.0);
        let expected_min = 1.0 / 3.0f64.ln();
        let expected_max = 1.0 / 2.0f64.ln();
        assert!((lambda_min - expected_min).abs() < 1e-6);
        assert!((lambda_max - expected_max).abs() < 1e-6);
    }

    #[test]
    fn test_eigenvalue_collapse_rejection() {
        // Linearly dependent vectors create singular Gram matrix (det = 0, lambda_min = 0)
        let primes = [2u64, 3, 5];
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![2.0, 4.0, 6.0]; // Exactly 2 * v1
        let gram = compute_gram_matrix(&[v1, v2], &primes).unwrap();

        let res = compute_gram_spectral_gap(&gram, Some(3), None, None);
        assert!(res.is_err(), "Linearly dependent vectors must be rejected");
        let err = res.unwrap_err();
        assert!(
            err.contains("eigenvalue collapse") || err.contains("uncertainty") || err.contains("diverges"),
            "Expected eigenvalue collapse or uncertainty error, got: {}",
            err
        );
    }

    #[test]
    fn test_scaled_rejection_threshold_formula() {
        let n = 5;
        let threshold = compute_scaled_rejection_threshold(n, None);
        let u = 2.0f64.powi(-53);
        let expected = (u / 1000.0) * (n as f64 * u / (1.0 - n as f64 * u));
        assert!((threshold - expected).abs() < 1e-45);
        assert!(threshold > 0.0);
    }

    #[test]
    fn test_scaled_rejection_threshold_with_kappa() {
        let n = 5;
        let u = 2.0f64.powi(-53);
        let gamma_5 = 5.0 * u / (1.0 - 5.0 * u);
        let base = u / 1000.0;

        // Baseline (kappa = 1.0)
        let thresh_1 = compute_scaled_rejection_threshold(n, None);
        assert!((thresh_1 - base * gamma_5).abs() < 1e-45);

        // Scaled by kappa = 100.0: threshold = (2^-53 / 1000) * gamma_N * kappa
        let kappa = 100.0;
        let thresh_k = compute_scaled_rejection_threshold(n, Some(kappa));
        assert!((thresh_k - base * gamma_5 * kappa).abs() < 1e-45);
        assert!(thresh_k > thresh_1);
        assert!((thresh_k - thresh_1 * 100.0).abs() < 1e-45);
    }

    #[test]
    fn test_ill_conditioned_support_rejected_by_kappa_scaled_threshold() {
        let primes = [2u64, 3];
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.5, 0.5];
        let gram = compute_gram_matrix(&[v1, v2], &primes).unwrap();

        // Baseline passes with normal kappa = 1.0
        let res_base = compute_gram_spectral_gap_certified(&gram, Some(2), Some(1.0), None);
        assert!(res_base.is_ok(), "Baseline should pass: {:?}", res_base.err());

        // Under an extreme stored kappa = 1e36, the rejection threshold (2^-53/1000)*gamma_N*kappa
        // scales up to exceed the computed eigenvalue gap, rejecting the transition
        let res_ill = compute_gram_spectral_gap_certified(&gram, Some(2), Some(1e36), None);
        assert!(res_ill.is_err(), "Extreme stored kappa must abort on compressed gap");
        let err = res_ill.unwrap_err();
        assert!(err.contains("spectral gap collapse") || err.contains("uncertainty"));
    }

    #[test]
    fn test_margin_guard_rejection_when_margin_le_perturbation() {
        let m = LIPSCHITZ_MODULUS; // 1000
        // When alpha = 1000, lf = 1000:
        // SlopeUB = 1000, discrete margin = 0
        let res = verify_discrete_margin_guard(m, 1000, 1000, 5, 2.0);
        assert!(res.is_err(), "Zero margin must fail margin guard");
        let err = res.unwrap_err();
        assert!(err.contains("SlopeUB violation undetectable inside numerical uncertainty"));

        // Artificially large kappa where perturbation exceeds discrete margin
        // discrete margin = 1000 - 950 = 50
        // gamma_5 = 5 * 2^-53 / (1 - 5 * 2^-53) ≈ 5.55e-16
        // If kappa = 1e17, perturbation ≈ 1000 * 5.55e-16 * 1e17 ≈ 55.5 > 50
        let res_large_kappa = verify_discrete_margin_guard(m, 100, 500, 5, 1e17);
        assert!(res_large_kappa.is_err(), "Perturbation exceeding margin must be rejected");
    }

    #[test]
    fn test_margin_guard_acceptance_and_spectral_state_invariant() {
        let m = LIPSCHITZ_MODULUS;
        // alpha = 500, lf = 800:
        // SlopeUB = 500 + 400 = 900, discrete margin = 100
        // gamma_5 ≈ 5.55e-16, kappa = 2.0 => perturbation ≈ 1.11e-12 << 100
        let res = verify_discrete_margin_guard(m, 500, 800, 5, 2.0);
        assert!(res.is_ok(), "Safe margin must pass guard: {:?}", res.err());

        let state = SpectralState::with_spectral_bound(900, 10, 50, 5, 2.0);
        assert!(state.is_margin_safe(m));
        assert!(state.is_invariant_satisfied());

        // State with margin = 0 fails
        let state_zero_margin = SpectralState::with_spectral_bound(1000, 10, 50, 5, 2.0);
        assert!(!state_zero_margin.is_margin_safe(m));
        assert!(!state_zero_margin.is_invariant_satisfied());
    }

    #[test]
    fn test_power_iteration_certificate_validation_rules() {
        // 1. Valid certificate passes
        let cert_valid = PowerIterationCertificate::new(0.5, 1e-7, 1e-6, 1e-8);
        assert!(cert_valid.is_valid());
        assert_eq!(cert_valid.verify().unwrap(), 0.5 - 1e-7);

        // 2. Residual exceeding tolerance is rejected (matches power_iteration_residual_tol_exceeded_rejected)
        let cert_tol_fail = PowerIterationCertificate::new(0.5, 5e-6, 1e-6, 1e-8);
        assert!(!cert_tol_fail.is_valid());
        assert!(cert_tol_fail.verify().unwrap_err().contains("iteration failed to converge"));

        // 3. Residual exceeding estimated eigenvalue is rejected (matches power_iteration_uncertainty_exceeds_eigenvalue_rejected)
        let cert_unc_fail = PowerIterationCertificate::new(1e-7, 2e-7, 1e-6, 1e-8);
        assert!(!cert_unc_fail.is_valid());
        assert!(cert_unc_fail.verify().unwrap_err().contains("uncertainty exceeds spectral distance"));

        // 4. Certified lower bound collapsing below epsilon_min is rejected (matches power_iteration_gap_collapse_rejected)
        let cert_gap_fail = PowerIterationCertificate::new(1e-8, 5e-9, 1e-6, 1e-8);
        assert!(!cert_gap_fail.is_valid());
        assert!(cert_gap_fail.verify().unwrap_err().contains("spectral gap collapse"));
    }

    #[test]
    fn test_gram_spectral_gap_certified_emits_valid_certificate() {
        let primes = [2u64, 3, 5];
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        let v3 = vec![0.0, 0.0, 1.0];
        let gram = compute_gram_matrix(&[v1, v2, v3], &primes).unwrap();

        let res = compute_gram_spectral_gap_certified(&gram, Some(3), None, None);
        assert!(res.is_ok(), "Orthogonal basis must certify: {:?}", res.err());
        let (lambda_min, lambda_max, kappa, cert) = res.unwrap();
        assert!(lambda_min > 0.0);
        assert!(lambda_max >= lambda_min);
        assert!(kappa >= 1.0);
        assert!(cert.is_valid());
        assert!(cert.residual <= 1e-6);
        assert!(cert.certified_lambda_lb >= cert.epsilon_min);

        let state = SpectralState::with_certified_residual(
            900, 10, 50, 3, kappa, cert.residual, cert.is_valid(),
        );
        assert!(state.is_invariant_satisfied());

        // An invalid residual cert marks state invariant as unsatisfied
        let state_invalid_cert = SpectralState::with_certified_residual(
            900, 10, 50, 3, kappa, 1e-4, false,
        );
        assert!(!state_invalid_cert.is_invariant_satisfied());
    }
}

