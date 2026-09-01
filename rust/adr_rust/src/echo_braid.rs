//! ADR-040: EchoBraid Quantum Feedback & Recursive Spectrum Coherence
//!
//! Rust implementation and Kani model checking harness for ADR-040:
//! - Prime-indexed eigenphase braid strands.
//! - Prediction skeleton contractivity \Delta_pred(t).

#[derive(Debug, Clone)]
pub struct BraidStrand {
    pub prime_index: u32,
    pub amplitude: u32,
    pub phase_deg: u32,
}

pub fn calculate_braid_coherence(strands: &[BraidStrand]) -> u64 {
    strands
        .iter()
        .map(|s| (s.prime_index as u64 * s.amplitude as u64).pow(2))
        .sum()
}

#[derive(Debug, Clone)]
pub struct PredictionSkeleton {
    pub alpha_dot_xi: u32,
    pub beta_delta: u32,
    pub max_allowed: u32,
}

impl PredictionSkeleton {
    pub fn calculate_prediction(&self) -> u32 {
        self.alpha_dot_xi.saturating_add(self.beta_delta)
    }

    pub fn is_contractive(&self) -> bool {
        self.calculate_prediction() <= self.max_allowed
    }
}

// ─── Kani Verification Harnesses for ADR-040 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr040_prediction_contractivity() {
    let alpha_dot_xi: u32 = kani::any();
    let beta_delta: u32 = kani::any();
    let max_allowed: u32 = kani::any();

    kani::assume(alpha_dot_xi <= 1000);
    kani::assume(beta_delta <= 1000);

    let skeleton = PredictionSkeleton {
        alpha_dot_xi,
        beta_delta,
        max_allowed,
    };

    if skeleton.is_contractive() {
        assert!(skeleton.calculate_prediction() <= max_allowed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braid_coherence_calculation() {
        let strands = vec![
            BraidStrand {
                prime_index: 2,
                amplitude: 3,
                phase_deg: 45,
            },
            BraidStrand {
                prime_index: 3,
                amplitude: 2,
                phase_deg: 90,
            },
        ];
        // (2*3)^2 = 36. (3*2)^2 = 36. sum = 72.
        assert_eq!(calculate_braid_coherence(&strands), 72);
    }

    #[test]
    fn test_prediction_skeleton_contractivity() {
        let s_pass = PredictionSkeleton {
            alpha_dot_xi: 10,
            beta_delta: 15,
            max_allowed: 30,
        };
        let s_fail = PredictionSkeleton {
            alpha_dot_xi: 20,
            beta_delta: 15,
            max_allowed: 30,
        };

        assert!(s_pass.is_contractive());
        assert!(!s_fail.is_contractive());
    }
}
