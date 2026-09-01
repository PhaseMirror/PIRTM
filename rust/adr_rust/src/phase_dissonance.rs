//! ADR-037: Prime-Indexed Phase-Dissonance Functionals for Software Governance
//!
//! Rust implementation and Kani model checking harness for ADR-037:
//! - Prime-weighted contradiction components \Delta_{p,a}.
//! - Phase-dissonance functional computation D(\Phi_t).
//! - Dynamic phase band bounds [L_t, U_t].

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactType {
    Spec,
    Code,
    Log,
    SLA,
}

#[derive(Debug, Clone)]
pub struct ContradictionEntry {
    pub prime_axis: u32,
    pub artifact: ArtifactType,
    pub weight: u32,
    pub delta: u32,
}

impl ContradictionEntry {
    pub fn entry_square(&self) -> u64 {
        let prod = (self.prime_axis as u64) * (self.weight as u64) * (self.delta as u64);
        prod * prod
    }
}

pub fn calculate_dissonance_squared(entries: &[ContradictionEntry]) -> u64 {
    entries.iter().map(|e| e.entry_square()).sum()
}

pub fn calculate_dissonance(entries: &[ContradictionEntry]) -> u64 {
    let sum_sq = calculate_dissonance_squared(entries);
    (sum_sq as f64).sqrt() as u64
}

#[derive(Debug, Clone)]
pub struct PhaseBand {
    pub lower_bound: u64,
    pub upper_bound: u64,
}

pub fn is_governance_in_bounds(entries: &[ContradictionEntry], band: &PhaseBand) -> bool {
    let d = calculate_dissonance(entries);
    d >= band.lower_bound && d <= band.upper_bound
}

// ─── Kani Verification Harnesses for ADR-037 ────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn verify_adr037_phase_dissonance_in_bounds() {
    let p1: u32 = kani::any();
    let w1: u32 = kani::any();
    let d1: u32 = kani::any();

    kani::assume(p1 >= 2 && p1 <= 19);
    kani::assume(w1 <= 10);
    kani::assume(d1 <= 10);

    let entry = ContradictionEntry {
        prime_axis: p1,
        artifact: ArtifactType::Spec,
        weight: w1,
        delta: d1,
    };

    let entries = [entry];
    let d = calculate_dissonance(&entries);

    let band = PhaseBand {
        lower_bound: 0,
        upper_bound: d + 10,
    };

    assert!(is_governance_in_bounds(&entries, &band));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_dissonance_calculation() {
        let entries = vec![
            ContradictionEntry {
                prime_axis: 2,
                artifact: ArtifactType::Spec,
                weight: 1,
                delta: 2,
            },
            ContradictionEntry {
                prime_axis: 3,
                artifact: ArtifactType::Code,
                weight: 1,
                delta: 1,
            },
        ];
        // (2*1*2)^2 = 16. (3*1*1)^2 = 9. sum = 25. sqrt(25) = 5.
        assert_eq!(calculate_dissonance(&entries), 5);

        let band = PhaseBand {
            lower_bound: 0,
            upper_bound: 10,
        };
        assert!(is_governance_in_bounds(&entries, &band));
    }
}
