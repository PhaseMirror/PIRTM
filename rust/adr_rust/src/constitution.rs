//! ADR-042: Prime-Constitutional Order & Conscious Sovereignty Layer (CSL)
//!
//! Rust implementation and Kani model checking harness for ADR-042:
//! - CSL intent operators (Neutrality, Beneficence, Silence).
//! - Gate evaluation logic defaulting to NO-OP.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CslIntent {
    pub is_neutral: bool,
    pub is_beneficent: bool,
    pub is_silence_safe: bool,
}

pub fn evaluate_csl(intent: &CslIntent) -> bool {
    intent.is_neutral && intent.is_beneficent && intent.is_silence_safe
}

// ─── Kani Verification Harnesses for ADR-042 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr042_csl_gate_soundness() {
    let is_neutral: bool = kani::any();
    let is_beneficent: bool = kani::any();
    let is_silence_safe: bool = kani::any();

    let intent = CslIntent {
        is_neutral,
        is_beneficent,
        is_silence_safe,
    };

    if evaluate_csl(&intent) {
        assert!(is_neutral);
        assert!(is_beneficent);
        assert!(is_silence_safe);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csl_gate_evaluation() {
        let intent_pass = CslIntent {
            is_neutral: true,
            is_beneficent: true,
            is_silence_safe: true,
        };
        let intent_fail = CslIntent {
            is_neutral: true,
            is_beneficent: false,
            is_silence_safe: true,
        };

        assert!(evaluate_csl(&intent_pass));
        assert!(!evaluate_csl(&intent_fail));
    }
}
