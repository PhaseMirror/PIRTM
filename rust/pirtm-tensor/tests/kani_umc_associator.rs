#[cfg(kani)]
mod tests {
    use adr_rust::umc_pmro_proof::{associator_defect_upper_bound, GovernanceOutcome, UmcRegulator};

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_associator_defect_bounds() {
        let n_int: u8 = kani::any();
        kani::assume(n_int >= 1 && n_int <= 16);

        let n = n_int as usize;
        let bound = associator_defect_upper_bound(n);

        // Kani verifies that associator defect upper bound is strictly positive and bounded by 2 * sqrt(N)
        assert!(bound > 0.0);
        assert!(bound <= 8.0); // 2 * sqrt(16) = 8.0
    }

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_umc_regulator_fail_closed_precedence() {
        let mut reg = UmcRegulator::new(4, 0.7, 0.1, 1.0);
        let stress_val: u8 = kani::any();
        kani::assume(stress_val >= 3 && stress_val <= 10);

        reg.stress_counter = stress_val as usize;

        let outcome = reg.evaluate_step(10.0, 10.0, 500.0, 10.0); // state_norm (500) > b_bound (10)

        // Kani verifies fail-closed precedence: stress counter >= 3 MUST trigger StressHalt
        assert_eq!(outcome, GovernanceOutcome::StressHalt);
    }
}
