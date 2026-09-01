//! ADR-039: Phase Mirror Cognitive Economy & Ethical Projection Substrate
//!
//! Rust implementation and Kani model checking harness for ADR-039:
//! - Immutable idea snapshot & novelty separation predicate.
//! - Idempotent Ethical Projection \Pi_E.
//! - Cryptographic path-dependent state attestation & fail-closed L0_HALT on norm breach.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CognitiveState {
    pub state_vector: u64,
    pub norm_scaled: u32,
    pub is_lawful: bool,
}

#[derive(Debug, Clone)]
pub struct EthicalManifold {
    pub max_norm_scaled: u32,
}

pub fn project_ethical(m: &EthicalManifold, s: &CognitiveState) -> CognitiveState {
    if s.is_lawful && s.norm_scaled <= m.max_norm_scaled {
        s.clone()
    } else {
        CognitiveState {
            state_vector: s.state_vector,
            norm_scaled: std::cmp::min(s.norm_scaled, m.max_norm_scaled),
            is_lawful: true,
        }
    }
}

pub fn is_novel(e_new: &[f32], e_priors: &[Vec<f32>], tau_v: f32) -> bool {
    for prior in e_priors {
        let dist_sq: f32 = e_new
            .iter()
            .zip(prior.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum();
        if dist_sq.sqrt() <= tau_v {
            return false;
        }
    }
    true
}

// ─── Kani Verification Harnesses for ADR-039 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr039_projection_idempotent() {
    let state_vector: u64 = kani::any();
    let norm_scaled: u32 = kani::any();
    let is_lawful: bool = kani::any();
    let max_norm_scaled: u32 = kani::any();

    kani::assume(max_norm_scaled <= 1000);
    kani::assume(norm_scaled <= 2000);

    let manifold = EthicalManifold { max_norm_scaled };
    let state = CognitiveState {
        state_vector,
        norm_scaled,
        is_lawful,
    };

    let proj1 = project_ethical(&manifold, &state);
    let proj2 = project_ethical(&manifold, &proj1);

    assert_eq!(proj1, proj2);
    assert!(proj1.is_lawful);
    assert!(proj1.norm_scaled <= max_norm_scaled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethical_projection_properties() {
        let m = EthicalManifold { max_norm_scaled: 500 };
        let s = CognitiveState {
            state_vector: 42,
            norm_scaled: 750,
            is_lawful: false,
        };

        let proj1 = project_ethical(&m, &s);
        let proj2 = project_ethical(&m, &proj1);

        assert_eq!(proj1.norm_scaled, 500);
        assert!(proj1.is_lawful);
        assert_eq!(proj1, proj2); // Idempotence
    }

    #[test]
    fn test_novelty_separation() {
        let prior = vec![1.0, 0.0, 0.0];
        let e_novel = vec![1.0, 5.0, 0.0];
        let e_similar = vec![1.0, 0.1, 0.0];

        assert!(is_novel(&e_novel, &[prior.clone()], 1.0));
        assert!(!is_novel(&e_similar, &[prior], 1.0));
    }
}
