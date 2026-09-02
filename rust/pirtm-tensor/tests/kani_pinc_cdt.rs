#[cfg(kani)]
mod tests {
    use adr_rust::pinc_cdt_proof::{compute_spectral_dimension_proxy, PincCdtSector, PincCdtSimplex};

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_pinc_cdt_action_density_bounds() {
        let eps_int: u8 = kani::any();
        let area_int: u8 = kani::any();
        let ncg_int: u8 = kani::any();

        kani::assume(eps_int <= 50);
        kani::assume(area_int <= 50);
        kani::assume(ncg_int <= 50);

        let eps = (eps_int as f64) / 10.0;
        let area = (area_int as f64) / 10.0;
        let ncg = (ncg_int as f64) / 10.0;

        let simplex = PincCdtSimplex {
            weight: 1.0,
            epsilon: eps,
            area,
            ncg_term: ncg,
        };
        let sector = PincCdtSector {
            prime: 2,
            theta: 1.5,
            simplices: vec![simplex],
        };

        let action = sector.action_density(0, 0.2);

        // Kani verifies that action density is non-negative and strictly bounded by maximum K_s
        assert!(action >= 0.0);
        assert!(action <= 60.0);
    }

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_spectral_dimension_bounds() {
        let avg_eps_int: u8 = kani::any();
        let eps_max_int: u8 = kani::any();

        kani::assume(avg_eps_int <= 100);
        kani::assume(eps_max_int >= 1 && eps_max_int <= 100);

        let avg_eps = (avg_eps_int as f64) / 10.0;
        let eps_max = (eps_max_int as f64) / 10.0;

        let ds = compute_spectral_dimension_proxy(avg_eps, eps_max);

        // Kani verifies that spectral dimension proxy is strictly bounded within valid CDT range [1.2, 2.0]
        assert!(ds >= 1.2);
        assert!(ds <= 2.0);
    }
}
