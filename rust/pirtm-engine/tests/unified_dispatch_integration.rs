//! Integration tests for LexLean Unified Stack Certification & Pre-Commit Dispatch Loop
//!
//! Verifies fail-closed behavior across:
//! - Nominal valid state transition path (commits state, advances sequence, computes PWEH chain)
//! - Recurrence contractivity breach (SG2 Weyl gate perturbation violation aborts before commit)
//! - Residual energy breach (SL4 / L0_HALT residual energy violation aborts before commit)
//! - Non-monotonic sequence index (SL5 monotonic index violation aborts before commit)
//! - Dynamic slope gap breach (SL3 dynamic contraction slope violation aborts before commit)
//! - Static criteria conjunction failure (SL1 static criteria violation aborts before commit)
//! - OpenCore invariant failure (C1, C2, C3 violation aborts before commit)
//! - Equational alignment with LexLean `certifyUnifiedStack` formal theorems

use pirtm_engine::unified_dispatch::{
    certify_unified_stack, DispatchError, LawfulnessStatus,
    RecurrenceState, SubstrateState, TransitionPayload, UnifiedDispatchLoop,
};

#[test]
fn test_lexlean_theorem_alignment_nominal() {
    // Directly mirrors Main.lex.tex theorem `valid_unified_stack_nominal`:
    // (certifyUnifiedStack 5 10 800 50 10 100 800 50 50 true 5 10 800 1 1 2)
    // = (active, (contractive, (lawful, 12)))
    let (sub, (rec, (law, hash))) = certify_unified_stack(
        5, 10, 800, 50, 10, 100, // OpenCore
        800, 50, 50,             // RecurrenceContractivity
        true, 5, 10, 800,        // SubstrateLawfulness
        1, 1, 2,                 // PWEH step
    );

    assert_eq!(sub, SubstrateState::Active);
    assert_eq!(rec, RecurrenceState::Contractive);
    assert_eq!(law, LawfulnessStatus::Lawful);
    assert_eq!(hash, 12);
}

#[test]
fn test_lexlean_theorem_alignment_recurrence_breach() {
    // Directly mirrors Main.lex.tex theorem `invalid_unified_recurrence_breach`:
    // (first (second (certifyUnifiedStack 5 10 800 50 10 100 850 150 50 true 5 10 800 1 1 2)))
    // = nonContractive 1
    let (sub, (rec, (law, _))) = certify_unified_stack(
        5, 10, 800, 50, 10, 100,
        850, 150, 50, // 850 + 150 + 50 = 1050 > 1000
        true, 5, 10, 800,
        1, 1, 2,
    );

    assert_eq!(sub, SubstrateState::Active);
    assert_eq!(rec, RecurrenceState::NonContractive(1));
    assert_eq!(law, LawfulnessStatus::Lawful);
}

#[test]
fn test_lexlean_theorem_alignment_residual_breach() {
    // Directly mirrors Main.lex.tex theorem `invalid_unified_residual_breach`:
    // (first (second (second (certifyUnifiedStack 5 10 800 50 10 100 800 50 50 true 15 10 800 1 1 2))))
    // = l0Halt 2
    let (sub, (rec, (law, _))) = certify_unified_stack(
        5, 10, 800, 50, 10, 100,
        800, 50, 50,
        true, 15, 10, 800, // residual 15 > eps 10
        1, 1, 2,
    );

    assert_eq!(sub, SubstrateState::Active);
    assert_eq!(rec, RecurrenceState::Contractive);
    assert_eq!(law, LawfulnessStatus::L0Halt(2));
}

#[test]
fn test_dispatch_nominal_path_commits_and_advances() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);
    assert_eq!(dispatch_loop.committed_transitions, 0);
    assert_eq!(dispatch_loop.current_seq_idx, 0);
    assert_eq!(dispatch_loop.current_pweh_hash, 1);

    // Transition 1: Nominal parameters
    let payload_1 = TransitionPayload {
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
        data: b"step_1_payload".to_vec(),
    };

    let receipt_1 = dispatch_loop
        .dispatch(&payload_1)
        .expect("Nominal transition 1 must commit");

    assert_eq!(receipt_1.transition_id, 1);
    assert_eq!(receipt_1.seq_idx, 1);
    assert_eq!(receipt_1.substrate_state, SubstrateState::Active);
    assert_eq!(receipt_1.recurrence_state, RecurrenceState::Contractive);
    assert_eq!(receipt_1.lawfulness_status, LawfulnessStatus::Lawful);
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 1);

    // Transition 2: Next nominal step
    let payload_2 = TransitionPayload {
        seq_idx: 2,
        state_hash: 2,
        data: b"step_2_payload".to_vec(),
        ..payload_1
    };

    let receipt_2 = dispatch_loop
        .dispatch(&payload_2)
        .expect("Nominal transition 2 must commit");

    assert_eq!(receipt_2.transition_id, 2);
    assert_eq!(receipt_2.seq_idx, 2);
    assert_eq!(dispatch_loop.committed_transitions, 2);
    assert_eq!(dispatch_loop.current_seq_idx, 2);
    assert_ne!(receipt_1.cryptographic_sha256, receipt_2.cryptographic_sha256);
}

#[test]
fn test_dispatch_aborts_on_recurrence_breach() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    // Initial nominal transition
    let p1 = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        ..Default::default()
    };
    dispatch_loop.dispatch(&p1).unwrap();
    assert_eq!(dispatch_loop.committed_transitions, 1);

    // Transition 2 breaches recurrence contractivity (base_radius + pert + margin = 850 + 150 + 50 = 1050 > 1000)
    let p_breach = TransitionPayload {
        seq_idx: 2,
        state_hash: 2,
        base_radius: 850,
        pert: 150,
        rec_margin: 50,
        ..Default::default()
    };

    let err = dispatch_loop
        .dispatch(&p_breach)
        .expect_err("Recurrence breach must abort before commit");

    assert_eq!(err, DispatchError::RecurrenceBreach(1));
    // Verify fail-closed invariant: NO mutation to loop state
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 1);
}

#[test]
fn test_dispatch_aborts_on_residual_breach() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    let p1 = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        ..Default::default()
    };
    dispatch_loop.dispatch(&p1).unwrap();
    assert_eq!(dispatch_loop.committed_transitions, 1);

    // Transition 2 breaches residual energy bounds (residual 25 > res_eps 10)
    let p_residual = TransitionPayload {
        seq_idx: 2,
        state_hash: 2,
        residual: 25,
        res_eps: 10,
        ..Default::default()
    };

    let err = dispatch_loop
        .dispatch(&p_residual)
        .expect_err("Residual energy breach must abort before commit");

    assert_eq!(
        err,
        DispatchError::ResidualEnergyBreach {
            residual: 25,
            res_eps: 10
        }
    );
    // Verify fail-closed invariant: committed transitions unchanged
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 1);
}

#[test]
fn test_dispatch_aborts_on_non_monotonic_index() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    let p1 = TransitionPayload {
        seq_idx: 5,
        state_hash: 1,
        ..Default::default()
    };
    dispatch_loop.dispatch(&p1).unwrap();
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 5);

    // 1. Decreasing sequence index (5 -> 3)
    let p_regress = TransitionPayload {
        seq_idx: 3,
        state_hash: 2,
        ..Default::default()
    };
    let err_regress = dispatch_loop
        .dispatch(&p_regress)
        .expect_err("Decreasing sequence index must abort");
    assert_eq!(
        err_regress,
        DispatchError::NonMonotonicSequenceIndex {
            prev: 5,
            requested: 3
        }
    );
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 5);

    // 2. Duplicate sequence index (5 -> 5)
    let p_duplicate = TransitionPayload {
        seq_idx: 5,
        state_hash: 2,
        ..Default::default()
    };
    let err_dup = dispatch_loop
        .dispatch(&p_duplicate)
        .expect_err("Duplicate sequence index must abort");
    assert_eq!(
        err_dup,
        DispatchError::NonMonotonicSequenceIndex {
            prev: 5,
            requested: 5
        }
    );
    assert_eq!(dispatch_loop.committed_transitions, 1);
    assert_eq!(dispatch_loop.current_seq_idx, 5);
}

#[test]
fn test_dispatch_aborts_on_dynamic_slope_gap_breach() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    // slope 1050 >= 1000
    let p_slope = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        slope: 1050,
        ..Default::default()
    };

    let err = dispatch_loop
        .dispatch(&p_slope)
        .expect_err("Dynamic slope breach must abort");
    assert_eq!(err, DispatchError::DynamicSlopeGapBreach { slope: 1050 });
    assert_eq!(dispatch_loop.committed_transitions, 0);
}

#[test]
fn test_dispatch_aborts_on_static_criteria_failure() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    // c_all = false (one of the 6 criteria SL1 failed)
    let p_criteria = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        c_all: false,
        ..Default::default()
    };

    let err = dispatch_loop
        .dispatch(&p_criteria)
        .expect_err("Static criteria failure must abort");
    assert_eq!(err, DispatchError::StaticCriteriaFailure);
    assert_eq!(dispatch_loop.committed_transitions, 0);
}

#[test]
fn test_dispatch_aborts_on_opencore_invariant_failure() {
    let mut dispatch_loop = UnifiedDispatchLoop::new(1);

    // C1 violation: comm 15 > eps 10
    let p_c1 = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        comm: 15,
        eps: 10,
        ..Default::default()
    };
    let err_c1 = dispatch_loop
        .dispatch(&p_c1)
        .expect_err("C1 commutator breach must abort");
    assert_eq!(err_c1, DispatchError::SubstrateHalted(1));
    assert_eq!(dispatch_loop.committed_transitions, 0);

    // C2 violation: gamma 1200 > 1000
    let p_c2 = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        gamma: 1200,
        ..Default::default()
    };
    let err_c2 = dispatch_loop
        .dispatch(&p_c2)
        .expect_err("C2 contractivity breach must abort");
    assert_eq!(err_c2, DispatchError::SubstrateHalted(1));
    assert_eq!(dispatch_loop.committed_transitions, 0);

    // C3 violation: rho 150 > r_max 100
    let p_c3 = TransitionPayload {
        seq_idx: 1,
        state_hash: 1,
        rho: 150,
        r_max: 100,
        ..Default::default()
    };
    let err_c3 = dispatch_loop
        .dispatch(&p_c3)
        .expect_err("C3 resonance breach must abort");
    assert_eq!(err_c3, DispatchError::SubstrateHalted(1));
    assert_eq!(dispatch_loop.committed_transitions, 0);
}
