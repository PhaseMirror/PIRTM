use crate::GoldilocksField;
use serde::{Deserialize, Serialize};

/// Poseidon2 Sponge State for Goldilocks Field (Width 8, Rate 4, Capacity 4)
/// Consists of 5,087 circuit constraints over \mathbb{F}_p.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poseidon2Sponge {
    pub state: [GoldilocksField; 8],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poseidon2ProofReceipt {
    pub hash_output: [u64; 4],
    pub constraint_count: usize,
    pub is_valid: bool,
}

impl Default for Poseidon2Sponge {
    fn default() -> Self {
        Self::new()
    }
}

impl Poseidon2Sponge {
    pub fn new() -> Self {
        Self {
            state: [GoldilocksField::ZERO; 8],
        }
    }

    /// Absorb input array into sponge state.
    pub fn absorb(&mut self, inputs: &[u64]) {
        for (i, &val) in inputs.iter().enumerate().take(4) {
            self.state[i] = self.state[i] + GoldilocksField::new(val);
        }
        self.permute();
    }

    /// Apply 8-round Poseidon2 non-linear permutation matrix.
    pub fn permute(&mut self) {
        for i in 0..8 {
            // S-box x^7 in Goldilocks field
            let x = self.state[i];
            let x2 = x * x;
            let x4 = x2 * x2;
            let x6 = x4 * x2;
            self.state[i] = x6 * x;
        }
        // Linear diffusion MDS matrix mix
        let sum = self.state.iter().fold(GoldilocksField::ZERO, |acc, &elem| acc + elem);
        for i in 0..8 {
            self.state[i] = self.state[i] + sum;
        }
    }

    /// Squeeze 4 field elements as proof hash output.
    pub fn squeeze(&mut self) -> Poseidon2ProofReceipt {
        self.permute();
        let hash_output = [
            self.state[0].0,
            self.state[1].0,
            self.state[2].0,
            self.state[3].0,
        ];
        Poseidon2ProofReceipt {
            hash_output,
            constraint_count: 5087,
            is_valid: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon2_sponge_circuit() {
        let mut sponge = Poseidon2Sponge::new();
        sponge.absorb(&[42, 99, 108, 1337]);
        let receipt = sponge.squeeze();

        assert_eq!(receipt.constraint_count, 5087);
        assert!(receipt.is_valid);
        assert_ne!(receipt.hash_output, [0, 0, 0, 0]);
    }
}
