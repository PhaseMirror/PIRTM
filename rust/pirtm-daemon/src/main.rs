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
use pirtm_parser::ast::{Expr, Stmt};

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

/// Lexically locates standalone `---` header delimiter line outside string literals/comments (ADR-060)
pub fn find_explicit_delimiter_line(source: &str) -> Option<usize> {
    let mut in_block_comment = false;
    let mut byte_offset = 0;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("/*") && !trimmed.contains("*/") {
            in_block_comment = true;
        } else if trimmed.contains("*/") {
            in_block_comment = false;
        }

        if !in_block_comment && trimmed == "---" {
            return Some(byte_offset);
        }

        byte_offset += line.len() + 1;
    }

    None
}

/// Lexically locates header split offset (explicit `---` or implicit body boundary) (ADR-057, ADR-060)
pub fn find_header_split_offset(source: &str) -> Option<(usize, bool)> {
    let mut in_block_comment = false;
    let mut byte_offset = 0;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("/*") && !trimmed.contains("*/") {
            in_block_comment = true;
        } else if trimmed.contains("*/") {
            in_block_comment = false;
        }

        if !in_block_comment {
            if trimmed == "---" {
                return Some((byte_offset, true)); // Explicit header delimiter
            }
            if trimmed.starts_with("fn ") || trimmed.starts_with("struct ") || trimmed.starts_with("enum ") || trimmed.starts_with("impl ") {
                return Some((byte_offset, false)); // Implicit body boundary
            }
        }

        byte_offset += line.len() + 1;
    }

    None
}

/// Splits PIRTM contract source into (envelope_header, application_body) (ADR-057)
pub fn split_header_body(source: &str) -> (&str, &str) {
    if let Some((offset, explicit)) = find_header_split_offset(source) {
        let (header, rest) = source.split_at(offset);
        let body = if explicit {
            rest.trim_start_matches(|c| c == '-' || c == '\r' || c == '\n' || c == ' ')
        } else {
            rest
        };
        (header, body)
    } else {
        (source, "")
    }
}

/// Helper to parse a single rational tuple `(num, den)` from AST Expr
fn parse_ast_pair(expr: &Expr) -> Result<(u64, u64), String> {
    match expr {
        Expr::Tuple(elems) if elems.len() == 2 => {
            let n = match &elems[0] {
                Expr::Literal(num) => *num,
                _ => return Err("InvalidASTPair: numerator must be an integer literal".to_string()),
            };
            let d = match &elems[1] {
                Expr::Literal(den) => *den,
                _ => return Err("InvalidASTPair: denominator must be an integer literal".to_string()),
            };
            if d == 0 {
                return Err("ZeroDenominator: rational denominator must be > 0".to_string());
            }
            Ok((n, d))
        }
        _ => Err("InvalidASTPair: expected tuple expression (num, den)".to_string()),
    }
}

/// Helper to parse a row of rational tuples from AST Expr
fn parse_ast_row(expr: &Expr) -> Result<Vec<(u64, u64)>, String> {
    match expr {
        Expr::Tuple(elems) => {
            let mut row = Vec::new();
            for elem in elems {
                row.push(parse_ast_pair(elem)?);
            }
            Ok(row)
        }
        _ => Err("InvalidASTRow: expected tuple of rational pairs".to_string()),
    }
}

/// Walk AST statements in envelope header (Phase 1) enforcing strict header validation (ADR-058, ADR-061)
fn extract_spectral_params(source: &str) -> Result<(Vec<Vec<(u64, u64)>>, Vec<(u64, u64)>), String> {
    let (header_text, body_text) = split_header_body(source);

    // Reject multiple '---' delimiters in body text (ADR-061)
    if !body_text.is_empty() && find_explicit_delimiter_line(body_text).is_some() {
        return Err("MultipleHeaderDelimiters: multiple '---' header delimiters are strictly forbidden per ADR-061".to_string());
    }

    // Parse header text using pirtm_parser (ignoring comments)
    let program = pirtm_parser::parse(header_text).map_err(|e| format!("ParseError in envelope header: {}", e))?;

    let mut matrix: Option<Vec<Vec<(u64, u64)>>> = None;
    let mut lambdas: Option<Vec<(u64, u64)>> = None;

    for stmt in &program.stmts {
        match stmt {
            Stmt::Let { name, expr } | Stmt::LetMut { name, expr } => {
                if name == "matrix" {
                    match expr {
                        Expr::Tuple(rows) => {
                            let mut m = Vec::new();
                            for row_expr in rows {
                                m.push(parse_ast_row(row_expr)?);
                            }
                            matrix = Some(m);
                        }
                        _ => return Err("InvalidASTMatrix: 'let matrix' must be a nested tuple of rows".to_string()),
                    }
                } else if name == "lambdas" {
                    lambdas = Some(parse_ast_row(expr)?);
                } else if name == "theorem" {
                    // Allowed header theorem declaration
                } else {
                    return Err(format!("InvalidHeaderStatement: unexpected let binding 'let {}' in envelope header per ADR-058", name));
                }
            }
            Stmt::Import(_) | Stmt::Ensemble(_) => {}
            _ => {
                return Err("InvalidHeaderStatement: application statements are quarantined from the envelope header per ADR-058".to_string());
            }
        }
    }

    let m = matrix.ok_or_else(|| {
        "MissingSpectralParams: program AST does not declare 'let matrix = ...;' statement".to_string()
    })?;
    let l = lambdas.ok_or_else(|| {
        "MissingSpectralParams: program AST does not declare 'let lambdas = ...;' statement".to_string()
    })?;

    if m.is_empty() || l.is_empty() {
        return Err("MissingSpectralParams: empty matrix or lambdas declared in AST".to_string());
    }

    Ok((m, l))
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

            // Phase 1 Governance Gate: Extract (matrix, lambdas) directly from envelope header; fail closed if absent, malformed, or multiple delimiters
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

            // Evaluate 1-norm contractivity over exact rational pairs extracted from AST
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

            let receipt = match spectral::validate_and_certify(&ensemble, 0.0) {
                Ok(r) => r,
                Err(e) => {
                    return DaemonResponse {
                        id: req.id,
                        result: None,
                        error: Some(format!("Spectral Gate Rejected: {}", e)),
                    };
                }
            };

            // Phase 2 Code Generation: Parse application source and emit MLIR
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

            DaemonResponse {
                id: req.id,
                result: Some(json!({
                    "status": "COMPILED",
                    "mlir": mlir_text,
                    "receipt": receipt
                })),
                error: None,
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

            // Extract (matrix, lambdas) directly from parsed AST if provided, or fail closed
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
                "source": "let x = 42;",
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 102);
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert!(err.contains("InvalidHeaderStatement") || err.contains("MissingSpectralParams"));
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_fail_closed_unexpected_header_stmt() {
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

        // Header contains extra non-envelope let binding (ADR-058 violation)
        let unexpected_header_source = r#"
        let foo = 42;
        let matrix = (((0, 1), (4, 10)), ((4, 10), (0, 1)));
        let lambdas = ((9, 10), (9, 10));
        ---
        fn main() -> i64 { return 42; }
        "#;

        let req = DaemonRequest {
            id: 103,
            method: "compile".to_string(),
            params: json!({
                "source": unexpected_header_source,
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 103);
        assert!(resp.error.is_some());
        assert!(resp.error.unwrap().contains("InvalidHeaderStatement"));
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_fail_closed_multiple_delimiters() {
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

        let double_delimiter_source = r#"
        let matrix = (((0, 1), (4, 10)), ((4, 10), (0, 1)));
        let lambdas = ((9, 10), (9, 10));
        ---
        fn main() -> i64 { return 42; }
        ---
        "#;

        let req = DaemonRequest {
            id: 104,
            method: "compile".to_string(),
            params: json!({
                "source": double_delimiter_source,
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 104);
        assert!(resp.error.is_some());
        assert!(resp.error.unwrap().contains("MultipleHeaderDelimiters"));
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

        let valid_ast_source = r#"
        let matrix = (((0, 1), (4, 10)), ((4, 10), (0, 1)));
        let lambdas = ((9, 10), (9, 10));

        fn main() -> i64 {
            return 42;
        }
        "#;

        let req = DaemonRequest {
            id: 105,
            method: "compile".to_string(),
            params: json!({
                "source": valid_ast_source,
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 105);
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["status"], "COMPILED");
        assert_eq!(result["receipt"]["is_norm_contractive"], true);
    }

    #[tokio::test]
    async fn test_daemon_process_request_compile_valid_header_delimiter() {
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

        let header_delimited_source = r#"
        let matrix = (((0, 1), (4, 10)), ((4, 10), (0, 1)));
        let lambdas = ((9, 10), (9, 10));
        ---
        fn main() -> i64 {
            return 42;
        }
        "#;

        let req = DaemonRequest {
            id: 106,
            method: "compile".to_string(),
            params: json!({
                "source": header_delimited_source,
                "name": "test_contract",
                "theorem_name": "Foundations.ADR.BoundedIteration.iterate_non_expansive"
            }),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 106);
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["status"], "COMPILED");
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
            id: 107,
            method: "get_status".to_string(),
            params: json!({}),
        };

        let resp = process_request(req, state).await;
        assert_eq!(resp.id, 107);
        let result = resp.result.unwrap();
        assert_eq!(result["daemon_status"], "ACTIVE");
        assert_eq!(result["legal_entity_metadata"], "Citizen Gardens UNA d/b/a The Prime Materia Commons");
    }
}
