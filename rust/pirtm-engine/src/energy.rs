use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnergyLedgerState {
    pub v_pair: u64,
    pub v_nuc: u64,
}

impl EnergyLedgerState {
    pub fn new(v_pair: u64, v_nuc: u64) -> Self {
        Self { v_pair, v_nuc }
    }

    pub fn calculate_total_energy(&self) -> i64 {
        self.v_pair as i64 - self.v_nuc as i64
    }

    pub fn is_ground_state(&self, other: &EnergyLedgerState) -> bool {
        self.calculate_total_energy() <= other.calculate_total_energy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_ledger_ground_state() {
        let e1 = EnergyLedgerState::new(10, 15);
        let e2 = EnergyLedgerState::new(12, 8);

        assert_eq!(e1.calculate_total_energy(), -5);
        assert_eq!(e2.calculate_total_energy(), 4);
        assert!(e1.is_ground_state(&e2));
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn verify_ground_state_energy_minimization() {
        let v_pair: u64 = kani::any();
        let v_nuc1: u64 = kani::any();
        let v_nuc2: u64 = kani::any();

        kani::assume(v_nuc1 <= v_nuc2);
        kani::assume(v_pair < i64::MAX as u64);
        kani::assume(v_nuc2 < i64::MAX as u64);

        let e1 = EnergyLedgerState::new(v_pair, v_nuc1);
        let e2 = EnergyLedgerState::new(v_pair, v_nuc2);

        kani::assert(e2.is_ground_state(&e1), "Higher purpose attraction must yield lower or equal energy");
    }
}
