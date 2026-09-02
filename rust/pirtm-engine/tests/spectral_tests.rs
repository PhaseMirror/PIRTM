use pirtm_engine::spectral::{self, Ensemble, PosRat};

fn q(n: u64, d: u64) -> PosRat {
    PosRat::new(n, d).unwrap()
}

#[test]
fn test_small_gain_pipeline_accepted() {
    // Feedforward: ||G||_1 = 4/5 < 1
    let ensemble = Ensemble::from_rationals(
        "pipeline_3",
        vec![
            vec![q(0, 1), q(1, 1), q(0, 1)],
            vec![q(0, 1), q(0, 1), q(1, 1)],
            vec![q(0, 1), q(0, 1), q(0, 1)],
        ],
        vec![q(4, 5), q(4, 5), q(4, 5)],
    );

    let n1 = spectral::check_small_gain(&ensemble, 0.0).expect("Pipeline should pass");
    assert!((n1 - 0.8).abs() < 1e-12);
}

#[test]
fn test_small_gain_contractive_feedback_accepted() {
    // Retuned pass: A = [[0, 2/5], [2/5, 0]], λ = (9/10, 9/10), ||G||_1 = 9/25
    let ensemble = Ensemble::from_rationals(
        "contractive_loop",
        vec![vec![q(0, 1), q(2, 5)], vec![q(2, 5), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    );

    let n1 = spectral::check_small_gain(&ensemble, 0.0).expect("Contractive loop should pass");
    assert!((n1 - 0.36).abs() < 1e-12);
}

#[test]
fn test_small_gain_resonant_feedback_rejected() {
    // Retired ρ=0.9 fixture: ||G||_1 = 9/5 >= 1
    let ensemble = Ensemble::from_rationals(
        "resonant_loop",
        vec![vec![q(0, 1), q(2, 1)], vec![q(1, 2), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    );

    let res = spectral::check_small_gain(&ensemble, 0.0);
    assert!(res.is_err(), "||G||_1 >= 1 must be rejected");
    let err = res.unwrap_err();
    assert!(err.contains("NormContractivityViolation"));
    assert!(!err.contains("SIG_GOV_KILL"));
}

#[test]
fn test_runtime_validate_and_certify() {
    let ensemble = Ensemble::from_rationals(
        "certified_ensemble",
        vec![vec![q(0, 1), q(2, 5)], vec![q(2, 5), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    )
    .with_theorem_name("author_declared_lambda");

    let receipt = spectral::validate_and_certify(&ensemble, 0.0).expect("Validation must succeed");
    assert!(receipt.is_norm_contractive);
    assert_eq!(receipt.exact_rational_norm_1, (9, 25));
    assert_eq!(receipt.dimension, 2);
    assert!(!receipt.hash.is_empty());
    assert_eq!(receipt.theorem_name, "author_declared_lambda");
    assert!(receipt.validate().is_ok());
}

#[test]
fn test_runtime_validate_rejects_missing_theorem_name() {
    let ensemble = Ensemble::from_rationals(
        "uncertified_ensemble",
        vec![vec![q(0, 1), q(2, 5)], vec![q(2, 5), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    );

    let err = spectral::validate_and_certify(&ensemble, 0.0).expect_err("empty theorem_name must fail");
    assert!(err.contains("MissingTheoremAnchor"));
}
