//! Spectral Small-Gain Runtime Gate
//!
//! Enforces the foundational Small-Gain Theorem invariant:
//!     ρ( |A| · diag(λ) ) < 1.0
//!
//! where A is the inter-ensemble interconnection matrix and λ is the vector of
//! certified per-atom contraction gains.
//!
//! `EnsembleContractivityReceipt.theorem_name` is a required author-supplied
//! Lean identifier. Presence is gated here. Existence and content of that
//! declaration are not checked in this module (ADR-053 remains open).

use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

fn default_ensemble_name() -> String {
    "default_ensemble".to_string()
}

/// Errors on ensemble certification. Spectral numeric failures remain `String`
/// from `check_small_gain`. Receipt issuance uses this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnsembleError {
    /// `theorem_name` missing, empty, or not a Lean-style identifier.
    MissingTheoremAnchor,
}

impl fmt::Display for EnsembleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnsembleError::MissingTheoremAnchor => {
                write!(f, "MissingTheoremAnchor: theorem_name is empty or missing")
            }
        }
    }
}

impl std::error::Error for EnsembleError {}

/// True iff `name` is a non-empty Lean-style identifier.
/// Does not prove a declaration of that name exists on the Lean tree.
pub fn is_theorem_anchor(name: &str) -> bool {
    let s = name.trim();
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'')
        }
        _ => false,
    }
}

/// An interconnected ensemble of components with coupling matrix A and local gains λ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ensemble {
    #[serde(default = "default_ensemble_name")]
    pub name: String,
    pub adjacency: Vec<Vec<f64>>,
    pub lambdas: Vec<f64>,
    /// Author-declared Lean theorem identifier. Default empty so JSON without
    /// the field fails certification rather than inventing an anchor.
    #[serde(default)]
    pub theorem_name: String,
}

impl Ensemble {
    pub fn new(name: impl Into<String>, adjacency: Vec<Vec<f64>>, lambdas: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            adjacency,
            lambdas,
            theorem_name: String::new(),
        }
    }

    pub fn with_theorem_name(mut self, theorem_name: impl Into<String>) -> Self {
        self.theorem_name = theorem_name.into();
        self
    }

    /// Compute the spectral radius of |A| * diag(λ).
    pub fn spectral_radius(&self) -> Option<f64> {
        check_small_gain(self, 0.0).ok()
    }
}

/// Cryptographic receipt certifying that an ensemble satisfies the Small-Gain stability criterion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnsembleContractivityReceipt {
    pub hash: String,
    pub ensemble_name: String,
    pub dimension: usize,
    pub spectral_radius: f64,
    pub is_stable: bool,
    pub timestamp: u64,
    /// Lean identifier supplied by the author. Not a proof of ρ.
    pub theorem_name: String,
}

impl EnsembleContractivityReceipt {
    /// Re-check the theorem-name field on an already-built receipt.
    pub fn validate(&self) -> Result<(), EnsembleError> {
        if is_theorem_anchor(&self.theorem_name) {
            Ok(())
        } else {
            Err(EnsembleError::MissingTheoremAnchor)
        }
    }
}

/// Calculate the spectral radius ρ(M) directly via complex eigenvalue decomposition
pub fn spectral_radius_direct(matrix: &[Vec<f64>]) -> Result<f64, String> {
    let n = matrix.len();
    if n == 0 {
        return Ok(0.0);
    }
    for row in matrix {
        if row.len() != n {
            return Err("Matrix is not square".to_string());
        }
    }

    let flat: Vec<f64> = matrix.iter().flatten().copied().collect();
    let m = DMatrix::from_row_slice(n, n, &flat);

    let complex_eigvals = m.complex_eigenvalues();
    let max_abs = complex_eigvals
        .iter()
        .map(|c| (c.re * c.re + c.im * c.im).sqrt())
        .fold(0.0, f64::max);

    Ok(max_abs)
}

/// Calculate spectral radius via power iteration for non-negative matrices (Perron-Frobenius)
pub fn spectral_radius_power(matrix: &[Vec<f64>], max_iter: usize, tol: f64) -> Result<f64, String> {
    let n = matrix.len();
    if n == 0 {
        return Ok(0.0);
    }

    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut last_rho = 0.0;

    for _ in 0..max_iter {
        let mut next_v = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                next_v[i] += matrix[i][j] * v[j];
            }
        }

        let norm: f64 = next_v.iter().map(|&x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 {
            return Ok(0.0);
        }

        for x in &mut next_v {
            *x /= norm;
        }

        let rho = norm;
        if (rho - last_rho).abs() < tol {
            return Ok(rho);
        }
        last_rho = rho;
        v = next_v;
    }

    Ok(last_rho)
}

/// Construct G = |A| · diag(λ) and verify that ρ(G) < 1.0 - margin
pub fn check_small_gain(ensemble: &Ensemble, margin: f64) -> Result<f64, String> {
    let n = ensemble.adjacency.len();
    if n == 0 {
        return Ok(0.0);
    }
    if n != ensemble.lambdas.len() {
        return Err(format!(
            "Dimension mismatch: adjacency matrix is {}x{}, but lambda vector has length {}",
            n, n, ensemble.lambdas.len()
        ));
    }
    for (i, row) in ensemble.adjacency.iter().enumerate() {
        if row.len() != n {
            return Err(format!("Row {} has length {} instead of {}", i, row.len(), n));
        }
    }

    // Build G = |A| · diag(λ), where G_ij = |A_ij| * λ_j
    let mut g = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let lambda_j = ensemble.lambdas[j];
            if lambda_j < 0.0 {
                return Err(format!("Invalid negative contraction factor λ_{} = {}", j, lambda_j));
            }
            g[i][j] = ensemble.adjacency[i][j].abs() * lambda_j;
        }
    }

    // Compute spectral radius
    let rho = if n <= 64 {
        spectral_radius_direct(&g)?
    } else {
        spectral_radius_power(&g, 1000, 1e-7)?
    };

    let limit = 1.0 - margin;
    if rho >= limit {
        return Err(format!(
            "SIG_GOV_KILL: Spectral small-gain violation in ensemble '{}': ρ(|A|·diag(λ)) = {:.6} >= limit {:.6}",
            ensemble.name, rho, limit
        ));
    }

    Ok(rho)
}

/// Validate an ensemble and generate a cryptographically anchored receipt.
/// Hard-fails if `theorem_name` is empty or not a Lean-style identifier.
pub fn validate_and_certify(ensemble: &Ensemble, margin: f64) -> Result<EnsembleContractivityReceipt, String> {
    if !is_theorem_anchor(&ensemble.theorem_name) {
        return Err(EnsembleError::MissingTheoremAnchor.to_string());
    }

    let rho = check_small_gain(ensemble, margin)?;

    let mut hasher = Sha256::new();
    hasher.update(ensemble.name.as_bytes());
    hasher.update(ensemble.theorem_name.as_bytes());
    hasher.update(&rho.to_le_bytes());
    for row in &ensemble.adjacency {
        for &val in row {
            hasher.update(&val.to_le_bytes());
        }
    }
    for &l in &ensemble.lambdas {
        hasher.update(&l.to_le_bytes());
    }
    let hash = format!("{:x}", hasher.finalize());

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let receipt = EnsembleContractivityReceipt {
        hash,
        ensemble_name: ensemble.name.clone(),
        dimension: ensemble.adjacency.len(),
        spectral_radius: rho,
        is_stable: true,
        timestamp,
        theorem_name: ensemble.theorem_name.trim().to_string(),
    };
    receipt.validate().map_err(|e| e.to_string())?;
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_gain_accept_acyclic() {
        // Acyclic pipeline: 1 -> 2, ρ = 0
        let ensemble = Ensemble::new(
            "pipeline",
            vec![
                vec![0.0, 1.0],
                vec![0.0, 0.0],
            ],
            vec![0.4, 0.4],
        );
        let rho = check_small_gain(&ensemble, 1e-4).expect("Acyclic pipeline must pass");
        assert!(rho < 1e-6);

        let cert = validate_and_certify(
            &ensemble.with_theorem_name("author_declared_lambda"),
            1e-4,
        )
        .unwrap();
        assert!(cert.is_stable);
        assert!(!cert.hash.is_empty());
        assert_eq!(cert.theorem_name, "author_declared_lambda");
    }

    #[test]
    fn test_small_gain_accept_stable_feedback() {
        // Cyclic coupling with small gains: A = [[0, 2], [0.5, 0]], λ = [0.9, 0.9]
        // G = [[0, 1.8], [0.45, 0]], eigenvalues = ±sqrt(1.8 * 0.45) = ±sqrt(0.81) = ±0.9
        // ρ = 0.9 < 1.0
        let ensemble = Ensemble::new(
            "stable_loop",
            vec![
                vec![0.0, 2.0],
                vec![0.5, 0.0],
            ],
            vec![0.9, 0.9],
        );
        let rho = check_small_gain(&ensemble, 0.0).expect("Stable loop with rho=0.9 must pass");
        assert!((rho - 0.9).abs() < 1e-4);
    }

    #[test]
    fn test_small_gain_reject_unstable_feedback() {
        // Cyclic coupling with expansive feedback: A = [[0, 1], [1, 0]], λ = [1.1, 1.1]
        // G = [[0, 1.1], [1.1, 0]], ρ = 1.1 >= 1.0
        let ensemble = Ensemble::new(
            "unstable_loop",
            vec![
                vec![0.0, 1.0],
                vec![1.0, 0.0],
            ],
            vec![1.1, 1.1],
        );
        let res = check_small_gain(&ensemble, 0.0);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("SIG_GOV_KILL"));
    }

    #[test]
    fn test_dimension_mismatch_fails() {
        let ensemble = Ensemble::new(
            "bad_dim",
            vec![
                vec![0.0, 1.0],
                vec![0.0, 0.0],
            ],
            vec![0.5], // 1 lambda for 2x2 matrix
        );
        assert!(check_small_gain(&ensemble, 0.0).is_err());
    }

    #[test]
    fn test_certify_rejects_missing_theorem_name() {
        let ensemble = Ensemble::new(
            "no_anchor",
            vec![vec![0.0, 1.0], vec![0.0, 0.0]],
            vec![0.4, 0.4],
        );
        let err = validate_and_certify(&ensemble, 1e-4).unwrap_err();
        assert!(err.contains("MissingTheoremAnchor"));
    }

    #[test]
    fn test_certify_rejects_whitespace_theorem_name() {
        let ensemble = Ensemble::new(
            "blank_anchor",
            vec![vec![0.0, 1.0], vec![0.0, 0.0]],
            vec![0.4, 0.4],
        )
        .with_theorem_name("   ");
        let err = validate_and_certify(&ensemble, 1e-4).unwrap_err();
        assert!(err.contains("MissingTheoremAnchor"));
    }
}
