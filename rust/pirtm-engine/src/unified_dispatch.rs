//! Unified Stack Certification & Pre-Commit Dispatch Loop
//!
//! Provides the runtime binding for the formal LexLean `certifyUnifiedStack` specification
//! (derived from `Main.lex.tex`, `OpenCore.lex.tex`, `RecurrenceContractivity.lex.tex`,
//! and `SubstrateLawfulness.lex.tex`).
//!
//! Every state transition must pass through the `UnifiedDispatchLoop` before committing.
//! Any violation of:
//!   - Monotonic sequence indices (SL5 / PWEH)
//!   - Substrate lawfulness & static criteria (SL1)
//!   - Contraction slope bounds (SL3)
//!   - Residual energy bounds (SL4 / L0_HALT)
//!   - Small-gain / recurrence perturbation bounds (SG2 / Weyl)
//!   - OpenCore invariant bounds (C1, C2, C3)
//! results in an immediate fail-closed transition abort.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Substrate operating state from LexLean `OpenCore::SubstrateState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubstrateState {
    Active,
    Halted(u64),
}

impl SubstrateState {
    pub fn is_active(&self) -> bool {
        matches!(self, SubstrateState::Active)
    }
}

/// Recurrence contractivity state from LexLean `RecurrenceContractivity::RecurrenceState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrenceState {
    Contractive,
    NonContractive(u64),
}

impl RecurrenceState {
    pub fn is_contractive(&self) -> bool {
        matches!(self, RecurrenceState::Contractive)
    }
}

/// Substrate lawfulness status from LexLean `SubstrateLawfulness::LawfulnessStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawfulnessStatus {
    Lawful,
    L0Halt(u64),
}

impl LawfulnessStatus {
    pub fn is_lawful(&self) -> bool {
        matches!(self, LawfulnessStatus::Lawful)
    }
}

/// Helper function to compute modular 2^n for n in {0, 1, 2, ...}
/// exactly as defined in `PrimeMonoid.lex.tex`:
/// powTwo 0 = 1, powTwo 1 = 2, powTwo 2 = 4, powTwo n = 8.
#[inline]
pub fn pow_two(exp: u64) -> u64 {
    match exp {
        0 => 1,
        1 => 2,
        2 => 4,
        _ => 8,
    }
}

/// Helper function to compute modular 3^n for n in {0, 1, 2, ...}
/// exactly as defined in `PrimeMonoid.lex.tex`:
/// powThree 0 = 1, powThree 1 = 3, powThree 2 = 9, powThree n = 27.
#[inline]
pub fn pow_three(exp: u64) -> u64 {
    match exp {
        0 => 1,
        1 => 3,
        2 => 9,
        _ => 27,
    }
}

/// Encodes two exponents into 2^p * 3^s (`PrimeMonoid::encodeTwoThree`).
#[inline]
pub fn encode_two_three(p: u64, s: u64) -> u64 {
    pow_two(p).saturating_mul(pow_three(s))
}

/// Evaluates OpenCore step invariants (`OpenCore::evaluateStep`).
#[inline]
pub fn evaluate_open_core_step(
    comm: u64,
    eps: u64,
    gamma: u64,
    rho: u64,
    r_min: u64,
    r_max: u64,
) -> SubstrateState {
    let c1 = comm <= eps;
    let c2 = gamma <= 1000;
    let c3 = r_min <= rho && rho <= r_max;
    if c1 && c2 && c3 {
        SubstrateState::Active
    } else {
        SubstrateState::Halted(1)
    }
}

/// Evaluates recurrence contractivity under Weyl perturbation (`RecurrenceContractivity::evaluateRecurrence`).
#[inline]
pub fn evaluate_recurrence(
    base_radius: u64,
    perturbation: u64,
    margin: u64,
) -> RecurrenceState {
    let sum = base_radius.saturating_add(perturbation).saturating_add(margin);
    if sum <= 1000 {
        RecurrenceState::Contractive
    } else {
        RecurrenceState::NonContractive(1)
    }
}

/// Evaluates substrate lawfulness criteria (`SubstrateLawfulness::evaluateSubstrate`).
#[inline]
pub fn evaluate_substrate(
    c_all: bool,
    residual: u64,
    res_eps: u64,
    slope: u64,
) -> LawfulnessStatus {
    if !c_all {
        LawfulnessStatus::L0Halt(1)
    } else if residual > res_eps {
        LawfulnessStatus::L0Halt(2)
    } else if slope >= 1000 {
        LawfulnessStatus::L0Halt(3)
    } else {
        LawfulnessStatus::Lawful
    }
}

/// Computes the PWEH hash step (`SubstrateLawfulness::pwehHashStep`).
#[inline]
pub fn pweh_hash_step(prev_hash: u64, state_hash: u64, seq_idx: u64) -> u64 {
    let prefix = encode_two_three(prev_hash, state_hash);
    prefix.saturating_mul(seq_idx)
}

/// Checks strict monotonicity of sequence indices (`SubstrateLawfulness::checkMonotonicSeqIndex`).
#[inline]
pub fn check_monotonic_seq_index(prev_idx: u64, next_idx: u64) -> bool {
    prev_idx < next_idx
}

/// Unified stack certification function directly corresponding to LexLean `Main::certifyUnifiedStack`.
#[inline]
pub fn certify_unified_stack(
    comm: u64,
    eps: u64,
    gamma: u64,
    rho: u64,
    r_min: u64,
    r_max: u64,
    base_radius: u64,
    pert: u64,
    rec_margin: u64,
    c_all: bool,
    residual: u64,
    res_eps: u64,
    slope: u64,
    prev_hash: u64,
    state_hash: u64,
    seq_idx: u64,
) -> (SubstrateState, (RecurrenceState, (LawfulnessStatus, u64))) {
    let sub = evaluate_open_core_step(comm, eps, gamma, rho, r_min, r_max);
    let rec = evaluate_recurrence(base_radius, pert, rec_margin);
    let law = evaluate_substrate(c_all, residual, res_eps, slope);
    let hash = pweh_hash_step(prev_hash, state_hash, seq_idx);
    (sub, (rec, (law, hash)))
}

/// Execution transition parameters submitted to the engine dispatch loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionPayload {
    pub seq_idx: u64,
    pub state_hash: u64,
    pub comm: u64,
    pub eps: u64,
    pub gamma: u64,
    pub rho: u64,
    pub r_min: u64,
    pub r_max: u64,
    pub base_radius: u64,
    pub pert: u64,
    pub rec_margin: u64,
    pub c_all: bool,
    pub residual: u64,
    pub res_eps: u64,
    pub slope: u64,
    pub data: Vec<u8>,
}

impl Default for TransitionPayload {
    fn default() -> Self {
        Self {
            seq_idx: 1,
            state_hash: 1,
            comm: 5,
            eps: 10,
            gamma: 800,
            rho: 50,
            r_min: 10,
            r_max: 100,
            base_radius: 800,
            pert: 50,
            rec_margin: 50,
            c_all: true,
            residual: 5,
            res_eps: 10,
            slope: 800,
            data: Vec::new(),
        }
    }
}

/// Errors returned by the unified dispatch loop when a pre-commit gate aborts.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DispatchError {
    #[error("Non-monotonic sequence index: previous {prev}, requested {requested}")]
    NonMonotonicSequenceIndex { prev: u64, requested: u64 },

    #[error("OpenCore substrate halted with code {0}")]
    SubstrateHalted(u64),

    #[error("Recurrence contractivity breach (SG2 Weyl gate) with code {0}")]
    RecurrenceBreach(u64),

    #[error("Static criteria conjunction failed (SL1)")]
    StaticCriteriaFailure,

    #[error("Residual energy breach (SL4 / L0_HALT): residual {residual} > epsilon {res_eps}")]
    ResidualEnergyBreach { residual: u64, res_eps: u64 },

    #[error("Dynamic slope gap breach (SL3): slope {slope} >= 1000")]
    DynamicSlopeGapBreach { slope: u64 },
}

/// Receipt confirming successful validation and state commitment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchReceipt {
    pub transition_id: usize,
    pub seq_idx: u64,
    pub pweh_chain_hash: u64,
    pub cryptographic_sha256: String,
    pub substrate_state: SubstrateState,
    pub recurrence_state: RecurrenceState,
    pub lawfulness_status: LawfulnessStatus,
}

/// The unified dispatch loop maintaining running PWEH hash and enforcing pre-commit gates.
#[derive(Debug, Clone)]
pub struct UnifiedDispatchLoop {
    pub current_seq_idx: u64,
    pub current_pweh_hash: u64,
    pub committed_transitions: usize,
    pub last_receipt: Option<DispatchReceipt>,
}

impl UnifiedDispatchLoop {
    /// Initialize a new dispatch loop starting from sequence index 0 and initial seed hash.
    pub fn new(initial_hash: u64) -> Self {
        Self {
            current_seq_idx: 0,
            current_pweh_hash: initial_hash,
            committed_transitions: 0,
            last_receipt: None,
        }
    }

    /// Dispatch a transition. Must invoke `certifyUnifiedStack` and sequence monotonicity checks
    /// before committing any state mutation.
    pub fn dispatch(&mut self, payload: &TransitionPayload) -> Result<DispatchReceipt, DispatchError> {
        // 1. Strict sequence index monotonicity check (SL5)
        if !check_monotonic_seq_index(self.current_seq_idx, payload.seq_idx) {
            return Err(DispatchError::NonMonotonicSequenceIndex {
                prev: self.current_seq_idx,
                requested: payload.seq_idx,
            });
        }

        // 2. Invoke LexLean-specified certifyUnifiedStack
        let (sub_state, (rec_state, (law_status, next_hash))) = certify_unified_stack(
            payload.comm,
            payload.eps,
            payload.gamma,
            payload.rho,
            payload.r_min,
            payload.r_max,
            payload.base_radius,
            payload.pert,
            payload.rec_margin,
            payload.c_all,
            payload.residual,
            payload.res_eps,
            payload.slope,
            self.current_pweh_hash,
            payload.state_hash,
            payload.seq_idx,
        );

        // 3. Evaluate OpenCore substrate state gate
        if let SubstrateState::Halted(code) = sub_state {
            return Err(DispatchError::SubstrateHalted(code));
        }

        // 4. Evaluate SG2 recurrence contractivity gate
        if let RecurrenceState::NonContractive(code) = rec_state {
            return Err(DispatchError::RecurrenceBreach(code));
        }

        // 5. Evaluate SL1-SL4 substrate lawfulness gate
        match law_status {
            LawfulnessStatus::Lawful => {}
            LawfulnessStatus::L0Halt(1) => return Err(DispatchError::StaticCriteriaFailure),
            LawfulnessStatus::L0Halt(2) => {
                return Err(DispatchError::ResidualEnergyBreach {
                    residual: payload.residual,
                    res_eps: payload.res_eps,
                })
            }
            LawfulnessStatus::L0Halt(3) => {
                return Err(DispatchError::DynamicSlopeGapBreach {
                    slope: payload.slope,
                })
            }
            LawfulnessStatus::L0Halt(other) => return Err(DispatchError::SubstrateHalted(other)),
        }

        // 6. Compute cryptographic SHA-256 witness anchor
        let mut hasher = Sha256::new();
        hasher.update(self.current_seq_idx.to_le_bytes());
        hasher.update(payload.seq_idx.to_le_bytes());
        hasher.update(self.current_pweh_hash.to_le_bytes());
        hasher.update(next_hash.to_le_bytes());
        hasher.update(&payload.data);
        let crypto_hash = format!("{:x}", hasher.finalize());

        // 7. Commit state atomically
        self.committed_transitions += 1;
        self.current_seq_idx = payload.seq_idx;
        self.current_pweh_hash = next_hash;

        let receipt = DispatchReceipt {
            transition_id: self.committed_transitions,
            seq_idx: self.current_seq_idx,
            pweh_chain_hash: self.current_pweh_hash,
            cryptographic_sha256: crypto_hash,
            substrate_state: sub_state,
            recurrence_state: rec_state,
            lawfulness_status: law_status,
        };

        self.last_receipt = Some(receipt.clone());
        Ok(receipt)
    }
}
