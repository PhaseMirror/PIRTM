//! ADR-048: Formal WardMonitor Drift Correction & Lyapunov Stability
//!
//! Rust model and Kani proof harness for WardMonitor Zeno attenuation
//! and Lyapunov stability verification.

pub fn apply_zeno_gain(rho: u32, kappa: u32) -> u32 {
    let k = if kappa > 100 { 100 } else { kappa };
    (rho * (100 - k)) / 100
}

pub fn lyapunov_energy(rho: u32) -> u64 {
    (rho as u64) * (rho as u64)
}

#[cfg(kani)]
#[kani::proof]
fn verify_adr048_ward_monitor_lyapunov_stability() {
    let rho: u32 = kani::any();
    let kappa: u32 = kani::any();

    kani::assume(kappa <= 100);

    let attenuated = apply_zeno_gain(rho, kappa);
    assert!(attenuated <= rho);
    assert!(lyapunov_energy(attenuated) <= lyapunov_energy(rho));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ward_monitor_attenuation_and_lyapunov() {
        let rho = 90;
        let kappa = 15;

        let att = apply_zeno_gain(rho, kappa);
        assert!(att <= rho);
        assert!(lyapunov_energy(att) <= lyapunov_energy(rho));
    }
}
