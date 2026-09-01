pub mod loader;
pub mod telemetry;
pub mod ffi;
pub mod monitor;
pub mod harmonia;
pub mod spectral;

use std::path::{Path, PathBuf};
use serde_json::json;
use sha2::{Sha256, Digest};
use std::process::{Command, Stdio};
use std::io::Write;
pub use spectral::{Ensemble, EnsembleContractivityReceipt, check_small_gain, validate_and_certify};

#[derive(Debug, Default)]
pub struct RuntimeConfig {
    pub jid_enabled: bool,
    pub ledger_enabled: bool,
    pub enforce_bounds: bool,
    pub input_args: Vec<String>,
}

pub struct ExecutionReceipt {
    pub return_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub metrics: telemetry::TelemetryMetrics,
    pub contractivity_hash: String,
}

pub struct Runtime {
    pub config: RuntimeConfig,
    ll_path: Option<PathBuf>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            ll_path: None,
        }
    }

    pub fn validate_ensemble(&self, ensemble: &Ensemble) -> Result<EnsembleContractivityReceipt, String> {
        let cert = spectral::validate_and_certify(ensemble, 1e-6)?;
        println!(
            "AUDIT EVENT: ensemble_validated - {}",
            json!({
                "ensemble_name": cert.ensemble_name,
                "dimension": cert.dimension,
                "spectral_radius": cert.spectral_radius,
                "is_stable": cert.is_stable,
                "receipt_hash": cert.hash,
            })
        );
        Ok(cert)
    }

    pub fn load(&mut self, mlir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut ll_path = mlir_path.to_path_buf();
        ll_path.set_extension("ll");
        self.ll_path = Some(ll_path);
        Ok(())
    }

    pub fn run(&mut self) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
        let metrics = telemetry::simulate_telemetry_collection();
        
        let mut stdout_buf = String::new();
        if !self.config.input_args.is_empty() {
            stdout_buf.push_str(&format!("Simulated output for input: {}\n", self.config.input_args.join(" ")));
        }

        let hash = if self.config.ledger_enabled {
            let mut hasher = Sha256::new();
            let data = format!("{:?}", metrics);
            hasher.update(data);
            format!("{:x}", hasher.finalize())
        } else {
            "no-ledger".to_string()
        };

        Ok(ExecutionReceipt {
            return_code: 0,
            stdout: stdout_buf,
            stderr: String::new(),
            metrics,
            contractivity_hash: hash,
        })
    }
}
