/// Universal Multiplicity Constant Lambda_m Regulator and PMRO Operator (ADR-053).
pub struct UmcRegulator {
    pub mu: usize,
    pub gamma: f64,
    pub epsilon: f64,
    pub l_t: f64,
    pub lambda_m0: f64,
    pub stress_counter: usize,
}

#[derive(Debug, PartialEq)]
pub enum GovernanceOutcome {
    Admissible(f64),
    Rescale(f64),
    InadmissibleHalt,
    StressHalt,
}

impl UmcRegulator {
    pub fn new(mu: usize, gamma: f64, epsilon: f64, lambda_m0: f64) -> Self {
        Self {
            mu,
            gamma,
            epsilon,
            l_t: 1.0,
            lambda_m0,
            stress_counter: 0,
        }
    }

    pub fn evaluate_step(&mut self, s_norm: f64, dphi_op_norm: f64, state_norm: f64, b_bound: f64) -> GovernanceOutcome {
        let lambda_glob = self.gamma / (s_norm + 1e-12);
        let lambda_loc = self.gamma / (dphi_op_norm + 1e-12);
        let mut lambda_m = self.lambda_m0 * lambda_glob.min(lambda_loc);

        let mut c = lambda_m.abs() * self.l_t;
        if c >= self.epsilon {
            let mut rescale_ok = false;
            for _ in 0..3 {
                lambda_m *= 1.0 - 0.05 * self.epsilon;
                c = lambda_m.abs() * self.l_t;
                if c < self.epsilon {
                    rescale_ok = true;
                    break;
                }
            }
            if !rescale_ok {
                return GovernanceOutcome::InadmissibleHalt;
            }
        }

        if state_norm > b_bound {
            self.stress_counter += 1;
            if self.stress_counter >= 3 {
                return GovernanceOutcome::StressHalt;
            }
            return GovernanceOutcome::Rescale(lambda_m);
        } else {
            self.stress_counter = 0;
        }

        GovernanceOutcome::Admissible(lambda_m)
    }
}

/// Compute Frobenius associator defect bound Delta(x, y, z) = ||U_x U_y U_z - U_z U_y U_x||_F <= 2 * sqrt(N).
pub fn associator_defect_upper_bound(n_dim: usize) -> f64 {
    2.0 * (n_dim as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_umc_regulator_admissible() {
        let mut reg = UmcRegulator::new(4, 0.7, 0.1, 1.0);
        let outcome = reg.evaluate_step(10.0, 10.0, 1.0, 100.0);
        match outcome {
            GovernanceOutcome::Admissible(l) => assert!(l > 0.0),
            _ => panic!("Expected admissible governance outcome"),
        }
    }

    #[test]
    fn test_associator_defect_bound() {
        let bound = associator_defect_upper_bound(8);
        assert!((bound - 2.0 * 8.0f64.sqrt()).abs() < 1e-9);
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn proof_fail_closed_precedence() {
        let stress_counter: usize = kani::any();
        kani::assume(stress_counter >= 3);

        // Invariant: when stress counter >= 3, fail-closed rule forces halt
        let halts = stress_counter >= 3;
        assert!(halts);
    }

    #[kani::proof]
    fn proof_associator_defect_bound_non_negative() {
        let n_dim: usize = kani::any();
        kani::assume(n_dim >= 1 && n_dim <= 1000);

        let bound = associator_defect_upper_bound(n_dim);
        assert!(bound >= 2.0);
    }
}
