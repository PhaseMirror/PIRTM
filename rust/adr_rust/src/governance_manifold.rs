//! ADR-038: Phase Mirror Governance Manifold & Fail-Closed Control
//!
//! Rust implementation and Kani model checking harness for ADR-038:
//! - Drift safety envelope (\delta_soft, \delta_hard).
//! - Continuous gain scaling \alpha(\delta) = min(1, \delta / \delta_hard).
//! - Fail-closed GovernorHalt arbitration logic.
//! - Drift-adaptive cache invalidation.

#[derive(Debug, Clone)]
pub struct DriftState {
    pub drift_scaled: u32,       // \delta * 100
    pub drift_dot_scaled: i32,   // \dot{\delta} * 100
    pub delta_soft_scaled: u32,  // \delta_soft * 100
    pub delta_hard_scaled: u32,  // \delta_hard * 100
}

pub fn calculate_gain_scaled(d: &DriftState) -> u32 {
    if d.delta_hard_scaled == 0 {
        100
    } else {
        std::cmp::min(100, (d.drift_scaled * 100) / d.delta_hard_scaled)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlArbitration {
    ContinuousDamping,
    GovernorHalt,
}

pub fn arbitrate_control(d: &DriftState) -> ControlArbitration {
    let alpha = calculate_gain_scaled(d);
    if alpha >= 100 && d.drift_dot_scaled > 0 {
        ControlArbitration::GovernorHalt
    } else {
        ControlArbitration::ContinuousDamping
    }
}

#[derive(Debug, Clone)]
pub struct ControlVectorCache {
    pub commit_time: u64,
    pub current_time: u64,
    pub ttl_max: u64,
}

pub fn is_cache_valid(d: &DriftState, cache: &ControlVectorCache) -> bool {
    d.drift_scaled <= d.delta_soft_scaled && (cache.current_time.saturating_sub(cache.commit_time) <= cache.ttl_max)
}

// ─── Kani Verification Harnesses for ADR-038 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr038_governor_halt_on_saturation() {
    let drift_scaled: u32 = kani::any();
    let drift_dot_scaled: i32 = kani::any();
    let delta_hard_scaled: u32 = kani::any();

    kani::assume(delta_hard_scaled >= 10 && delta_hard_scaled <= 1000);
    kani::assume(drift_scaled >= delta_hard_scaled);
    kani::assume(drift_dot_scaled > 0);

    let d = DriftState {
        drift_scaled,
        drift_dot_scaled,
        delta_soft_scaled: delta_hard_scaled / 2,
        delta_hard_scaled,
    };

    assert_eq!(arbitrate_control(&d), ControlArbitration::GovernorHalt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_manifold_arbitration() {
        let d = DriftState {
            drift_scaled: 350,
            drift_dot_scaled: 50,
            delta_soft_scaled: 200,
            delta_hard_scaled: 300,
        };
        assert_eq!(arbitrate_control(&d), ControlArbitration::GovernorHalt);
    }

    #[test]
    fn test_drift_adaptive_cache_invalidation() {
        let d_safe = DriftState {
            drift_scaled: 150,
            drift_dot_scaled: 10,
            delta_soft_scaled: 200,
            delta_hard_scaled: 300,
        };
        let d_unsafe = DriftState {
            drift_scaled: 250,
            drift_dot_scaled: 10,
            delta_soft_scaled: 200,
            delta_hard_scaled: 300,
        };
        let cache = ControlVectorCache {
            commit_time: 100,
            current_time: 120,
            ttl_max: 50,
        };

        assert!(is_cache_valid(&d_safe, &cache));
        assert!(!is_cache_valid(&d_unsafe, &cache));
    }
}
