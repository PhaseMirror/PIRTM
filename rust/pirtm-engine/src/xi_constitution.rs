use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CslOperators {
    pub is_neutral: bool,
    pub is_beneficent: bool,
    pub is_silent: bool,
}

impl CslOperators {
    pub fn evaluate_gate(&self) -> bool {
        self.is_neutral && self.is_beneficent && self.is_silent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawfulRecursionState {
    pub drift_delta: u64,
    pub bound_epsilon: u64,
}

impl LawfulRecursionState {
    pub fn is_lawful(&self) -> bool {
        self.drift_delta <= self.bound_epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xi_constitution_csl_and_lawful_recursion() {
        let csl = CslOperators {
            is_neutral: true,
            is_beneficent: true,
            is_silent: true,
        };
        assert!(csl.evaluate_gate());

        let rec = LawfulRecursionState {
            drift_delta: 3,
            bound_epsilon: 10,
        };
        assert!(rec.is_lawful());
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn verify_csl_gate_fail_closed() {
        let is_neutral: bool = kani::any();
        let is_beneficent: bool = kani::any();
        let is_silent: bool = kani::any();

        let csl = CslOperators { is_neutral, is_beneficent, is_silent };
        if !is_neutral || !is_beneficent || !is_silent {
            kani::assert(!csl.evaluate_gate(), "CSL gate must fail closed if any operator fails");
        }
    }
}
