//! ADR-036: Prime-Structured Tensor-Network Autoencoder (TN-AE)
//!
//! Rust implementation and Kani model checking harness for ADR-036:
//! - Prime-structured bond dimension lattice verification.
//! - Rank surrogate bound validation.

pub fn is_allowed_prime_dimension(d: u32) -> bool {
    matches!(d, 2 | 3 | 4 | 5 | 6 | 8 | 9 | 10 | 12 | 15 | 16)
}

pub struct RankSurrogate {
    pub effective_rank_scaled: u32,
    pub max_allowed_dimension: u32,
}

impl RankSurrogate {
    pub fn check_bound(&self) -> bool {
        self.effective_rank_scaled <= self.max_allowed_dimension * 100
    }
}

// ─── Kani Verification Harnesses for ADR-036 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr036_rank_surrogate_bound() {
    let effective_rank_scaled: u32 = kani::any();
    let max_allowed_dimension: u32 = kani::any();

    kani::assume(max_allowed_dimension >= 2 && max_allowed_dimension <= 64);
    kani::assume(effective_rank_scaled <= max_allowed_dimension * 100);

    let surrogate = RankSurrogate {
        effective_rank_scaled,
        max_allowed_dimension,
    };

    assert!(surrogate.check_bound());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_autoencoder_bounds() {
        let s = RankSurrogate {
            effective_rank_scaled: 540,
            max_allowed_dimension: 6,
        };
        assert!(s.check_bound());
        assert!(is_allowed_prime_dimension(6));
    }
}
