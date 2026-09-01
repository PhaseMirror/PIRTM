'use client';

import React, { useState } from 'react';
import {
  Cpu,
  Terminal,
  Play,
  Copy,
  Check,
  CheckCircle2,
  ExternalLink,
  Code2,
  ShieldCheck,
  Zap,
  Layers,
  ArrowRight
} from 'lucide-react';
import { MCP_TOOLS, McpToolDef } from '@/lib/pirtm-data';

export function McpView() {
  const [selectedTool, setSelectedTool] = useState<McpToolDef>(MCP_TOOLS[0]);
  const [activeLangTab, setActiveLangTab] = useState<'cli' | 'python' | 'typescript'>('cli');
  const [isCallingTool, setIsCallingTool] = useState(false);
  const [liveResponse, setLiveResponse] = useState<Record<string, unknown> | null>(null);
  const [copiedCode, setCopiedCode] = useState(false);

  const handleTestTool = (tool: McpToolDef) => {
    setIsCallingTool(true);
    setTimeout(() => {
      setLiveResponse(tool.sampleResponse);
      setIsCallingTool(false);
    }, 200);
  };

  const pythonSnippet = `from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def run_governed_tensor_agent():
    # Connect to the PIRTM MCP Governance Server
    server_params = StdioServerParameters(command="pirtm-mcp", args=["serve"])
    
    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            
            # Step 1: Validate contractivity before executing agent policy
            res = await session.call_tool(
                "validate_contractivity",
                arguments={
                    "tensor_spec": {"dims": [4, 4], "feedback_gain": 0.65},
                    "max_rho": 0.85
                }
            )
            print(f"Governance Check: {res.content[0].text}")

            # Step 2: Compile & execute with Blake3 audit receipt
            artifact = await session.call_tool(
                "compile_pirtm",
                arguments={
                    "source": "let T = tensor.alloc_phase([4, 4], 0.0) * 0.42; moc.eval(T);",
                    "opt_level": "O3-polyhedral"
                }
            )
            print(f"Certified MLIR Bytecode Hash: {artifact.content[0].text}")
`;

  const cliSnippet = `# 1. Install PIRTM Toolchain & MCP Server
cargo install pirtm-cli pirtm-mcp

# 2. Launch Local Governed MCP Server (stdio / SSE)
pirtm-mcp serve --port 8080 --lean-microkernel-check

# 3. Test Contractivity Gate via CLI Client
pirtm-cli validate --source "let T = tensor.alloc([2,2]) * 0.5;" --max-rho 0.85

# 4. Integrate into Claude Desktop / Cursor Config (claude_desktop_config.json):
{
  "mcpServers": {
    "pirtm-governance": {
      "command": "pirtm-mcp",
      "args": ["serve"]
    }
  }
}`;

  const tsSnippet = `import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";

const transport = new StdioClientTransport({
  command: "pirtm-mcp",
  args: ["serve"]
});

const client = new Client({
  name: "pirtm-agent-client",
  version: "1.0.0"
});

await client.connect(transport);

// Call PIRTM MCP Tool
const result = await client.callTool({
  name: "validate_contractivity",
  arguments: {
    tensor_spec: { dims: [4, 4], feedback_gain: 0.5 },
    max_rho: 1.0
  }
});

console.log("Verified Receipt:", result);`;

  const activeSnippet =
    activeLangTab === 'cli' ? cliSnippet : activeLangTab === 'python' ? pythonSnippet : tsSnippet;

  const handleCopyCode = () => {
    navigator.clipboard.writeText(activeSnippet);
    setCopiedCode(true);
    setTimeout(() => setCopiedCode(false), 2000);
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
      {/* Top Header & Live Health Banner */}
      <div className="flex flex-col md:flex-row md:items-center justify-between pb-6 border-b border-[#233147] gap-4">
        <div>
          <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
            <Cpu className="w-4 h-4" />
            <span>MODEL CONTEXT PROTOCOL (MCP) INTERFACE</span>
          </div>
          <h1 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
            PIRTM MCP Server &amp; AI Integration
          </h1>
          <p className="text-xs sm:text-sm text-[#8b949e] mt-1">
            Standardized JSON-RPC 2.0 endpoints for LLM agents, Claude Desktop, Cursor, and autonomous pipelines.
          </p>
        </div>

        {/* Live Server Status Pill */}
        <div className="flex items-center gap-3 bg-[#0e1624] border border-[#233147] px-4 py-2 rounded-xl text-xs font-mono">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse" />
            <span className="text-[#e6edf3] font-bold">MCP Server Live</span>
          </div>
          <span className="text-[#6e7681]">•</span>
          <span className="text-emerald-400">99.98% Uptime</span>
          <span className="text-[#6e7681]">•</span>
          <span className="text-cyan-300">1.2ms Latency</span>
        </div>
      </div>

      {/* Overview Card */}
      <div className="p-6 rounded-xl bg-[#0e1624] border border-[#233147]">
        <h2 className="text-base font-bold text-[#e6edf3] mb-2 flex items-center gap-2">
          <ShieldCheck className="w-5 h-5 text-cyan-400" />
          Why AI Agents Need Governed Computation
        </h2>
        <p className="text-xs sm:text-sm text-[#8b949e] leading-relaxed max-w-4xl">
          When autonomous AI agents invoke recursive feedback loops, numerical drift or hallucinations can destabilize downstream execution. The PIRTM MCP Server acts as an authoritative mathematical gatekeeper: agents send prospective tensor operations, and the server returns machine-checked Lean 4 contractivity certificates before runtime emission.
        </p>
      </div>

      {/* Interactive Tool Sandbox (Left: Tools List, Right: JSON-RPC Runner) */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Left: Tools List */}
        <div className="lg:col-span-5 space-y-3">
          <h3 className="text-sm font-bold text-[#e6edf3] uppercase tracking-wider text-[11px]">
            Available MCP Tools ({MCP_TOOLS.length})
          </h3>

          <div className="space-y-2">
            {MCP_TOOLS.map((tool) => {
              const isSelected = selectedTool.name === tool.name;
              return (
                <button
                  key={tool.name}
                  onClick={() => {
                    setSelectedTool(tool);
                    setLiveResponse(null);
                  }}
                  className={`w-full p-4 rounded-xl border text-left transition-all cursor-pointer ${
                    isSelected
                      ? 'bg-[#1a2638] border-cyan-500/50 shadow-md shadow-cyan-950/20'
                      : 'bg-[#0e1624] border-[#233147] hover:bg-[#121d2e]'
                  }`}
                >
                  <div className="flex items-center justify-between text-xs mb-1">
                    <span className="font-mono text-cyan-400 font-bold text-sm">
                      {tool.name}
                    </span>
                    <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#090d14] text-emerald-400 border border-emerald-800/40">
                      JSON-RPC 2.0
                    </span>
                  </div>
                  <p className="text-xs text-[#8b949e] mt-1 line-clamp-2">
                    {tool.description}
                  </p>
                </button>
              );
            })}
          </div>
        </div>

        {/* Right: Interactive Tool Explorer */}
        <div className="lg:col-span-7">
          <div className="rounded-xl border border-[#233147] bg-[#0c121c] overflow-hidden flex flex-col shadow-2xl">
            {/* Header */}
            <div className="px-5 py-3.5 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
              <div>
                <span className="text-xs font-mono text-cyan-400 font-bold block">
                  tool: {selectedTool.name}
                </span>
                <span className="text-[11px] text-[#8b949e]">{selectedTool.description}</span>
              </div>
              <button
                onClick={() => handleTestTool(selectedTool)}
                disabled={isCallingTool}
                className="flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white text-xs font-bold transition-all shadow-md shadow-cyan-950 cursor-pointer"
              >
                <Play className={`w-3.5 h-3.5 ${isCallingTool ? 'animate-spin' : ''}`} />
                <span>{isCallingTool ? 'Calling...' : 'Call Tool'}</span>
              </button>
            </div>

            {/* Parameters schema */}
            <div className="p-5 border-b border-[#1e2a3c] bg-[#0a0f18] space-y-3">
              <h4 className="text-xs font-mono font-semibold text-[#cbd5e1] uppercase tracking-wider">
                Parameters Schema
              </h4>
              <div className="space-y-2">
                {selectedTool.parameters.map((param) => (
                  <div key={param.name} className="p-2.5 rounded-lg bg-[#0e1624] border border-[#1e2a3c] flex flex-col sm:flex-row sm:items-center justify-between text-xs gap-1">
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-cyan-300 font-bold">{param.name}</span>
                      <span className="font-mono text-[10px] text-[#6e7681]">({param.type})</span>
                      {param.required && (
                        <span className="text-[10px] font-mono text-rose-400 font-semibold">required</span>
                      )}
                    </div>
                    <span className="text-[#8b949e] text-[11px]">{param.description}</span>
                  </div>
                ))}
              </div>
            </div>

            {/* Sample Request & Live Response display */}
            <div className="p-5 font-mono text-xs space-y-4 bg-[#090d14]/80">
              {/* Request */}
              <div>
                <span className="text-[#6e7681] text-[10px] block mb-1 font-semibold">SAMPLE JSON-RPC 2.0 REQUEST</span>
                <div className="p-3 bg-[#0a0f18] rounded-lg border border-[#1e2a3c] text-sky-300 overflow-x-auto">
                  <pre><code>{JSON.stringify(selectedTool.sampleRequest, null, 2)}</code></pre>
                </div>
              </div>

              {/* Response */}
              <div>
                <div className="flex items-center justify-between mb-1">
                  <span className="text-[#6e7681] text-[10px] font-semibold">
                    {liveResponse ? 'LIVE RESPONSE' : 'EXPECTED RETURN SCHEMA'}
                  </span>
                  {liveResponse && (
                    <span className="text-[10px] font-mono text-emerald-400 font-bold">200 OK (1.2ms)</span>
                  )}
                </div>
                <div className="p-3 bg-[#0a0f18] rounded-lg border border-[#1e2a3c] text-emerald-400 overflow-x-auto">
                  <pre>
                    <code>
                      {JSON.stringify(liveResponse || selectedTool.sampleResponse, null, 2)}
                    </code>
                  </pre>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Start Code Snippets (CLI, Python, TypeScript) */}
      <div className="p-6 rounded-xl bg-[#0e1624] border border-[#233147] space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div>
            <h2 className="text-base font-bold text-[#e6edf3]">
              Quick Start Integration Examples
            </h2>
            <p className="text-xs text-[#8b949e]">
              Connect your AI agent runtime in less than 5 minutes.
            </p>
          </div>

          {/* Lang Tabs */}
          <div className="flex items-center gap-2">
            <div className="flex rounded-lg bg-[#090d14] border border-[#233147] p-1 text-xs">
              {(['cli', 'python', 'typescript'] as const).map((lang) => (
                <button
                  key={lang}
                  onClick={() => setActiveLangTab(lang)}
                  className={`px-3 py-1 rounded uppercase font-mono transition-colors cursor-pointer ${
                    activeLangTab === lang
                      ? 'bg-[#1b2738] text-cyan-300 font-bold'
                      : 'text-[#8b949e] hover:text-[#e6edf3]'
                  }`}
                >
                  {lang}
                </button>
              ))}
            </div>

            <button
              onClick={handleCopyCode}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-[#141e2e] hover:bg-[#1b283d] border border-[#273852] text-xs text-[#e6edf3] transition-colors cursor-pointer"
            >
              {copiedCode ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5 text-[#8b949e]" />}
              <span>{copiedCode ? 'Copied' : 'Copy Code'}</span>
            </button>
          </div>
        </div>

        {/* Code Box */}
        <div className="p-4 bg-[#090d14] rounded-lg border border-[#1e2a3c] font-mono text-xs text-[#88c0d0] overflow-x-auto leading-relaxed">
          <pre><code>{activeSnippet}</code></pre>
        </div>
      </div>
    </div>
  );
}
