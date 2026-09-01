//! ADR-047: Sedona Spine & RSL v5 Sentinel Integration
//!
//! Rust implementation and Kani model checking harness for ADR-047:
//! - Dual-layer contractivity validation (static small-gain + dynamic drift metrics).
//! - Deterministic receipt generation and fail-closed halt bounds.

#[derive(Debug, Clone)]
pub struct SentinelConfig {
    pub rho_halt: u32,  // \rho * 100 (e.g. 100)
    pub rho_warn: u32,  // \rho * 100 (e.g. 85)
    pub delta_max: u32, // \delta * 10000 (e.g. 10)
}

#[derive(Debug, Clone)]
pub struct ManifoldState {
    pub rho: u32,
    pub delta: u32,
    pub lambda_l_product: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SentinelOutcome {
    Pass,
    Warn,
    Kill(&'static str),
}

pub fn validate_sentinel(state: &ManifoldState, cfg: &SentinelConfig, static_ok: bool) -> SentinelOutcome {
    if !static_ok {
        SentinelOutcome::Kill("Registration-time small gain failure")
    } else if state.rho >= cfg.rho_halt {
        SentinelOutcome::Kill("Drift exceeded halt threshold")
    } else if state.delta >= cfg.delta_max {
        SentinelOutcome::Kill("Liquidity pool drift exceeded")
    } else if state.lambda_l_product >= 100 {
        SentinelOutcome::Kill("Stability product exceeded")
    } else if state.rho >= cfg.rho_warn {
        SentinelOutcome::Warn
    } else {
        SentinelOutcome::Pass
    }
}

// ─── Kani Verification Harnesses for ADR-047 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr047_sentinel_admissible_safety() {
    let rho: u32 = kani::any();
    let delta: u32 = kani::any();
    let lambda_l_product: u32 = kani::any();

    let cfg = SentinelConfig {
        rho_halt: 100,
        rho_warn: 85,
        delta_max: 10,
    };

    kani::assume(rho < cfg.rho_halt);
    kani::assume(delta < cfg.delta_max);
    kani::assume(lambda_l_product < 100);

    let state = ManifoldState {
        rho,
        delta,
        lambda_l_product,
    };

    let outcome = validate_sentinel(&state, &cfg, true);

    assert!(outcome != SentinelOutcome::Kill("Registration-time small gain failure"));
    assert!(outcome != SentinelOutcome::Kill("Drift exceeded halt threshold"));
    assert!(outcome != SentinelOutcome::Kill("Liquidity pool drift exceeded"));
    assert!(outcome != SentinelOutcome::Kill("Stability product exceeded"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentinel_outcomes() {
        let cfg = SentinelConfig {
            rho_halt: 100,
            rho_warn: 85,
            delta_max: 10,
        };

        let state_pass = ManifoldState { rho: 42, delta: 2, lambda_l_product: 50 };
        let state_warn = ManifoldState { rho: 90, delta: 2, lambda_l_product: 50 };
        let state_kill = ManifoldState { rho: 105, delta: 2, lambda_l_product: 50 };

        assert_eq!(validate_sentinel(&state_pass, &cfg, true), SentinelOutcome::Pass);
        assert_eq!(validate_sentinel(&state_warn, &cfg, true), SentinelOutcome::Warn);
        assert_eq!(validate_sentinel(&state_kill, &cfg, true), SentinelOutcome::Kill("Drift exceeded halt threshold"));
        assert_eq!(validate_sentinel(&state_pass, &cfg, false), SentinelOutcome::Kill("Registration-time small gain failure"));
    }
}
