//! Phase-Aware Hypergraph Distance Metric and Pre-Flight Generator Interlock
//!
//! Enforces exact topological distance bounds D_Phi(H_t, H_{t+1}) < epsilon_crit
//! using induced l1 operator norm over exact Rational64 arithmetic.

use num_rational::Ratio;
use thiserror::Error;

/// Critical phase divergence threshold: epsilon_crit = Delta_max = 3/100 = 0.03
pub const EPSILON_CRIT: Ratio<i64> = Ratio::new_raw(3, 100);

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum GeneratorViolation {
    #[error("SIG_GOV_KILL: Phase Dissonance Breach: D_Phi({0}/{1}) >= epsilon_crit({2}/{3})")]
    PhaseDissonance(i64, i64, i64, i64),
    #[error("Topological Incoherence: dimension mismatch between state hypergraphs")]
    DimensionMismatch,
}

/// Phase-projected hypergraph representation in exact Rational64
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseHypergraph {
    pub dimension: usize,
    /// Adjacency tensor entries A_Phi(i, j) represented as exact Rational64
    pub tensor: Vec<Vec<Ratio<i64>>>,
}

impl PhaseHypergraph {
    pub fn new(dim: usize) -> Self {
        Self {
            dimension: dim,
            tensor: vec![vec![Ratio::new(0, 1); dim]; dim],
        }
    }

    /// Compute D_Phi(H_t, H_next) as the induced l1 operator norm in exact rational arithmetic
    pub fn distance(&self, next: &PhaseHypergraph) -> Result<Ratio<i64>, GeneratorViolation> {
        if self.dimension != next.dimension {
            return Err(GeneratorViolation::DimensionMismatch);
        }

        let mut max_col_sum = Ratio::new(0, 1);

        for j in 0..self.dimension {
            let mut col_sum = Ratio::new(0, 1);
            for i in 0..self.dimension {
                let diff = self.tensor[i][j] - next.tensor[i][j];
                let abs_diff = if diff >= Ratio::new(0, 1) { diff } else { -diff };
                col_sum = col_sum + abs_diff;
            }
            if col_sum > max_col_sum {
                max_col_sum = col_sum;
            }
        }

        Ok(max_col_sum)
    }

    /// Pre-flight generator interlock: Aborts synthesis before MLIR lowering or ACE budgeting
    pub fn verify_transition(&self, next: &PhaseHypergraph) -> Result<Ratio<i64>, GeneratorViolation> {
        let d_phi = self.distance(next)?;

        if d_phi >= EPSILON_CRIT {
            return Err(GeneratorViolation::PhaseDissonance(
                *d_phi.numer(),
                *d_phi.denom(),
                *EPSILON_CRIT.numer(),
                *EPSILON_CRIT.denom(),
            ));
        }

        Ok(d_phi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_self_distance() {
        let mut h = PhaseHypergraph::new(3);
        h.tensor[0][1] = Ratio::new(1, 2);
        h.tensor[1][2] = Ratio::new(1, 3);
        
        let dist = h.distance(&h).expect("Self-distance calculation should succeed");
        assert_eq!(dist, Ratio::new(0, 1), "D_Phi(H, H) must be identically zero");
        assert!(h.verify_transition(&h).is_ok());
    }

    #[test]
    fn test_triangle_inequality() {
        let mut h1 = PhaseHypergraph::new(2);
        let mut h2 = PhaseHypergraph::new(2);
        let mut h3 = PhaseHypergraph::new(2);

        h1.tensor[0][1] = Ratio::new(1, 100);
        h2.tensor[0][1] = Ratio::new(2, 100);
        h3.tensor[0][1] = Ratio::new(5, 100);

        let d12 = h1.distance(&h2).unwrap();
        let d23 = h2.distance(&h3).unwrap();
        let d13 = h1.distance(&h3).unwrap();

        assert!(d13 <= d12 + d23, "Triangle inequality must hold: D(1,3) <= D(1,2) + D(2,3)");
    }

    #[test]
    fn test_transition_pass_contractive() {
        let mut h1 = PhaseHypergraph::new(2);
        let mut h2 = PhaseHypergraph::new(2);

        // Perturbation of 1/100 = 0.01 < 0.03
        h1.tensor[0][0] = Ratio::new(5, 100);
        h2.tensor[0][0] = Ratio::new(6, 100);

        let res = h1.verify_transition(&h2);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Ratio::new(1, 100));
    }

    #[test]
    fn test_transition_fail_phase_dissonance() {
        let mut h1 = PhaseHypergraph::new(2);
        let mut h2 = PhaseHypergraph::new(2);

        // Perturbation of 4/100 = 0.04 >= 0.03 (Breach!)
        h1.tensor[0][0] = Ratio::new(10, 100);
        h2.tensor[0][0] = Ratio::new(14, 100);

        let res = h1.verify_transition(&h2);
        assert!(res.is_err());
        match res.unwrap_err() {
            GeneratorViolation::PhaseDissonance(n, d, cn, cd) => {
                assert_eq!((n, d), (1, 25)); // 4/100 simplified = 1/25
                assert_eq!((cn, cd), (3, 100));
            }
            other => panic!("Unexpected error variant: {:?}", other),
        }
    }

    #[test]
    fn test_dimension_mismatch() {
        let h1 = PhaseHypergraph::new(2);
        let h2 = PhaseHypergraph::new(3);

        let res = h1.verify_transition(&h2);
        assert_eq!(res.unwrap_err(), GeneratorViolation::DimensionMismatch);
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    #[kani::unwind(4)]
    pub fn prove_phase_distance_bounded() {
        let dim: usize = 2;
        let mut h1 = PhaseHypergraph::new(dim);
        let mut h2 = PhaseHypergraph::new(dim);

        for i in 0..dim {
            for j in 0..dim {
                let n1: i64 = kani::any();
                let n2: i64 = kani::any();
                kani::assume(n1 >= -50 && n1 <= 50);
                kani::assume(n2 >= -50 && n2 <= 50);
                h1.tensor[i][j] = Ratio::new(n1, 100);
                h2.tensor[i][j] = Ratio::new(n2, 100);
            }
        }

        let d_phi = h1.distance(&h2).unwrap();
        // Distance is non-negative
        kani::assert(d_phi >= Ratio::new(0, 1), "Distance must be non-negative");

        let check = h1.verify_transition(&h2);
        if d_phi >= Ratio::new(3, 100) {
            kani::assert(matches!(check, Err(GeneratorViolation::PhaseDissonance(..))), "Must halt on breach");
        } else {
            kani::assert(check.is_ok(), "Must accept contractive transition");
        }
    }
}
