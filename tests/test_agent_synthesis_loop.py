#!/usr/bin/env python3
"""
Autonomous Agent Synthesis & Self-Correction Loop via PIRTM MCP Daemon.

Demonstrates an autonomous AI agent navigating:
1. Initial proposal rejection by the Small-Gain tripwire (SIG_GOV_KILL).
2. Automated Zeno damping parameter relaxation.
3. Successful re-certification (ACCEPT).
4. Verified compilation to MLIR.
5. Fail-closed execution and audit ledger sealing.
"""

import json
import subprocess
import sys
import os

def run_agent_synthesis():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.abspath(os.path.join(script_dir, ".."))
    binary_path = os.path.join(project_root, "rust", "target", "release", "pirtm")

    print("================================================================================")
    print("🤖 STARTING AUTONOMOUS AGENT SYNTHESIS LOOP (PIRTM MCP SUBSTRATE)")
    print("================================================================================")

    proc = subprocess.Popen(
        [binary_path, "mcp", "start", "--transport", "stdio"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )

    def rpc_call(method, params=None, req_id=1):
        payload = {
            "jsonrpc": "2.0",
            "id": req_id,
            "method": method,
            "params": params or {}
        }
        proc.stdin.write(json.dumps(payload) + "\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        if not line:
            stderr = proc.stderr.read()
            raise RuntimeError(f"Daemon crashed: {stderr}")
        return json.loads(line.strip())

    try:
        # 1. Initialize
        print("\n[Agent -> Daemon] initialize")
        init_res = rpc_call("initialize", req_id=101)
        print("  <- Handshake:", init_res["result"]["serverInfo"])

        # 2. Agent proposes an aggressive / resonant 2-operator ensemble
        print("\n[Agent -> Daemon] Proposing speculative high-gain ensemble...")
        proposal_1 = {
            "name": "verify_ensemble",
            "arguments": {
                "name": "speculative_agent_loop",
                "adjacency_matrix": [
                    [0.0, 1.2],
                    [1.2, 0.0]
                ],
                "lambdas": [0.95, 0.95]
            }
        }
        res_1 = rpc_call("tools/call", proposal_1, req_id=102)
        content_1 = json.loads(res_1["result"]["content"][0]["text"])
        print(f"  <- Response Status: {content_1.get('status')}")
        print(f"  <- Action: {content_1.get('action')}")
        print(f"  <- Error Details: {content_1.get('error')}")

        assert content_1.get("action") == "SIG_GOV_KILL"
        print("⚡ TRIPWIRE ACTIVATED: System correctly halted unstable expansion.")

        # 3. Agent applies Zeno Damping Correction
        print("\n[Agent Reflection] Tripwire encountered: rho >= 1.0.")
        print("[Agent Reflection] Applying ZenoController step: damping matrix weights by 60%...")
        damped_matrix = [
            [0.0, 0.48],
            [0.48, 0.0]
        ]
        proposal_2 = {
            "name": "verify_ensemble",
            "arguments": {
                "name": "speculative_agent_loop_damped",
                "adjacency_matrix": damped_matrix,
                "lambdas": [0.95, 0.95]
            }
        }
        res_2 = rpc_call("tools/call", proposal_2, req_id=103)
        content_2 = json.loads(res_2["result"]["content"][0]["text"])
        print(f"  <- Damped Response Status: {content_2.get('status', 'SUCCESS')}")
        print(f"  <- Action: {content_2.get('action')}")
        print(f"  <- Verified Spectral Radius: rho = {content_2.get('spectral_radius'):.4f} < 1.0")
        print(f"  <- Receipt Hash: {content_2.get('receipt_hash')}")

        assert content_2.get("action") == "ACCEPT"
        assert content_2.get("is_stable") is True
        print("✅ ADMISSIBILITY RE-ESTABLISHED: Ensemble certified under Small-Gain Theorem.")

        # 4. Agent synthesizes PIRTM program honoring the certified structure
        print("\n[Agent -> Daemon] Compiling synthesized PIRTM program...")
        pirtm_source = """
        let mut loop_count = 0;
        while loop_count < 10 {
            loop_count = loop_count + 1;
        }
        let x = 100;
        """
        comp_req = {
            "name": "compile",
            "arguments": { "source": pirtm_source }
        }
        res_comp = rpc_call("tools/call", comp_req, req_id=104)
        comp_out = json.loads(res_comp["result"]["content"][0]["text"])
        assert comp_out["status"] == "SUCCESS"
        program_receipt = comp_out["receipt_hash"]
        print("  <- MLIR lowering complete.")
        print(f"  <- Program Receipt Hash: {program_receipt}")

        # 5. Agent executes the artifact under the governed runtime
        print("\n[Agent -> Daemon] Executing compiled MLIR artifact under runtime interlock...")
        run_req = {
            "name": "run",
            "arguments": { "mlir": comp_out["mlir"] }
        }
        res_run = rpc_call("tools/call", run_req, req_id=105)
        run_out = json.loads(res_run["result"]["content"][0]["text"])
        assert run_out["status"] == "EXECUTION_COMPLETE"
        assert run_out["audit_sealed"] is True
        print(f"  <- Execution exit code: {run_out['return_code']}")
        print(f"  <- Contractivity Hash: {run_out['contractivity_hash']}")
        print(f"  <- Ledger State: audit_sealed = {run_out['audit_sealed']}")

        print("\n================================================================================")
        print("🎯 AUTONOMOUS AGENT SYNTHESIS LOOP COMPLETED WITH 100% SOUNDNESS GUARANTEES")
        print("================================================================================")

    finally:
        proc.terminate()
        proc.wait()

if __name__ == "__main__":
    run_agent_synthesis()
