use pirtm_engine::governance::Sentinel;
use pirtm_engine::spectral::Ensemble;
use pirtm_monitor::{ManifoldState, MockStateProvider, MonitorConfig};

#[test]
fn test_sentinel_validate_and_seal_stable_state() {
    let stable_ensemble = Ensemble::from_rationals(
        "test_stable",
        vec![vec![(0, 1), (1, 2)], vec![(1, 2), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "author_declared_lambda",
    )
    .unwrap();
    let provider = MockStateProvider::new(vec![ManifoldState {
        rho: 0.42,
        delta: 1e-5,
        lambda_l_product: 0.5,
        timestamp: 1000,
    }]);
    let mut sentinel = Sentinel::new(provider, MonitorConfig::default());
    let receipt = sentinel.validate_and_seal(&stable_ensemble);
    assert!(receipt.is_ok());
    assert!(!receipt.unwrap().is_empty());
}

#[test]
fn test_sentinel_rejects_missing_theorem_name() {
    let err = Ensemble::from_rationals(
        "test_unanchored",
        vec![vec![(0, 1), (1, 2)], vec![(1, 2), (0, 1)]],
        vec![(9, 10), (9, 10)],
        "",
    )
    .expect_err("empty theorem_name must not construct");
    assert_eq!(err, pirtm_engine::EnsembleError::MissingTheoremAnchor);
}
