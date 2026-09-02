use pirtm_engine::governance::Sentinel;
use pirtm_engine::spectral::Ensemble;
use pirtm_monitor::{ManifoldState, MockStateProvider, MonitorConfig};

#[test]
fn test_sentinel_validate_and_seal_stable_state() {
    let stable_ensemble = Ensemble::new(
        "test_stable",
        vec![vec![0.0, 0.5], vec![0.5, 0.0]],
        vec![0.9, 0.9],
    )
    .with_theorem_name("author_declared_lambda");

    let provider = MockStateProvider::new(vec![ManifoldState {
        rho: 0.42,
        delta: 1e-5,
        lambda_l_product: 0.5,
        timestamp: 1000,
    }]);

    let mut sentinel = Sentinel::new(provider, MonitorConfig::default());
    let receipt = sentinel.validate_and_seal(&stable_ensemble);

    assert!(receipt.is_ok());
    let hash = receipt.unwrap();
    assert!(!hash.is_empty());
}

#[test]
fn test_sentinel_rejects_missing_theorem_name() {
    let ensemble = Ensemble::new(
        "test_unanchored",
        vec![vec![0.0, 0.5], vec![0.5, 0.0]],
        vec![0.9, 0.9],
    );

    let provider = MockStateProvider::new(vec![ManifoldState {
        rho: 0.42,
        delta: 1e-5,
        lambda_l_product: 0.5,
        timestamp: 1000,
    }]);

    let mut sentinel = Sentinel::new(provider, MonitorConfig::default());
    let err = sentinel.validate_and_seal(&ensemble).expect_err("empty theorem_name must not seal");
    assert!(err.contains("MissingTheoremAnchor"));
}
