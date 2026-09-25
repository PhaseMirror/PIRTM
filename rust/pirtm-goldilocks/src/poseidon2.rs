use crate::GoldilocksField;
use serde::{Deserialize, Serialize};

use p3_goldilocks::{Goldilocks, default_goldilocks_poseidon2_8};
use p3_symmetric::Permutation;
use p3_field::PrimeField64;

/// Poseidon2 Sponge State for Goldilocks Field (Width 8)
/// Implemented via p3-poseidon2 / p3-goldilocks wrap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poseidon2Sponge {
    pub state: [GoldilocksField; 8],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poseidon2ProofReceipt {
    pub hash_output: [u64; 4],
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

    pub fn absorb(&mut self, inputs: &[u64]) {
        for (i, &val) in inputs.iter().enumerate().take(4) {
            self.state[i] = self.state[i] + GoldilocksField::new(val);
        }
        self.permute();
    }

    pub fn permute(&mut self) {
        let mut p3_state: [Goldilocks; 8] = [Goldilocks::new(0); 8];
        for i in 0..8 {
            p3_state[i] = Goldilocks::new(self.state[i].to_canonical());
        }
        
        let mut poseidon = default_goldilocks_poseidon2_8();
        poseidon.permute_mut(&mut p3_state);
        
        for i in 0..8 {
            self.state[i] = GoldilocksField::from_canonical(p3_state[i].as_canonical_u64());
        }
    }

    pub fn squeeze(&mut self) -> Poseidon2ProofReceipt {
        self.permute();
        let hash_output = [
            self.state[0].to_canonical(),
            self.state[1].to_canonical(),
            self.state[2].to_canonical(),
            self.state[3].to_canonical(),
        ];
        Poseidon2ProofReceipt { hash_output }
    }
    
    pub fn compress(left: [u64; 4], right: [u64; 4]) -> [u64; 4] {
        let mut sponge = Self::new();
        for i in 0..4 {
            sponge.state[i] = GoldilocksField::new(left[i]);
            sponge.state[i+4] = GoldilocksField::new(right[i]);
        }
        sponge.permute();
        [
            sponge.state[0].to_canonical(),
            sponge.state[1].to_canonical(),
            sponge.state[2].to_canonical(),
            sponge.state[3].to_canonical(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_goldilocks::Goldilocks;
    use p3_field::PrimeField64;

    #[test]
    fn test_known_answer_width_8() {
        use p3_field::PrimeField64;
        let mut sponge = Poseidon2Sponge::new();
        let mut state = [crate::GoldilocksField(0); 8];
        for i in 0..8 {
            state[i] = crate::GoldilocksField(i as u64);
        }
        sponge.state = state;
        
        sponge.permute();
        
        let expected = [
            0x020cf04a1b214d14,
            0x84e14aaaeacaed25,
            0x1ae0f640e81c7457,
            0xa4d204cbaeb0d8a5,
            0x0cf637b627b3a7ff,
            0x788d304d948b486b,
            0x7327133ea1949af4,
            0xf415abb924da395b,
        ];
        
        for i in 0..8 {
            assert_eq!(sponge.state[i].0, expected[i], "Mismatch at index {}", i);
        }
    }
}
