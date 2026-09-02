use pirtm_engine::spectral::{self, Ensemble};

#[test]
fn test_small_gain_pipeline_accepted() {
    let ensemble = Ensemble::from_rationals(
        "pipeline_3",
        vec![
            vec![(0, 1), (1, 1), (0, 1)],
            vec![(0, 1), (0, 1), (1, 1)],
            vec![(0, 1), (0, 1), (0, 1)],
        ],
        vec![(4, 5), (4, 5), (4, 5)],
        "author_declared_lambda",
    )
    .unwrap();
    let n1 = spectral::check_small_gain(&ensemble, 0.0).expect("Pipeline should pass");
    assert!((n1 - 0.8).abs() < 1e-12);
}

#[test]
fn test_small_gain_contractive_feedback_accepted() {
    let ensemble = Ensemble::from_rationals(
        "contractive_loop",
        vec![vec![(0, 1), (2, 5)], vec![(2, 5), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "author_declared_lambda",
    )
    .unwrap();
    let n1 = spectral::check_small_gain(&ensemble, 0.0).expect("Contractive loop should pass");
    assert!((n1 - 0.36).abs() < 1e-12);
}

#[test]
fn test_small_gain_resonant_feedback_rejected() {
    let ensemble = Ensemble::from_rationals(
        "resonant_loop",
        vec![vec![(0, 1), (2, 1)], vec![(1, 2), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "author_declared_lambda",
    )
    .unwrap();
    let res = spectral::check_small_gain(&ensemble, 0.0);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("NormContractivityViolation"));
}

#[test]
fn test_runtime_validate_and_certify() {
    let ensemble = Ensemble::from_rationals(
        "certified_ensemble",
        vec![vec![(0, 1), (2, 5)], vec![(2, 5), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "author_declared_lambda",
    )
    .unwrap();
    let receipt = spectral::validate_and_certify(&ensemble, 0.0).expect("Validation must succeed");
    assert!(receipt.is_norm_contractive);
    assert_eq!(receipt.exact_rational_norm_1, (9, 25));
    assert_eq!(receipt.theorem_name, "author_declared_lambda");
}

#[test]
fn test_runtime_validate_rejects_missing_theorem_name() {
    let err = Ensemble::from_rationals(
        "uncertified_ensemble",
        vec![vec![(0, 1), (2, 5)], vec![(2, 5), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "",
    )
    .expect_err("empty theorem_name must fail");
    assert_eq!(err, pirtm_engine::EnsembleError::MissingTheoremAnchor);
}
