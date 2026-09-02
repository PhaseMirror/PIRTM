/// Collaborative CRDT State & Governance Preservation Engine (ADR-056).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrdtDocumentState {
    pub clock_alice: u64,
    pub clock_bob: u64,
    pub norm_num: u64,
    pub norm_den: u64,
}

impl CrdtDocumentState {
    pub fn new(clock_alice: u64, clock_bob: u64, norm_num: u64, norm_den: u64) -> Self {
        assert!(norm_den > 0, "Denominator must be strictly positive");
        Self {
            clock_alice,
            clock_bob,
            norm_num,
            norm_den,
        }
    }

    pub fn is_contractive(&self) -> bool {
        self.norm_num < self.norm_den
    }

    pub fn merge(&self, other: &Self) -> Self {
        let clock_alice = self.clock_alice.max(other.clock_alice);
        let clock_bob = self.clock_bob.max(other.clock_bob);

        // Max exact rational evaluation: max(num1/den1, num2/den2)
        let scaled_1 = (self.norm_num as u128) * (other.norm_den as u128);
        let scaled_2 = (other.norm_num as u128) * (self.norm_den as u128);

        let max_scaled = scaled_1.max(scaled_2);
        let common_den = (self.norm_den as u128) * (other.norm_den as u128);

        // Reduce back to u64 bounds safely
        let g = gcd(max_scaled, common_den);
        let num = (max_scaled / g) as u64;
        let den = (common_den / g) as u64;

        Self {
            clock_alice,
            clock_bob,
            norm_num: num,
            norm_den: den,
        }
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crdt_merge_commutativity() {
        let s1 = CrdtDocumentState::new(1, 0, 1, 2);
        let s2 = CrdtDocumentState::new(0, 2, 3, 5);

        let m1 = s1.merge(&s2);
        let m2 = s2.merge(&s1);

        assert_eq!(m1, m2);
        assert!(m1.is_contractive());
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof]
    fn proof_crdt_merge_commutative() {
        let c1_a: u64 = kani::any();
        let c1_b: u64 = kani::any();
        let n1: u64 = kani::any();
        let d1: u64 = kani::any();

        let c2_a: u64 = kani::any();
        let c2_b: u64 = kani::any();
        let n2: u64 = kani::any();
        let d2: u64 = kani::any();

        kani::assume(d1 > 0 && d1 < 100);
        kani::assume(d2 > 0 && d2 < 100);
        kani::assume(n1 < d1);
        kani::assume(n2 < d2);

        let s1 = CrdtDocumentState::new(c1_a, c1_b, n1, d1);
        let s2 = CrdtDocumentState::new(c2_a, c2_b, n2, d2);

        let m1 = s1.merge(&s2);
        let m2 = s2.merge(&s1);

        assert_eq!(m1.clock_alice, m2.clock_alice);
        assert_eq!(m1.clock_bob, m2.clock_bob);
        assert_eq!(m1.norm_num, m2.norm_num);
        assert_eq!(m1.norm_den, m2.norm_den);
    }

    #[kani::proof]
    fn proof_crdt_governance_preserved() {
        let n1: u64 = kani::any();
        let d1: u64 = kani::any();
        let n2: u64 = kani::any();
        let d2: u64 = kani::any();

        kani::assume(d1 > 0 && d1 < 100);
        kani::assume(d2 > 0 && d2 < 100);
        kani::assume(n1 < d1);
        kani::assume(n2 < d2);

        let s1 = CrdtDocumentState::new(1, 0, n1, d1);
        let s2 = CrdtDocumentState::new(0, 1, n2, d2);

        let merged = s1.merge(&s2);
        assert!(merged.is_contractive());
    }
}
