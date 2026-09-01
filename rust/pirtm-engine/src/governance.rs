use crate::spectral::{check_small_gain, Ensemble};
use pirtm_monitor::{ManifoldState, ManifoldStateProvider, MonitorConfig};
use serde_json::json;
use sha2::{Digest, Sha256};

fn record_audit_event(name: &str, payload: serde_json::Value) {
    println!("AUDIT EVENT: {} - {}", name, payload);
}

/// Sentinel: Governance gate integrating static small-gain check and dynamic stress bounds.
pub struct Sentinel<P: ManifoldStateProvider> {
    pub config: MonitorConfig,
    pub provider: P,
}

impl<P: ManifoldStateProvider> Sentinel<P> {
    pub fn new(provider: P, config: MonitorConfig) -> Self {
        Self { config, provider }
    }

    /// Validate static contractivity and dynamic drift bounds. Emits receipt hash on success or triggers SIG_GOV_KILL on violation.
    pub fn validate_and_seal(&mut self, ensemble: &Ensemble) -> Result<String, String> {
        // 1. Static small-gain check (Rule-HO-01)
        if let Err(e) = check_small_gain(ensemble, 1e-6) {
            self.trigger_kill(&format!("Registration-time contractivity violation: {}", e));
        }

        // 2. Dynamic drift check via ManifoldStateProvider
        let state: ManifoldState = self
            .provider
            .fetch_state()
            .map_err(|e| format!("Failed to fetch manifold state: {}", e))?;

        if state.rho >= self.config.rho_halt {
            self.trigger_kill(&format!("Drift exceeded halt threshold: rho = {}", state.rho));
        } else if state.rho >= self.config.rho_warn {
            record_audit_event("sentinel_warning", json!({ "rho": state.rho, "delta": state.delta }));
        }

        if state.delta >= self.config.delta_max {
            self.trigger_kill(&format!("Liquidity pool drift exceeded: delta = {}", state.delta));
        }

        if state.lambda_l_product >= 1.0 {
            self.trigger_kill(&format!("Stability product exceeded: lambda_l_product = {}", state.lambda_l_product));
        }

        // 3. Seal and return receipt hash
        let receipt_hash = self.generate_receipt(ensemble, state.rho, state.delta);
        record_audit_event(
            "sentinel_sealed",
            json!({
                "receipt": receipt_hash,
                "rho": state.rho,
                "delta": state.delta,
                "lambda_l_product": state.lambda_l_product
            }),
        );

        Ok(receipt_hash)
    }

    pub fn trigger_kill(&self, reason: &str) -> ! {
        eprintln!("💀 SIG_GOV_KILL: {}", reason);
        record_audit_event("sentinel_kill", json!({ "reason": reason }));
        std::process::exit(1);
    }

    fn generate_receipt(&self, ensemble: &Ensemble, rho: f64, delta: f64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}:rho={}:delta={}", ensemble, rho, delta).as_bytes());
        hex::encode(hasher.finalize())
    }
}
