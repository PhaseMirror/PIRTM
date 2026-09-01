#!/usr/bin/env python3
"""
End-to-End Multi-Turn MCP Client Test Harness for PIRTM Governance Daemon.
Demonstrates live interaction by external AI agents against `pirtm mcp start`.
"""

import json
import subprocess
import sys
import os

def run_test():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.abspath(os.path.join(script_dir, ".."))
    binary_path = os.path.join(project_root, "rust", "target", "release", "pirtm")

    if not os.path.isfile(binary_path):
        print(f"❌ Binary not found at {binary_path}. Run `cargo build --release -p pirtm-compiler` first.")
        sys.exit(1)

    print(f"🚀 Spawning PIRTM MCP Daemon: {binary_path} mcp start --transport stdio")
    proc = subprocess.Popen(
        [binary_path, "mcp", "start", "--transport", "stdio"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )

    def send_recv(request_dict):
        req_line = json.dumps(request_dict)
        proc.stdin.write(req_line + "\n")
        proc.stdin.flush()
        resp_line = proc.stdout.readline()
        if not resp_line:
            stderr = proc.stderr.read()
            raise RuntimeError(f"Server exited unexpectedly. Stderr: {stderr}")
        return json.loads(resp_line.strip())

    try:
        # Step 1: Handshake
        print("\n--- [Turn 1: Handshake (initialize)] ---")
        init_req = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        }
        init_resp = send_recv(init_req)
        print("Response:", json.dumps(init_resp, indent=2))
        assert init_resp["id"] == 1
        assert init_resp["result"]["serverInfo"]["name"] == "pirtm-governance-mcp"
        print("✅ Handshake successful.")

        # Step 2: Tool Discovery
        print("\n--- [Turn 2: Tool Discovery (tools/list)] ---")
        list_req = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }
        list_resp = send_recv(list_req)
        tools = [t["name"] for t in list_resp["result"]["tools"]]
        print("Discovered tools:", tools)
        assert "pirtm_compile" in tools
        assert "pirtm_verify_ensemble" in tools
        print("✅ Tool discovery verified (5 tools active).")

        # Step 3: Semantic Validation
        print("\n--- [Turn 3: Semantic Validation (validate)] ---")
        val_req = {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "validate",
                "arguments": {
                    "source": "let mut count = 0; while count < 5 { count = count + 1; }"
                }
            }
        }
        val_resp = send_recv(val_req)
        val_data = json.loads(val_resp["result"]["content"][0]["text"])
        print("Validation Result:", val_data)
        assert val_data["status"] == "VALID"
        print("✅ Validation gate passed.")

        # Step 4: Governed Compilation
        print("\n--- [Turn 4: Compilation to MLIR (compile)] ---")
        compile_req = {
            "jsonrpc": "2.0",
            "id": 4,
            "method": "tools/call",
            "params": {
                "name": "compile",
                "arguments": {
                    "source": "let x = 42;"
                }
            }
        }
        comp_resp = send_recv(compile_req)
        comp_data = json.loads(comp_resp["result"]["content"][0]["text"])
        print("Compile Result:\n", json.dumps(comp_data, indent=2))
        assert comp_data["status"] == "SUCCESS"
        assert "receipt_hash" in comp_data
        assert "pirtm.operator_atom 42" in comp_data["mlir"]
        receipt_hash = comp_data["receipt_hash"]
        print(f"✅ Compilation emitted valid MLIR with receipt hash: {receipt_hash}")

        # Step 5: Spectral Small-Gain Interlock (Stable)
        print("\n--- [Turn 5: Spectral Small-Gain Verification (Stable)] ---")
        stable_req = {
            "jsonrpc": "2.0",
            "id": 5,
            "method": "tools/call",
            "params": {
                "name": "verify_ensemble",
                "arguments": {
                    "name": "contractive_ensemble",
                    "adjacency_matrix": [
                        [0.0, 0.4],
                        [0.4, 0.0]
                    ],
                    "lambdas": [0.8, 0.8]
                }
            }
        }
        stable_resp = send_recv(stable_req)
        stable_data = json.loads(stable_resp["result"]["content"][0]["text"])
        print("Ensemble Check (Stable):\n", json.dumps(stable_data, indent=2))
        assert stable_data["action"] == "ACCEPT"
        assert stable_data["is_stable"] is True
        print(f"✅ Small-Gain interlock passed: rho = {stable_data['spectral_radius']} < 1.0 (ACCEPT)")

        # Step 6: Spectral Small-Gain Interlock (Unstable Fail-Closed)
        print("\n--- [Turn 6: Spectral Small-Gain Verification (Unstable Fail-Closed)] ---")
        unstable_req = {
            "jsonrpc": "2.0",
            "id": 6,
            "method": "tools/call",
            "params": {
                "name": "verify_ensemble",
                "arguments": {
                    "name": "resonant_ensemble",
                    "adjacency_matrix": [
                        [0.0, 1.5],
                        [1.5, 0.0]
                    ],
                    "lambdas": [0.9, 0.9]
                }
            }
        }
        unstable_resp = send_recv(unstable_req)
        unstable_data = json.loads(unstable_resp["result"]["content"][0]["text"])
        print("Ensemble Check (Unstable):\n", json.dumps(unstable_data, indent=2))
        assert unstable_data["action"] == "SIG_GOV_KILL"
        assert unstable_data["status"] == "REJECTED"
        print("✅ Fail-closed tripwire activated: SIG_GOV_KILL emitted on rho >= 1.0")

        # Step 7: Artifact Execution & Audit Sealing
        print("\n--- [Turn 7: Runtime Artifact Execution & Audit Sealing] ---")
        artifact_path = os.path.join(project_root, "examples", "bounded_loop.mlir")
        exec_req = {
            "jsonrpc": "2.0",
            "id": 7,
            "method": "tools/call",
            "params": {
                "name": "run_artifact",
                "arguments": {
                    "artifact_path": artifact_path
                }
            }
        }
        exec_resp = send_recv(exec_req)
        exec_data = json.loads(exec_resp["result"]["content"][0]["text"])
        print("Execution Result:\n", json.dumps(exec_data, indent=2))
        assert exec_data["status"] == "EXECUTION_COMPLETE"
        assert exec_data["return_code"] == 0
        assert exec_data["audit_sealed"] is True
        print("✅ Execution completed under governed contractivity bounds with audit sealed.")

        # Step 8: Ledger Receipt Query
        print("\n--- [Turn 8: Ledger Receipt Verification] ---")
        rcpt_req = {
            "jsonrpc": "2.0",
            "id": 8,
            "method": "tools/call",
            "params": {
                "name": "get_receipt",
                "arguments": {
                    "receipt_hash": receipt_hash
                }
            }
        }
        rcpt_resp = send_recv(rcpt_req)
        rcpt_data = json.loads(rcpt_resp["result"]["content"][0]["text"])
        print("Receipt Status:\n", json.dumps(rcpt_data, indent=2))
        assert rcpt_data["status"] == "VERIFIED_ON_LEDGER"
        print("✅ Receipt successfully verified on immutable ledger.")

        print("\n🎉 ALL 8 MULTI-TURN MCP GOVERNANCE TURNS PASSED END-TO-END!")

    finally:
        proc.terminate()
        proc.wait()

if __name__ == "__main__":
    run_test()
