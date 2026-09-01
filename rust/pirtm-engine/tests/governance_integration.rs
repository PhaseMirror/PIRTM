use pirtm_engine::governance::Sentinel;
use pirtm_engine::spectral::Ensemble;
use pirtm_monitor::{ManifoldState, MockStateProvider, MonitorConfig};

#[test]
fn test_sentinel_validate_and_seal_stable_state() {
    let stable_ensemble = Ensemble {
        name: "test_stable".to_string(),
        adjacency: vec![vec![0.0, 0.5], vec![0.5, 0.0]],
        lambdas: vec![0.9, 0.9],
    };

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
