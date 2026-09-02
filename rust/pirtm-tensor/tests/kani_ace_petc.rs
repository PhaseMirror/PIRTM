#[cfg(kani)]
mod tests {
    use adr_rust::ace_petc_proof::{project_weighted_l1, PrimeLedger};

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_petc_exponent_conservation() {
        let e1_2: i8 = kani::any();
        let e1_3: i8 = kani::any();
        let e2_2: i8 = kani::any();
        let e2_3: i8 = kani::any();

        kani::assume(e1_2 >= -10 && e1_2 <= 10);
        kani::assume(e1_3 >= -10 && e1_3 <= 10);
        kani::assume(e2_2 >= -10 && e2_2 <= 10);
        kani::assume(e2_3 >= -10 && e2_3 <= 10);

        let mut l1 = PrimeLedger::new();
        l1.add_event(2, e1_2.abs() as u64, if e1_2 >= 0 { 1 } else { -1 });
        l1.add_event(3, e1_3.abs() as u64, if e1_3 >= 0 { 1 } else { -1 });

        let mut l2 = PrimeLedger::new();
        l2.add_event(2, e2_2.abs() as u64, if e2_2 >= 0 { 1 } else { -1 });
        l2.add_event(3, e2_3.abs() as u64, if e2_3 >= 0 { 1 } else { -1 });

        let mut out = PrimeLedger::new();
        let sum2 = (e1_2 as i64) + (e2_2 as i64);
        let sum3 = (e1_3 as i64) + (e2_3 as i64);

        out.add_event(2, sum2.abs() as u64, if sum2 >= 0 { 1 } else { -1 });
        out.add_event(3, sum3.abs() as u64, if sum3 >= 0 { 1 } else { -1 });

        // Kani verifies that exponent addition is lossless and exactly conserved
        assert!(PrimeLedger::is_conserved(&[&l1, &l2], &out));
    }

    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_ace_projection_nonexpansiveness() {
        let w0_int: i8 = kani::any();
        let w1_int: i8 = kani::any();

        let w0 = (w0_int as f64) / 10.0;
        let w1 = (w1_int as f64) / 10.0;

        let w = vec![w0, w1];
        let b = vec![1.0, 1.0];
        let tau = 0.8;

        let w_proj = project_weighted_l1(&w, &b, tau, 20);

        // Kani verifies non-expansiveness: |w_proj[i]| <= |w[i]|
        assert!(w_proj[0].abs() <= w[0].abs() + 1e-6);
        assert!(w_proj[1].abs() <= w[1].abs() + 1e-6);

        // Kani verifies budget satisfaction: sum b_i |w_proj_i| <= tau
        let budget: f64 = w_proj.iter().zip(b.iter()).map(|(&wi, &bi)| bi * wi.abs()).sum();
        assert!(budget <= tau + 1e-6);
    }
}
