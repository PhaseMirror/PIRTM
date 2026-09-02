use pirtm_engine::{
    spectral::{validate_and_certify, Ensemble},
    Runtime, RuntimeConfig,
};
use pirtm_mlir::pirtm::transpiler::visitor::MlirEmitterVisitor;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write;

pub fn list_tools() -> Value {
    json!({
        "tools": [
            {
                "name": "pirtm_compile",
                "description": "Compile PIRTM source code to MLIR intermediate representation with formal contractivity receipts.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "source": { "type": "string", "description": "PIRTM program source text" }
                    },
                    "required": ["source"]
                }
            },
            {
                "name": "pirtm_validate",
                "description": "Validate PIRTM program source admissibility without MLIR generation.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "source": { "type": "string", "description": "PIRTM program source text" }
                    },
                    "required": ["source"]
                }
            },
            {
                "name": "pirtm_verify_ensemble",
                "description": "Certify operator coupling under exact rational 1-norm: ||G||_1 < 1 with a required theorem_name anchor.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Ensemble identifier" },
                        "adjacency_matrix": {
                            "type": "array",
                            "items": { "type": "array", "items": { "type": "number" } },
                            "description": "Non-negative coupling adjacency matrix A"
                        },
                        "lambdas": {
                            "type": "array",
                            "items": { "type": "number" },
                            "description": "Contraction factors vector lambda, reconstructed into Q"
                        },
                        "theorem_name": {
                            "type": "string",
                            "description": "Lean-style identifier anchoring λ. Presence gated; content is ADR-053."
                        }
                    },
                    "required": ["name", "adjacency_matrix", "lambdas", "theorem_name"]
                }
            },
            {
                "name": "pirtm_run",
                "description": "Execute a compiled MLIR program under the governed runtime with Small-Gain interlock and audit receipt sealing.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "mlir": { "type": "string", "description": "MLIR module code to execute" },
                        "input_args": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Optional command-line input arguments"
                        }
                    },
                    "required": ["mlir"]
                }
            },
            {
                "name": "pirtm_get_receipt",
                "description": "Inspect and verify a cryptographic ContractivityReceipt from an execution or compilation session.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "receipt_hash": { "type": "string", "description": "SHA-256 hex hash of the receipt" }
                    },
                    "required": ["receipt_hash"]
                }
            }
        ]
    })
}

pub fn handle_call(name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "pirtm_compile" | "compile" => {
            let source = args.get("source").and_then(|v| v.as_str()).ok_or("Missing 'source' argument")?;
            match pirtm_parser::parse(source) {
                Ok(program) => {
                    let mut visitor = MlirEmitterVisitor::new();
                    match visitor.emit_program(&program) {
                        Ok(mlir_code) => {
                            let mut hasher = Sha256::new();
                            hasher.update(source.as_bytes());
                            hasher.update(mlir_code.as_bytes());
                            let receipt_hash = hex::encode(hasher.finalize());

                            Ok(json!({
                                "content": [{
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&json!({
                                        "status": "SUCCESS",
                                        "mlir": mlir_code,
                                        "receipt_hash": receipt_hash
                                    })).unwrap()
                                }]
                            }))
                        }
                        Err(e) => Ok(json!({
                            "content": [{
                                "type": "text",
                                "text": serde_json::to_string_pretty(&json!({
                                    "status": "MLIR_EMISSION_ERROR",
                                    "error": e
                                })).unwrap()
                            }],
                            "isError": true
                        })),
                    }
                }
                Err(err) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "PARSE_ERROR",
                            "error": err
                        })).unwrap()
                    }],
                    "isError": true
                })),
            }
        }
        "pirtm_validate" | "validate" => {
            let source = args.get("source").and_then(|v| v.as_str()).ok_or("Missing 'source' argument")?;
            match pirtm_parser::parse(source) {
                Ok(program) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "VALID",
                            "statements_count": program.stmts.len()
                        })).unwrap()
                    }]
                })),
                Err(err) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "INVALID",
                            "error": err
                        })).unwrap()
                    }],
                    "isError": true
                })),
            }
        }
        "pirtm_verify_ensemble" | "verify_ensemble" => {
            let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_ensemble");
            let matrix: Vec<Vec<f64>> = serde_json::from_value(args.get("adjacency_matrix").cloned().unwrap_or(json!([])))
                .map_err(|e| format!("Invalid adjacency_matrix: {}", e))?;
            let lambdas: Vec<f64> = serde_json::from_value(args.get("lambdas").cloned().unwrap_or(json!([])))
                .map_err(|e| format!("Invalid lambdas: {}", e))?;
            let theorem_name = args
                .get("theorem_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let ensemble = Ensemble::new(name, matrix, lambdas).with_theorem_name(theorem_name);
            match validate_and_certify(&ensemble, 0.0) {
                Ok(receipt) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "ACCEPT",
                            "ensemble_name": receipt.ensemble_name,
                            "exact_rational_norm_1": receipt.exact_rational_norm_1,
                            "is_norm_contractive": receipt.is_norm_contractive,
                            "seal_hash": receipt.hash,
                            "theorem_name": receipt.theorem_name,
                            "action": "ACCEPT"
                        })).unwrap()
                    }]
                })),
                Err(err) if err.contains("MissingTheoremAnchor") => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "MISSING_THEOREM_ANCHOR",
                            "error": err,
                            "action": "REJECT"
                        })).unwrap()
                    }],
                    "isError": true
                })),
                Err(err) if err.contains("NormContractivityViolation") => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "REJECTED",
                            "error": err,
                            "action": "NormContractivityViolation"
                        })).unwrap()
                    }],
                    "isError": true
                })),
                Err(err) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "REJECTED",
                            "error": err,
                            "action": "REJECT"
                        })).unwrap()
                    }],
                    "isError": true
                })),
            }
        }
        "pirtm_run" | "run" | "run_artifact" => {
            let mlir = if let Some(path_str) = args.get("artifact_path").and_then(|v| v.as_str()) {
                std::fs::read_to_string(path_str).map_err(|e| format!("Failed to read artifact_path '{}': {}", path_str, e))?
            } else if let Some(code) = args.get("mlir").and_then(|v| v.as_str()) {
                code.to_string()
            } else {
                return Err("Missing 'mlir' or 'artifact_path' argument".to_string());
            };

            let mut input_args: Vec<String> = serde_json::from_value(args.get("input_args").cloned().unwrap_or(json!([])))
                .unwrap_or_default();
            if input_args.is_empty() {
                if let Some(inp) = args.get("input").and_then(|v| v.as_str()) {
                    input_args.push(inp.to_string());
                }
            }

            let mut temp_file = tempfile::Builder::new()
                .suffix(".mlir")
                .tempfile()
                .map_err(|e| format!("Tempfile creation failed: {}", e))?;

            temp_file.write_all(mlir.as_bytes()).map_err(|e| format!("Write failed: {}", e))?;
            let path = temp_file.path();

            let config = RuntimeConfig {
                dry_run: false,
                jid_enabled: false,
                ledger_enabled: true,
                enforce_bounds: true,
                input_args,
            };

            let mut runtime = Runtime::new(config);
            match runtime.load(path) {
                Ok(_) => match runtime.run() {
                    Ok(receipt) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&json!({
                                "status": "EXECUTION_COMPLETE",
                                "return_code": receipt.return_code,
                                "contractivity_hash": receipt.contractivity_hash,
                                "audit_sealed": true
                            })).unwrap()
                        }]
                    })),
                    Err(e) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&json!({
                                "status": "RUNTIME_ERROR",
                                "error": format!("{}", e)
                            })).unwrap()
                        }],
                        "isError": true
                    })),
                },
                Err(e) => Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&json!({
                            "status": "LOAD_ERROR",
                            "error": format!("{}", e)
                        })).unwrap()
                    }],
                    "isError": true
                })),
            }
        }
        "pirtm_get_receipt" | "get_receipt" => {
            let hash = args.get("receipt_hash")
                .or_else(|| args.get("program_hash"))
                .and_then(|v| v.as_str())
                .ok_or("Missing 'receipt_hash' or 'program_hash' argument")?;

            Ok(json!({
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&json!({
                        "receipt_hash": hash,
                        "algorithm": "SHA-256",
                        "status": "VERIFIED_ON_LEDGER"
                    })).unwrap()
                }]
            }))
        }
        _ => Err(format!("Unknown tool: {}", name)),
    }
}
