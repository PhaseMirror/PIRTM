//! ADR-035: Prime-Encoded Quantum States & Subspace Error Detection
//!
//! Rust implementation and Kani model checking harness for ADR-035:
//! - Prime subspace syndrome operator S_P = 2\Pi_P - I.
//! - Subspace error detection primitives.

pub fn is_prime_basis(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Syndrome eigenvalue: +1 for prime basis states, -1 for composite/non-prime states.
pub fn prime_syndrome_eigenvalue(n: u64) -> i8 {
    if is_prime_basis(n) {
        1
    } else {
        -1
    }
}

pub struct PrimeSubspaceState {
    pub basis_state: u64,
    pub num_qubits: u8,
}

impl PrimeSubspaceState {
    pub fn new(basis_state: u64, num_qubits: u8) -> Option<Self> {
        if num_qubits < 64 && basis_state >= (1u64 << num_qubits) {
            None
        } else {
            Some(Self { basis_state, num_qubits })
        }
    }

    pub fn syndrome(&self) -> i8 {
        prime_syndrome_eigenvalue(self.basis_state)
    }

    pub fn is_in_prime_subspace(&self) -> bool {
        self.syndrome() == 1
    }
}

// ─── Kani Verification Harnesses for ADR-035 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr035_prime_syndrome_invariants() {
    let basis_state: u64 = kani::any();
    kani::assume(basis_state <= 255);

    let state = PrimeSubspaceState::new(basis_state, 8).unwrap();
    let syn = state.syndrome();

    if state.is_in_prime_subspace() {
        assert_eq!(syn, 1);
        assert!(is_prime_basis(basis_state));
    } else {
        assert_eq!(syn, -1);
        assert!(!is_prime_basis(basis_state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_syndrome_values() {
        let p_state = PrimeSubspaceState::new(13, 8).unwrap();
        let c_state = PrimeSubspaceState::new(14, 8).unwrap();

        assert_eq!(p_state.syndrome(), 1);
        assert!(p_state.is_in_prime_subspace());

        assert_eq!(c_state.syndrome(), -1);
        assert!(!c_state.is_in_prime_subspace());
    }
}
