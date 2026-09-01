//! ADR-045: UI/UX Integration for PIRTM
//!
//! Rust implementation and Kani model checking harness for ADR-045:
//! - In-browser WASM compilation & UI request evaluation.
//! - Contractivity gate check (\rho < 1.0) and MLIR receipt generation.

#[derive(Debug, Clone)]
pub struct UiCompileRequest {
    pub code_source: String,
    pub spectral_radius: u32, // \rho * 100
    pub is_read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiCompileReceipt {
    pub mlir_generated: bool,
    pub receipt_hash: u64,
    pub is_admissible: bool,
}

pub fn evaluate_ui_request(req: &UiCompileRequest) -> UiCompileReceipt {
    let admissible = req.spectral_radius < 100;
    UiCompileReceipt {
        mlir_generated: admissible,
        receipt_hash: (req.code_source.len() as u64) + (req.spectral_radius as u64),
        is_admissible: admissible,
    }
}

// ─── Kani Verification Harnesses for ADR-045 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr045_ui_contractivity_gate() {
    let spectral_radius: u32 = kani::any();
    let is_read_only: bool = kani::any();

    kani::assume(spectral_radius <= 200);

    let req = UiCompileRequest {
        code_source: "Ap(2)".to_string(),
        spectral_radius,
        is_read_only,
    };

    let receipt = evaluate_ui_request(&req);

    if spectral_radius < 100 {
        assert!(receipt.is_admissible);
        assert!(receipt.mlir_generated);
    } else {
        assert!(!receipt.is_admissible);
        assert!(!receipt.mlir_generated);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_compile_evaluation() {
        let req_pass = UiCompileRequest {
            code_source: "Ap(2) + 3".to_string(),
            spectral_radius: 85,
            is_read_only: false,
        };
        let req_fail = UiCompileRequest {
            code_source: "Ap(2) + 3".to_string(),
            spectral_radius: 110,
            is_read_only: false,
        };

        let receipt_pass = evaluate_ui_request(&req_pass);
        let receipt_fail = evaluate_ui_request(&req_fail);

        assert!(receipt_pass.is_admissible);
        assert!(receipt_pass.mlir_generated);
        assert!(!receipt_fail.is_admissible);
        assert!(!receipt_fail.mlir_generated);
    }
}
