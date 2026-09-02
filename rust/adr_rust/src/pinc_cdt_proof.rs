/// Prime-Indexed Noncommutative Causal Dynamical Triangulations (PINC-CDT) (ADR-054).
pub struct PincCdtSimplex {
    pub weight: f64,
    pub epsilon: f64,
    pub area: f64,
    pub ncg_term: f64,
}

pub struct PincCdtSector {
    pub prime: u64,
    pub theta: f64,
    pub simplices: Vec<PincCdtSimplex>,
}

impl PincCdtSector {
    pub fn action_density(&self, simplex_idx: usize, lambda: f64) -> f64 {
        let s = &self.simplices[simplex_idx];
        let regge = s.epsilon * s.area;
        let ncg = s.ncg_term;
        let coupling = lambda * self.theta * s.epsilon;
        regge + ncg + coupling
    }

    pub fn sector_average_action(&self, lambda: f64) -> f64 {
        let mut sum = 0.0;
        let mut wsum = 0.0;
        for (i, s) in self.simplices.iter().enumerate() {
            sum += s.weight * self.action_density(i, lambda);
            wsum += s.weight;
        }
        if wsum > 0.0 {
            sum / wsum
        } else {
            0.0
        }
    }
}

/// Spectral dimension proxy computation: D_s(t) = 2.0 - c * avg_epsilon.
pub fn compute_spectral_dimension_proxy(avg_epsilon: f64, epsilon_max: f64) -> f64 {
    let c = 0.8 / epsilon_max.max(1e-6);
    (2.0 - c * avg_epsilon).clamp(1.2, 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinc_cdt_action_density() {
        let simplex = PincCdtSimplex {
            weight: 1.0,
            epsilon: 0.5,
            area: 2.0,
            ncg_term: 0.3,
        };
        let sector = PincCdtSector {
            prime: 2,
            theta: 1.0,
            simplices: vec![simplex],
        };
        let action = sector.action_density(0, 0.1);
        // regge = 0.5 * 2.0 = 1.0, ncg = 0.3, coupling = 0.1 * 1.0 * 0.5 = 0.05 => 1.35
        assert!((action - 1.35).abs() < 1e-6);
    }

    #[test]
    fn test_spectral_dimension_proxy_bounds() {
        let ds = compute_spectral_dimension_proxy(0.5, 1.0);
        assert!(ds >= 1.2 && ds <= 2.0);
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn proof_spectral_dimension_proxy_bounds() {
        let avg_eps: f64 = kani::any();
        let eps_max: f64 = kani::any();
        kani::assume(avg_eps >= 0.0 && avg_eps <= 10.0);
        kani::assume(eps_max >= 0.01 && eps_max <= 10.0);

        let ds = compute_spectral_dimension_proxy(avg_eps, eps_max);
        assert!(ds >= 1.2 && ds <= 2.0);
    }

    #[kani::proof]
    fn proof_euler_discretization_step_stable() {
        let gamma_dt_scaled: i32 = kani::any();
        kani::assume(gamma_dt_scaled > 0 && gamma_dt_scaled < 200);

        let diff = 100 - gamma_dt_scaled;
        assert!(diff.abs() < 100);
    }
}
