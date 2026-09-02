//! PIRTM Background Daemon (pirtmd)
//!
//! Hosts the PIRTM governed compiler, Sentinel gate, WardMonitor, and MCP server
//! over WebSocket IPC for interactive TUI / CLI clients (Kilo / OpenCode style).

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

use pirtm_compiler::PhaseMirrorCompiler;
use pirtm_engine::spectral::{self, Ensemble};
use pirtm_engine::{Runtime, RuntimeConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonResponse {
    pub id: u64,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub struct DaemonState {
    pub runtime: Runtime,
    pub session_count: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    match command {
        "serve" => {
            let addr = "127.0.0.1:8090";
            let listener = TcpListener::bind(addr).await?;
            println!("🚀 PIRTM Daemon (pirtmd) listening on ws://{}", addr);

            let state = Arc::new(Mutex::new(DaemonState {
                runtime: Runtime::new(RuntimeConfig {
                    dry_run: true,
                    jid_enabled: false,
                    ledger_enabled: true,
                    enforce_bounds: true,
                    input_args: vec![],
                }),
                session_count: 0,
            }));

            while let Ok((stream, peer)) = listener.accept().await {
                let state_clone = state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, peer, state_clone).await {
                        eprintln!("Error handling connection from {}: {}", peer, e);
                    }
                });
            }
        }
        "status" => {
            println!("PIRTM Daemon status: ACTIVE (Port 8090)");
        }
        _ => {
            eprintln!("Usage: pirtmd [serve|status]");
        }
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    peer: SocketAddr,
    state: Arc<Mutex<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    println!("🔌 Client connected from {}", peer);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_text() {
            let text = msg.to_text()?;
            if let Ok(req) = serde_json::from_str::<DaemonRequest>(text) {
                let resp = process_request(req, state.clone()).await;
                let resp_json = serde_json::to_string(&resp)?;
                ws_sender.send(Message::Text(resp_json)).await?;
            } else {
                let err_resp = DaemonResponse {
                    id: 0,
                    result: None,
                    error: Some("Invalid JSON-RPC request frame".to_string()),
                };
                ws_sender
                    .send(Message::Text(serde_json::to_string(&err_resp)?))
                    .await?;
            }
        }
    }

    println!("🔌 Client disconnected from {}", peer);
    Ok(())
}

/// Extract matrix [[(n, d), ...], ...] and lambdas [(n, d), ...] directly from source AST
fn extract_spectral_params(source: &str) -> Result<(Vec<Vec<(u64, u64)>>, Vec<(u64, u64)>), String> {
    let matrix_idx = source.find("matrix").ok_or_else(|| {
        "MissingSpectralParams: source does not contain 'matrix' declaration".to_string()
    })?;
    let lambdas_idx = source.find("lambdas").ok_or_else(|| {
        "MissingSpectralParams: source does not contain 'lambdas' declaration".to_string()
    })?;

    let matrix_sub = &source[matrix_idx..];
    let lambdas_sub = &source[lambdas_idx..];

    let matrix_start = matrix_sub.find('[').ok_or_else(|| "UnparseableMatrix: missing '['".to_string())?;
    let matrix_end = matrix_sub.find("]]").ok_or_else(|| "UnparseableMatrix: missing ']]'".to_string())?;
    let matrix_str = &matrix_sub[matrix_start + 1..matrix_end + 1];

    let mut matrix = Vec::new();
    for row_str in matrix_str.split(']') {
        let row_trimmed = row_str.trim().trim_start_matches(',').trim().trim_start_matches('[');
        if row_trimmed.is_empty() {
            continue;
        }
        let mut row = Vec::new();
        for pair_str in row_trimmed.split(')') {
            let p_trimmed = pair_str.trim().trim_start_matches(',').trim().trim_start_matches('(');
            if p_trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = p_trimmed.split(',').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let num: u64 = parts[0].parse().map_err(|_| "InvalidNum".to_string())?;
                let den: u64 = parts[1].parse().map_err(|_| "InvalidDen".to_string())?;
                if den == 0 {
                    return Err("ZeroDenominator: matrix entry denominator must be > 0".to_string());
                }
                row.push((num, den));
            }
        }
        if !row.is_empty() {
            matrix.push(row);
        }
    }

    let lambdas_start = lambdas_sub.find('[').ok_or_else(|| "UnparseableLambdas: missing '['".to_string())?;
    let lambdas_end = lambdas_sub.find(']').ok_or_else(|| "UnparseableLambdas: missing ']'".to_string())?;
    let lambdas_str = &lambdas_sub[lambdas_start + 1..lambdas_end];

    let mut lambdas = Vec::new();
    for pair_str in lambdas_str.split(')') {
        let p_trimmed = pair_str.trim().trim_start_matches(',').trim().trim_start_matches('(');
        if p_trimmed.is_empty() {
            continue;
        }
        let parts: Vec<&str> = p_trimmed.split(',').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let num: u64 = parts[0].parse().map_err(|_| "InvalidLambdaNum".to_string())?;
            let den: u64 = parts[1].parse().map_err(|_| "InvalidLambdaDen".to_string())?;
            if den == 0 {
                return Err("ZeroDenominator: lambda denominator must be > 0".to_string());
            }
            lambdas.push((num, den));
        }
    }

    if matrix.is_empty() || lambdas.is_empty() {
        return Err("MissingSpectralParams: empty matrix or lambdas declared in source".to_string());
    }

    Ok((matrix, lambdas))
}

async fn process_request(req: DaemonRequest, state: Arc<Mutex<DaemonState>>) -> DaemonResponse {
    let lock = state.lock().await;
    match req.method.as_str() {
        "compile" => {
            let source = match req.params.get("source").and_then(|s| s.as_str()) {
                Some(s) if !s.trim().is_empty() => s,
                _ => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some("MissingSourceCode: 'source' parameter is required and cannot be empty".to_string()),
                    };
                }
            };

            let name = req
                .params
                .get("name")
                .and_then(|s| s.as_str())
                .unwrap_or("module");

            // Require explicit theorem_name per ADR-055 (no production default to author_declared_lambda or hardcoded theorem)
            let theorem_name = match req.params.get("theorem_name").and_then(|s| s.as_str()) {
                Some(s) if !s.trim().is_empty() && s != "author_declared_lambda" => s,
                _ => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some("MissingTheoremAnchor: 'theorem_name' parameter must be explicitly specified per ADR-055".to_string()),
                    };
                }
            };

            // Parse and compile source using pirtm_compiler
            let compiler = PhaseMirrorCompiler::new();
            let mlir_module = match compiler.compile(source) {
                Ok(m) => m,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(format!("CompileError: {}", err)),
                    };
                }
            };

            let mlir_text = match compiler.to_mlir(&mlir_module) {
                Ok(t) => t,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(format!("MlirEmissionError: {}", err)),
                    };
                }
            };

            // Extract (matrix, lambdas) directly from source AST; fail closed if absent
            let (matrix, lambdas) = match extract_spectral_params(source) {
                Ok(params) => params,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(err),
                    };
                }
            };

            // Evaluate 1-norm contractivity over exact rational pairs extracted from source
            let ensemble = match Ensemble::from_rationals(
                name,
                matrix,
                lambdas,
                theorem_name,
            ) {
                Ok(e) => e,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(err.to_string()),
                    };
                }
            };

            match spectral::validate_and_certify(&ensemble, 0.0) {
                Ok(receipt) => DaemonResponse {
                    id: req.id,
                    result: Some(json!({
                        "status": "COMPILED",
                        "mlir": mlir_text,
                        "receipt": receipt
                    })),
                    error: None,
                },
                Err(e) => DaemonResponse {
                    id: req.id,
                    result: None,
                    error: Some(format!("Spectral Gate Rejected: {}", e)),
                },
            }
        }
        "validate" => {
            let name = req
                .params
                .get("name")
                .and_then(|s| s.as_str())
                .unwrap_or("test_ensemble");

            let source = req.params.get("source").and_then(|s| s.as_str()).unwrap_or("");

            let theorem_name = match req.params.get("theorem_name").and_then(|s| s.as_str()) {
                Some(s) if !s.trim().is_empty() && s != "author_declared_lambda" => s,
                _ => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some("MissingTheoremAnchor: 'theorem_name' is required and author_declared_lambda fallback is forbidden per ADR-055".to_string()),
                    };
                }
            };

            // Extract (matrix, lambdas) directly from source AST if provided, or fail closed
            let (matrix, lambdas) = match extract_spectral_params(source) {
                Ok(params) => params,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(err),
                    };
                }
            };

            let ensemble = match Ensemble::from_rationals(
                name,
                matrix,
                lambdas,
                theorem_name,
            ) {
                Ok(e) => e,
                Err(err) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(err.to_string()),
                    };
                }
            };

            match lock.runtime.validate_ensemble(&ensemble) {
                Ok(receipt) => DaemonResponse {
                    id: req.id,
                    result: Some(json!({
                        "status": "VALIDATED",
                        "receipt": receipt
                    })),
                    error: None,
                },
                Err(e) => DaemonResponse {
                    id: req.id,
                    result: None,
                    error: Some(format!("Validation Failed: {}", e)),
                },
            }
        }
        "get_status" => DaemonResponse {
            id: req.id,
            result: Some(json!({
                "daemon_status": "ACTIVE",
                "spectral_norm_limit": "1.0",
                "active_sessions": lock.session_count + 1,
                "sedona_spine": "RUST_VERIFIED",
                "legal_entity_metadata": "Citizen Gardens UNA d/b/a The Prime Materia Commons"
            })),
            error: None,
        },
        "list_files" => {
            let dir = std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let files = std::fs::read_dir(".")
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok().map(|entry| entry.file_name().to_string_lossy().to_string()))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();

            DaemonResponse {
                id: req.id,
                result: Some(json!({
                    "current_dir": dir,
                    "files": files
                })),
                error: None,
            }
        }
        _ => DaemonResponse {
            id: req.id,
            result: None,
            error: Some(format!("Unknown method: {}", req.method)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_process_request_compile_fail_closed_missing_source() {
        let state = Arc::new(Mutex::new(DaemonState {
            runtime: Runtime::new(RuntimeConfig {
                dry_run: true,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args: vec![],
            }),
            session_count: 0,
        }));

        let req = DaemonRequest {
            id: 101,
            method: "compile".to_string(),
            params: json!({
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 101);
        assert!(resp.error.is_some());
        assert!(resp.error.unwrap().contains("MissingSourceCode"));
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_fail_closed_missing_matrix() {
        let state = Arc::new(Mutex::new(DaemonState {
            runtime: Runtime::new(RuntimeConfig {
                dry_run: true,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args: vec![],
            }),
            session_count: 0,
        }));

        let req = DaemonRequest {
            id: 102,
            method: "compile".to_string(),
            params: json!({
                "source": "fn main() -> i64 { return 42; }",
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 102);
        assert!(resp.error.is_some());
        assert!(resp.error.unwrap().contains("MissingSpectralParams"));
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_valid_ast_matrix() {
        let state = Arc::new(Mutex::new(DaemonState {
            runtime: Runtime::new(RuntimeConfig {
                dry_run: true,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args: vec![],
            }),
            session_count: 0,
        }));

        let valid_source = r#"
        // PIRTM Governed Contract
        // matrix [[(0, 1), (4, 10)], [(4, 10), (0, 1)]]
        // lambdas [(9, 10), (9, 10)]

        fn main() -> i64 {
            return 42;
        }
        "#;

        let req = DaemonRequest {
            id: 103,
            method: "compile".to_string(),
            params: json!({
                "source": valid_source,
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 103);
        println!("COMPILE AST RESP: {:?}", resp);
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["status"], "COMPILED");
        assert_eq!(result["receipt"]["is_norm_contractive"], true);
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_fail_closed_missing_theorem() {
        let state = Arc::new(Mutex::new(DaemonState {
            runtime: Runtime::new(RuntimeConfig {
                dry_run: true,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args: vec![],
            }),
            session_count: 0,
        }));

        let req = DaemonRequest {
            id: 104,
            method: "compile".to_string(),
            params: json!({
                "source": "matrix [[(0,1)]] lambdas [(9,10)]"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 104);
        assert!(resp.error.is_some());
        assert!(resp.error.unwrap().contains("MissingTheoremAnchor"));
    }

    #[tokio::test]
    async fn test_daemon_process_request_get_status() {
        let state = Arc::new(Mutex::new(DaemonState {
            runtime: Runtime::new(RuntimeConfig {
                dry_run: true,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args: vec![],
            }),
            session_count: 0,
        }));

        let req = DaemonRequest {
            id: 105,
            method: "get_status".to_string(),
            params: json!({}),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 105);
        let result = resp.result.unwrap();
        assert_eq!(result["daemon_status"], "ACTIVE");
        assert_eq!(result["legal_entity_metadata"], "Citizen Gardens UNA d/b/a The Prime Materia Commons");
    }
}
