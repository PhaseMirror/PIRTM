use pirtm_engine::{spectral::Ensemble, Runtime, RuntimeConfig};
use std::path::Path;

#[test]
#[ignore = "Requires LLVM toolchain (mlir-translate, llc, clang) to be in PATH"]
fn test_end_to_end_json_parser_execution() {
    let ensemble = Ensemble::new(
        "json_parser_pipeline",
        vec![
            vec![0.0, 0.4],
            vec![0.4, 0.0],
        ],
<<<<<<< HEAD
        vec![0.8, 0.8],
    )
    .with_theorem_name("author_declared_lambda");
=======
        vec![(4, 5), (4, 5)],
    ).with_theorem_name("author_declared_lambda");
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)

    let config = RuntimeConfig {
        dry_run: false,
        jid_enabled: false,
        ledger_enabled: true,
        enforce_bounds: true,
        input_args: vec!["{\"key\": 42}".to_string()],
    };

    let mut runtime = Runtime::new(config);

<<<<<<< HEAD
    let cert = runtime.validate_ensemble(&ensemble).expect("Ensemble should pass 1-norm gate");
    assert!(cert.is_norm_contractive);
    assert!(cert.exact_rational_norm_1.0 < cert.exact_rational_norm_1.1);
=======
    // 1. Validate Small-Gain stability at link-time
    let cert = runtime.validate_ensemble(&ensemble).expect("Ensemble should pass small-gain gate");
    assert!(cert.is_stable);
    assert_eq!(cert.exact_rational_norm_1, (32, 100));
>>>>>>> 5318951 (Refactor Ensemble Initialization and Validation Logic)
    assert!(!cert.hash.is_empty());
    assert_eq!(cert.theorem_name, "author_declared_lambda");

    let mlir_path = Path::new("../../examples/json_parser.mlir");
    if mlir_path.exists() {
        runtime.load(mlir_path).expect("Failed to load json_parser.mlir");
        let receipt = runtime.run().expect("Execution failed");
        assert_eq!(receipt.return_code, 0);
        assert!(!receipt.contractivity_hash.is_empty());
    }
}
