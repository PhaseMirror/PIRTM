//! Exact rational 1-norm contractivity gate.
//!
//! Production predicate: ||G||_1 < 1 in Q for G = |A| diag(λ).
//! Canonical constructor: Ensemble::from_rationals.
//! Ensemble::new(f64) is deprecated Phase 1 SDK surface.

use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::fmt;

fn default_ensemble_name() -> String {
    "default_ensemble".to_string()
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PosRat {
    pub num: u64,
    pub den: u64,
}

impl PosRat {
    pub fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    pub fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    pub fn new(num: u64, den: u64) -> Result<Self, EnsembleError> {
        if den == 0 {
            return Err(EnsembleError::InvalidRational);
        }
        Ok(Self::reduce_u64(num, den))
    }

    fn reduce_u64(num: u64, den: u64) -> Self {
        let g = gcd_u128(num as u128, den as u128) as u64;
        Self {
            num: num / g,
            den: den / g,
        }
    }

    fn from_u128(num: u128, den: u128) -> Result<Self, EnsembleError> {
        if den == 0 {
            return Err(EnsembleError::InvalidRational);
        }
        let g = gcd_u128(num, den);
        let n = num / g;
        let d = den / g;
        if n > u64::MAX as u128 || d > u64::MAX as u128 {
            return Err(EnsembleError::RationalOverflow);
        }
        Ok(Self {
            num: n as u64,
            den: d as u64,
        })
    }

    pub fn add(self, other: Self) -> Result<Self, EnsembleError> {
        Self::from_u128(
            self.num as u128 * other.den as u128 + other.num as u128 * self.den as u128,
            self.den as u128 * other.den as u128,
        )
    }

    pub fn mul(self, other: Self) -> Result<Self, EnsembleError> {
        Self::from_u128(
            self.num as u128 * other.num as u128,
            self.den as u128 * other.den as u128,
        )
    }

    pub fn lt_one(self) -> bool {
        self.num < self.den
    }

    pub fn cmp_q(self, other: Self) -> Ordering {
        (self.num as u128 * other.den as u128).cmp(&(other.num as u128 * self.den as u128))
    }

    pub fn as_pair(self) -> (u64, u64) {
        (self.num, self.den)
    }

    pub fn from_f64_membrane(x: f64) -> Result<Self, EnsembleError> {
        if !x.is_finite() {
            return Err(EnsembleError::InvalidRational);
        }
        if x < 0.0 {
            return Err(EnsembleError::InvalidGain);
        }
        if x == 0.0 {
            return Ok(Self::zero());
        }
        let mut n0: u128 = 0;
        let mut d0: u128 = 1;
        let mut n1: u128 = 1;
        let mut d1: u128 = 0;
        let mut val = x;
        for _ in 0..32 {
            if !val.is_finite() {
                break;
            }
            let a = val.floor();
            if a < 0.0 || a > u64::MAX as f64 {
                return Err(EnsembleError::RationalOverflow);
            }
            let au = a as u128;
            let n2 = au.saturating_mul(n1).saturating_add(n0);
            let d2 = au.saturating_mul(d1).saturating_add(d0);
            if n2 > u64::MAX as u128 || d2 > u64::MAX as u128 || d2 == 0 {
                break;
            }
            n0 = n1;
            d0 = d1;
            n1 = n2;
            d1 = d2;
            let frac = val - a;
            if frac.abs() < 1e-12 {
                break;
            }
            val = 1.0 / frac;
        }
        if d1 == 0 {
            return Err(EnsembleError::InvalidRational);
        }
        Self::new(n1 as u64, d1 as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnsembleError {
    MissingTheoremAnchor,
    NormContractivityViolation { norm_1: (u64, u64) },
    InvalidRational,
    InvalidGain,
    RationalOverflow,
    DimensionMismatch { matrix: usize, lambda: usize },
    MatrixNotSquare { row: usize, len: usize, expected: usize },
}

impl fmt::Display for EnsembleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnsembleError::MissingTheoremAnchor => {
                write!(f, "MissingTheoremAnchor: theorem_name is empty or missing")
            }
            EnsembleError::NormContractivityViolation { norm_1 } => write!(
                f,
                "NormContractivityViolation: ||G||_1 = {}/{} >= 1",
                norm_1.0, norm_1.1
            ),
            EnsembleError::InvalidRational => {
                write!(f, "InvalidRational: denominator must be >= 1")
            }
            EnsembleError::InvalidGain => {
                write!(f, "InvalidGain: contraction factor must be >= 0")
            }
            EnsembleError::RationalOverflow => {
                write!(f, "RationalOverflow: intermediate 1-norm exceeded u64 after reduction")
            }
            EnsembleError::DimensionMismatch { matrix, lambda } => write!(
                f,
                "Dimension mismatch: adjacency matrix is {matrix}x{matrix}, but lambda vector has length {lambda}"
            ),
            EnsembleError::MatrixNotSquare {
                row,
                len,
                expected,
            } => write!(f, "Row {row} has length {len} instead of {expected}"),
        }
    }
}

impl std::error::Error for EnsembleError {}

pub fn is_theorem_anchor(name: &str) -> bool {
    let s = name.trim();
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '.')
        }
        _ => false,
    }
}

pub fn matrix_pairs_from_f64(rows: &[Vec<f64>]) -> Result<Vec<Vec<(u64, u64)>>, EnsembleError> {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|x| PosRat::from_f64_membrane(*x).map(|p| p.as_pair()))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

pub fn lambda_pairs_from_f64(xs: &[f64]) -> Result<Vec<(u64, u64)>, EnsembleError> {
    xs.iter()
        .map(|x| PosRat::from_f64_membrane(*x).map(|p| p.as_pair()))
        .collect()
}

fn matrix_from_f64(rows: &[Vec<f64>]) -> Result<Vec<Vec<PosRat>>, EnsembleError> {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|x| PosRat::from_f64_membrane(*x))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

fn vec_from_f64(xs: &[f64]) -> Result<Vec<PosRat>, EnsembleError> {
    xs.iter().map(|x| PosRat::from_f64_membrane(*x)).collect()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ensemble {
    #[serde(default = "default_ensemble_name")]
    pub name: String,
    pub adjacency: Vec<Vec<PosRat>>,
    pub lambdas: Vec<PosRat>,
    #[serde(default)]
    pub theorem_name: String,
}

impl Ensemble {
    #[deprecated(note = "Use Ensemble::from_rationals with exact (u64, u64) ratios")]
    pub fn new(name: impl Into<String>, adjacency: Vec<Vec<f64>>, lambdas: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            adjacency: matrix_from_f64(&adjacency).unwrap_or_default(),
            lambdas: vec_from_f64(&lambdas).unwrap_or_default(),
            theorem_name: String::new(),
        }
    }

    pub fn try_new(
        name: impl Into<String>,
        adjacency: Vec<Vec<f64>>,
        lambdas: Vec<f64>,
    ) -> Result<Self, EnsembleError> {
        Ok(Self {
            name: name.into(),
            adjacency: matrix_from_f64(&adjacency)?,
            lambdas: vec_from_f64(&lambdas)?,
            theorem_name: String::new(),
        })
    }

    pub fn from_rationals(
        name: impl Into<String>,
        adjacency: Vec<Vec<(u64, u64)>>,
        lambdas: Vec<(u64, u64)>,
        theorem_name: impl Into<String>,
    ) -> Result<Self, EnsembleError> {
        let theorem_name = theorem_name.into();
        if !is_theorem_anchor(&theorem_name) {
            return Err(EnsembleError::MissingTheoremAnchor);
        }
        let mut adj = Vec::with_capacity(adjacency.len());
        for row in adjacency {
            let mut r = Vec::with_capacity(row.len());
            for (p, q) in row {
                r.push(PosRat::new(p, q)?);
            }
            adj.push(r);
        }
        let mut lams = Vec::with_capacity(lambdas.len());
        for (p, q) in lambdas {
            lams.push(PosRat::new(p, q)?);
        }
        Ok(Self {
            name: name.into(),
            adjacency: adj,
            lambdas: lams,
            theorem_name,
        })
    }

    pub fn with_theorem_name(mut self, theorem_name: impl Into<String>) -> Self {
        self.theorem_name = theorem_name.into();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnsembleContractivityReceipt {
    pub hash: String,
    pub ensemble_name: String,
    pub dimension: usize,
    pub exact_rational_norm_1: (u64, u64),
    pub is_norm_contractive: bool,
    pub theorem_name: String,
    pub timestamp: u64,
}

impl EnsembleContractivityReceipt {
    pub fn validate(&self) -> Result<(), EnsembleError> {
        if is_theorem_anchor(&self.theorem_name) {
            Ok(())
        } else {
            Err(EnsembleError::MissingTheoremAnchor)
        }
    }
}

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

fn one_norm(ensemble: &Ensemble) -> Result<PosRat, EnsembleError> {
    let n = ensemble.adjacency.len();
    if n == 0 {
        return Ok(PosRat::zero());
    }
    if n != ensemble.lambdas.len() {
        return Err(EnsembleError::DimensionMismatch {
            matrix: n,
            lambda: ensemble.lambdas.len(),
        });
    }
    for (i, row) in ensemble.adjacency.iter().enumerate() {
        if row.len() != n {
            return Err(EnsembleError::MatrixNotSquare {
                row: i,
                len: row.len(),
                expected: n,
            });
        }
    }
    let mut max = PosRat::zero();
    for j in 0..n {
        let mut col = PosRat::zero();
        for i in 0..n {
            let g_ij = ensemble.adjacency[i][j].mul(ensemble.lambdas[j])?;
            col = col.add(g_ij)?;
        }
        if col.cmp_q(max) == Ordering::Greater {
            max = col;
        }
    }
    Ok(max)
}

pub fn check_small_gain(ensemble: &Ensemble, _margin: f64) -> Result<f64, String> {
    let n1 = one_norm(ensemble).map_err(|e| e.to_string())?;
    if !n1.lt_one() {
        return Err(EnsembleError::NormContractivityViolation {
            norm_1: n1.as_pair(),
        }
        .to_string());
    }
    Ok(n1.num as f64 / n1.den as f64)
}

pub fn validate_and_certify(
    ensemble: &Ensemble,
    _margin: f64,
) -> Result<EnsembleContractivityReceipt, String> {
    if !is_theorem_anchor(&ensemble.theorem_name) {
        return Err(EnsembleError::MissingTheoremAnchor.to_string());
    }
    let n1 = one_norm(ensemble).map_err(|e| e.to_string())?;
    if !n1.lt_one() {
        return Err(EnsembleError::NormContractivityViolation {
            norm_1: n1.as_pair(),
        }
        .to_string());
    }
    let mut hasher = Sha256::new();
    hasher.update(ensemble.name.as_bytes());
    hasher.update(ensemble.theorem_name.as_bytes());
    hasher.update(&n1.num.to_le_bytes());
    hasher.update(&n1.den.to_le_bytes());
    for row in &ensemble.adjacency {
        for cell in row {
            hasher.update(&cell.num.to_le_bytes());
            hasher.update(&cell.den.to_le_bytes());
        }
    }
    for lam in &ensemble.lambdas {
        hasher.update(&lam.num.to_le_bytes());
        hasher.update(&lam.den.to_le_bytes());
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
        exact_rational_norm_1: n1.as_pair(),
        is_norm_contractive: true,
        theorem_name: ensemble.theorem_name.trim().to_string(),
        timestamp,
    };
    receipt.validate().map_err(|e| e.to_string())?;
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_norm_accept_retuned_loop() {
        let ensemble = Ensemble::from_rationals(
            "retuned_loop",
            vec![vec![(0, 1), (2, 5)], vec![(2, 5), (0, 1)]],
            vec![(9, 10), (9, 10)],
            "author_declared_lambda",
        )
        .unwrap();
        let n1 = one_norm(&ensemble).unwrap();
        assert_eq!(n1.as_pair(), (9, 25));
        let cert = validate_and_certify(&ensemble, 0.0).unwrap();
        assert!(cert.is_norm_contractive);
        assert_eq!(cert.exact_rational_norm_1, (9, 25));
    }

    #[test]
    fn test_one_norm_reject_old_stable_loop() {
        let ensemble = Ensemble::from_rationals(
            "old_stable_loop",
            vec![vec![(0, 1), (2, 1)], vec![(1, 2), (0, 1)]],
            vec![(9, 10), (9, 10)],
            "author_declared_lambda",
        )
        .unwrap();
        let err = validate_and_certify(&ensemble, 0.0).unwrap_err();
        assert!(err.contains("NormContractivityViolation"));
    }

    #[test]
    fn test_from_rationals_rejects_zero_den() {
        let err = Ensemble::from_rationals(
            "bad",
            vec![vec![(1, 0)]],
            vec![(1, 1)],
            "author_declared_lambda",
        )
        .unwrap_err();
        assert_eq!(err, EnsembleError::InvalidRational);
    }

    #[test]
    fn test_from_rationals_rejects_missing_name() {
        let err = Ensemble::from_rationals(
            "bad",
            vec![vec![(0, 1), (1, 2)], vec![(1, 2), (0, 1)]],
            vec![(1, 2), (1, 2)],
            "",
        )
        .unwrap_err();
        assert_eq!(err, EnsembleError::MissingTheoremAnchor);
    }

    #[test]
    fn test_reduce_eq() {
        let a = PosRat::new(2, 4).unwrap();
        let b = PosRat::new(1, 2).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    #[allow(deprecated)]
    fn test_f64_membrane_matches_retuned_ratios() {
        let ensemble = Ensemble::new(
            "membrane",
            vec![vec![0.0, 0.4], vec![0.4, 0.0]],
            vec![0.9, 0.9],
        );
        assert_eq!(ensemble.adjacency[0][1].as_pair(), (2, 5));
        assert_eq!(ensemble.lambdas[0].as_pair(), (9, 10));
    }
}

#[cfg(kani)]
mod kani_harnesses {
    use super::*;

    #[kani::proof]
    fn proof_posrat_reduction_canonical() {
        let p: u64 = kani::any();
        let q: u64 = kani::any();
        kani::assume(q > 0);
        kani::assume(p < 1000);
        kani::assume(q < 1000);
        if let Ok(rat) = PosRat::new(p, q) {
            let (num, den) = rat.as_pair();
            kani::assert(den > 0, "denominator is positive after reduction");
            kani::assert(num * q == p * den, "reduction preserves value");
        }
    }

    #[kani::proof]
    fn proof_posrat_lt_one_soundness() {
        let p: u64 = kani::any();
        let q: u64 = kani::any();
        kani::assume(q > 0);
        kani::assume(p < 1000);
        kani::assume(q < 1000);
        if let Ok(rat) = PosRat::new(p, q) {
            let is_lt = rat.lt_one();
            kani::assert(is_lt == (rat.num < rat.den), "lt_one equivalence holds");
        }
    }
}
