pub mod protocol;
pub mod tools;

use protocol::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;
use std::io::{BufRead, Write};

pub struct McpServer;

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = req.id;
        match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse::success(
                id,
                json!({
                    "protocolVersion": "2024-11-05",
                    "serverInfo": {
                        "name": "pirtm-governance-mcp",
                        "version": "0.1.0"
                    },
                    "capabilities": {
                        "tools": {}
                    }
                }),
            )),
            "notifications/initialized" => None,
            "ping" => Some(JsonRpcResponse::success(id, json!({}))),
            "tools/list" => Some(JsonRpcResponse::success(id, tools::list_tools())),
            "tools/call" => {
                let params = req.params.unwrap_or(json!({}));
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                match tools::handle_call(tool_name, &args) {
                    Ok(result) => Some(JsonRpcResponse::success(id, result)),
                    Err(err) => Some(JsonRpcResponse::error(id, -32602, err)),
                }
            }
            other => Some(JsonRpcResponse::error(
                id,
                -32601,
                format!("Method not found: {}", other),
            )),
        }
    }

    pub fn run_stdio<R: BufRead, W: Write>(&self, mut reader: R, mut writer: W) -> Result<(), anyhow::Error> {
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
                    if let Some(resp) = self.handle_request(req) {
                        let resp_json = serde_json::to_string(&resp)?;
                        writeln!(writer, "{}", resp_json)?;
                        writer.flush()?;
                    }
                }
            }
            line.clear();
        }
        Ok(())
    }
}
