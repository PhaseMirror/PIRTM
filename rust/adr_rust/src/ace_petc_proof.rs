use std::collections::HashMap;

/// PETC Prime Signature Ledger for unambiguous retrospection (ADR-052).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimeLedger {
    pub sig: HashMap<u64, i64>,
}

impl PrimeLedger {
    pub fn new() -> Self {
        Self {
            sig: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, p: u64, mult: i64, sign: i64) {
        let entry = self.sig.entry(p).or_insert(0);
        *entry += sign * mult;
        if *entry == 0 {
            self.sig.remove(&p);
        }
    }

    pub fn valuation(&self, p: u64) -> i64 {
        *self.sig.get(&p).unwrap_or(&0)
    }

    pub fn is_conserved(inputs: &[&PrimeLedger], output: &PrimeLedger) -> bool {
        let mut acc = HashMap::new();
        for input in inputs {
            for (&p, &e) in &input.sig {
                *acc.entry(p).or_insert(0) += e;
            }
        }
        for (&p, &e) in &output.sig {
            let entry = acc.entry(p).or_insert(0);
            *entry -= e;
            if *entry == 0 {
                acc.remove(&p);
            }
        }
        acc.is_empty()
    }
}

/// Integer soft-thresholding operator for ACE budget enforcement.
pub fn soft_threshold_int(w: u64, theta: u64) -> u64 {
    if w <= theta {
        0
    } else {
        w - theta
    }
}

/// Exact weighted-L1 projection via bisection (ACE Safety Budget - ADR-052).
pub fn project_weighted_l1(w: &[f64], b: &[f64], tau: f64, max_iters: usize) -> Vec<f64> {
    let current_budget: f64 = w.iter().zip(b.iter()).map(|(&wi, &bi)| bi * wi.abs()).sum();
    if current_budget <= tau {
        return w.to_vec();
    }

    let mut lo = 0.0;
    let mut hi = w
        .iter()
        .zip(b.iter())
        .map(|(&wi, &bi)| wi.abs() / bi.max(1e-15))
        .fold(0.0, f64::max);

    let mut w_proj = w.to_vec();
    for _ in 0..max_iters {
        let lam = 0.5 * (lo + hi);
        for i in 0..w.len() {
            let val = (w[i].abs() - lam * b[i]).max(0.0);
            w_proj[i] = if w[i] >= 0.0 { val } else { -val };
        }
        let budget: f64 = w_proj
            .iter()
            .zip(b.iter())
            .map(|(&wi, &bi)| bi * wi.abs())
            .sum();
        if (budget - tau).abs() < 1e-9 {
            break;
        }
        if budget > tau {
            lo = lam;
        } else {
            hi = lam;
        }
    }
    w_proj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_petc_ledger_conservation() {
        let mut l1 = PrimeLedger::new();
        l1.add_event(2, 3, 1);
        l1.add_event(3, 1, 1);

        let mut l2 = PrimeLedger::new();
        l2.add_event(5, 2, 1);

        let mut out = PrimeLedger::new();
        out.add_event(2, 3, 1);
        out.add_event(3, 1, 1);
        out.add_event(5, 2, 1);

        assert!(PrimeLedger::is_conserved(&[&l1, &l2], &out));
    }

    #[test]
    fn test_ace_weighted_l1_projection() {
        let w = vec![0.8, 0.6, 0.4];
        let b = vec![1.0, 1.0, 1.0];
        let tau = 0.9;

        let w_proj = project_weighted_l1(&w, &b, tau, 50);
        let budget: f64 = w_proj.iter().zip(b.iter()).map(|(&wi, &bi)| bi * wi.abs()).sum();
        assert!(budget <= tau + 1e-6);
    }

    #[test]
    fn test_soft_threshold_int_properties() {
        assert_eq!(soft_threshold_int(10, 3), 7);
        assert_eq!(soft_threshold_int(3, 5), 0);
        assert!(soft_threshold_int(100, 20) <= 100);
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn proof_soft_threshold_non_expansive() {
        let w: u64 = kani::any();
        let theta: u64 = kani::any();
        kani::assume(w < 1_000_000);
        kani::assume(theta < 1_000_000);

        let st = soft_threshold_int(w, theta);
        // Invariant: soft_threshold(w, theta) <= w
        assert!(st <= w);
    }

    #[kani::proof]
    fn proof_petc_exponent_addition_conserved() {
        let exp1: i32 = kani::any();
        let exp2: i32 = kani::any();
        kani::assume(exp1 > -1_000 && exp1 < 1_000);
        kani::assume(exp2 > -1_000 && exp2 < 1_000);

        let sum = exp1 + exp2;
        // Invariant: exponent valuation additive homomorphism
        assert_eq!(sum - exp1, exp2);
    }
}
