<<<<<<< HEAD
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
=======
use pirtm_engine::spectral::{self, Ensemble, EnsembleError};

#[test]
fn test_small_gain_pipeline_accepted() {
    // 3-stage linear feedforward pipeline
    let ensemble = Ensemble::new(
        "pipeline_3",
        vec![
            vec![0.0, 0.4, 0.0],
            vec![0.0, 0.0, 0.4],
            vec![0.0, 0.0, 0.0],
        ],
        vec![(1, 2), (1, 2), (1, 2)],
    ).with_theorem_name("Foundations.ADR.BoundedIteration.iterate_non_expansive");

    let norm_1 = ensemble.compute_rational_norm_1().expect("Pipeline should pass");
    assert!(norm_1 < num_rational::Ratio::new(1, 1));
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
}

#[test]
fn test_small_gain_contractive_feedback_accepted() {
<<<<<<< HEAD
    // Retuned pass: A = [[0, 2/5], [2/5, 0]], λ = (9/10, 9/10), ||G||_1 = 9/25
    let ensemble = Ensemble::from_rationals(
        "contractive_loop",
        vec![vec![q(0, 1), q(2, 5)], vec![q(2, 5), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    );

    let n1 = spectral::check_small_gain(&ensemble, 0.0).expect("Contractive loop should pass");
    assert!((n1 - 0.36).abs() < 1e-12);
=======
    // Cyclic 2-node feedback loop with contractive loop gain:
    // A_01 = 0.4, A_10 = 0.4, λ_0 = 0.8 (4/5), λ_1 = 0.8 (4/5)
    // ||G||_1 = max(0.4 * 0.8, 0.4 * 0.8) = 0.32 < 1.0
    let ensemble = Ensemble::new(
        "contractive_loop",
        vec![
            vec![0.0, 0.4],
            vec![0.4, 0.0],
        ],
        vec![(4, 5), (4, 5)],
    ).with_theorem_name("author_declared_lambda");

    let cert = spectral::validate_and_certify(&ensemble, 0.0).expect("Contractive feedback loop should pass");
    assert!(cert.is_stable);
    assert_eq!(cert.exact_rational_norm_1, (8, 25));
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
}

#[test]
fn test_small_gain_resonant_feedback_rejected() {
<<<<<<< HEAD
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
=======
    // Resonant 2-node feedback loop:
    // A_01 = 2.0, A_10 = 0.5, λ_0 = 0.95 (19/20), λ_1 = 0.95 (19/20)
    // ||G||_1 = max(0.5 * 0.95, 2.0 * 0.95) = max(0.475, 1.9) = 1.9 >= 1.0
    let ensemble = Ensemble::new(
        "resonant_loop",
        vec![
            vec![0.0, 2.0],
            vec![0.5, 0.0],
        ],
        vec![(19, 20), (19, 20)],
    ).with_theorem_name("author_declared_lambda");

    let res = spectral::validate_and_certify(&ensemble, 0.0);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err(),
        EnsembleError::NormContractivityViolation(19, 10)
    );
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
}

#[test]
fn test_runtime_validate_and_certify() {
    let ensemble = Ensemble::from_rationals(
        "certified_ensemble",
<<<<<<< HEAD
        vec![vec![q(0, 1), q(2, 5)], vec![q(2, 5), q(0, 1)]],
        vec![q(9, 10), q(9, 10)],
    )
    .with_theorem_name("author_declared_lambda");

    let receipt = spectral::validate_and_certify(&ensemble, 0.0).expect("Validation must succeed");
    assert!(receipt.is_norm_contractive);
    assert_eq!(receipt.exact_rational_norm_1, (9, 25));
=======
        vec![
            vec![0.0, 0.3],
            vec![0.3, 0.0],
        ],
        vec![(1, 2), (1, 2)],
    ).with_theorem_name("author_declared_lambda");

    let receipt = spectral::validate_and_certify(&ensemble, 0.0).expect("Validation must succeed");
    assert!(receipt.is_stable);
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
    assert_eq!(receipt.dimension, 2);
    assert_eq!(receipt.theorem_name, "author_declared_lambda");
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
