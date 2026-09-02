use crate::spectral::{validate_and_certify, Ensemble};
use pirtm_monitor::{ManifoldState, ManifoldStateProvider, MonitorConfig};
use serde_json::json;
use sha2::{Digest, Sha256};

fn record_audit_event(name: &str, payload: serde_json::Value) {
    println!("AUDIT EVENT: {} - {}", name, payload);
}

/// Sentinel: Governance gate integrating certified small-gain and dynamic stress bounds.
pub struct Sentinel<P: ManifoldStateProvider> {
    pub config: MonitorConfig,
    pub provider: P,
}

impl<P: ManifoldStateProvider> Sentinel<P> {
    pub fn new(provider: P, config: MonitorConfig) -> Self {
        Self { config, provider }
    }

    /// Require a certified ensemble (theorem_name present) before sealing.
    /// MissingTheoremAnchor returns Err and does not stamp a WORM receipt.
    pub fn validate_and_seal(&mut self, ensemble: &Ensemble) -> Result<String, String> {
<<<<<<< HEAD
        // 1. Certified small-gain (Rule-HO-01). Raw ρ is not a seal.
        let cert = match validate_and_certify(ensemble, 1e-6) {
            Ok(c) => c,
            Err(e) if e.contains("MissingTheoremAnchor") => {
                return Err(e);
            }
            Err(e) => {
                self.trigger_kill(&format!("Registration-time contractivity violation: {}", e));
            }
=======
        // 1. Static small-gain check (Rule-HO-01)
        let cert = match validate_and_certify(ensemble, 1e-6) {
            Ok(c) => c,
            Err(e) => self.trigger_kill(&format!("Registration-time contractivity violation: {}", e)),
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
        };

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

        // 3. Seal and return receipt hash bound to the certified theorem_name
        let receipt_hash = self.generate_receipt(ensemble, &cert.hash, state.rho, state.delta);
        record_audit_event(
            "sentinel_sealed",
            json!({
                "receipt": receipt_hash,
                "cert_hash": cert.hash,
                "theorem_name": cert.theorem_name,
                "rho": state.rho,
                "delta": state.delta,
                "lambda_l_product": state.lambda_l_product
            }),
        );

        Ok(receipt_hash)
    }

    pub fn trigger_kill(&self, reason: &str) -> ! {
        eprintln!("SIG_GOV_KILL: {}", reason);
        record_audit_event("sentinel_kill", json!({ "reason": reason }));
        std::process::exit(1);
    }

    fn generate_receipt(&self, ensemble: &Ensemble, cert_hash: &str, rho: f64, delta: f64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(ensemble.name.as_bytes());
        hasher.update(ensemble.theorem_name.as_bytes());
        hasher.update(cert_hash.as_bytes());
        hasher.update(format!("rho={}:delta={}", rho, delta).as_bytes());
        hex::encode(hasher.finalize())
    }
}
