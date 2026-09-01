'use client';

import React, { useState } from 'react';
import {
  Compass,
  ArrowRight,
  ShieldCheck,
  Cpu,
  Terminal,
  Activity,
  CheckCircle2,
  Lock,
  Layers,
  Sparkles,
  Zap,
  BookOpen,
  Code2,
  FileCode,
  Check,
  Copy
} from 'lucide-react';
import { ADR_LIST, LEAN_THEOREMS } from '@/lib/pirtm-data';

interface HomeViewProps {
  onNavigate: (tab: string, subSection?: string) => void;
}

export function HomeView({ onNavigate }: HomeViewProps) {
  const [copied, setCopied] = useState(false);
  const [selectedPersona, setSelectedPersona] = useState<number>(0);

  const samplePirtmCode = `// PIRTM Phase-Indexed Tensor Loop
import pirtm.tensor;
import pirtm.moc;
import pirtm.ward;

fn compute_bounded_sum(n: i64) -> Result<PhaseTensor<[2, 2]>, GovernanceError> {
    let mut state = tensor.alloc_phase([2, 2], 0.25 * PI);
    let kernel = tensor.alloc_phase([2, 2], 0.0) * 0.42;

    for i in 0..n {
        state = tensor.contract_bounded(state, kernel, 0.85)?;
        ward.assert_small_gain(state)?;
    }

    Ok(state)
}`;

  const sampleMlirCode = `// Lowered to 'pirtm.moc' MLIR Dialect
module attributes {pirtm.verified = true, pirtm.lean_thm = "contractivity_small_gain_thm"} {
  func.func @compute_bounded_sum(%n: i64) -> (!pirtm.receipt, tensor<2x2xf64>) {
    %c_gain = arith.constant 0.420 : f64
    %t0 = pirtm.alloc_phase_tensor [2, 2] phase(0.7853) : tensor<2x2xf64>
    %t1 = pirtm.scale %t0, %c_gain { spectral_norm = 0.420 : f64 } : tensor<2x2xf64>
    
    // Continuous Lyapunov Small-Gain Invariant Gate (rho < 1.0)
    %receipt = pirtm.spectral_gate %t1 { max_rho = 1.000 : f64 } : !pirtm.receipt
    
    affine.for %i = 0 to %n {
      %t2 = pirtm.contract_bounded %t1, %t0 { max_gain = 0.420 : f64 } : tensor<2x2xf64>
    }
    return %receipt, %t1 : !pirtm.receipt, tensor<2x2xf64>
  }
}`;

  const personas = [
    {
      role: 'Language Enthusiast',
      title: 'Explore Type System & Phase Algebra',
      goal: 'Understand the formal syntax, phase indexing, and multiplicity operator semantics.',
      actionLabel: 'Read Language Spec',
      actionTab: 'docs',
      subSection: 'language',
      highlight: 'First-class phase manifold annotations & algebraic multiplicity rings.'
    },
    {
      role: 'Formal Methods Researcher',
      title: 'Inspect Lean Proofs & ADRs',
      goal: 'Audit machine-checked Lean 4 theorems and the zero-Mathlib microkernel proof certificates.',
      actionLabel: 'View Lean Proofs',
      actionTab: 'docs',
      subSection: 'proofs',
      highlight: 'Zero-sorry contractivity bounds & verified Lyapunov stability theorems.'
    },
    {
      role: 'DevOps / Security Engineer',
      title: 'Monitor Runtime Spectral Bounds',
      goal: 'Deploy WardMonitor to track spectral drift and enforce kill-switch boundaries in real-time.',
      actionLabel: 'Open Governance Dashboard',
      actionTab: 'dashboard',
      highlight: 'Sub-microsecond WardMonitor watchdog with Blake3 Merkle audit receipts.'
    },
    {
      role: 'AI Agent Integrator',
      title: 'Integrate MCP Server',
      goal: 'Connect autonomous agents to PIRTM MCP tools to prevent runaway recursive feedback.',
      actionLabel: 'Explore MCP Endpoints',
      actionTab: 'mcp',
      highlight: 'Standardized JSON-RPC 2.0 tools for safe AI-generated tensor pipelines.'
    }
  ];

  const handleCopyCode = () => {
    navigator.clipboard.writeText(samplePirtmCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen bg-[#080a0d] text-[#e6edf3]">
      {/* 1. Immersive UI Hero Section (Split 7 / 5 Grid) */}
      <section className="relative border-b border-[#30363d] bg-gradient-to-br from-[#0d1117] to-[#080a0d]">
        <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 min-h-[640px]">
          {/* Left Column (7 cols) */}
          <div className="lg:col-span-7 p-6 sm:p-10 md:p-12 flex flex-col justify-center space-y-8">
            <div className="space-y-4">
              <div className="inline-block px-3 py-1 bg-[#58a6ff]/10 border border-[#58a6ff]/30 text-[#58a6ff] text-xs font-mono font-semibold rounded tracking-wider">
                PHASE-INDEXED RECURSIVE TENSOR MATHEMATICS
              </div>
              <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold leading-tight tracking-tighter text-[#e6edf3]">
                Governed computation <br />
                <span className="text-[#58a6ff]">from first principles.</span>
              </h1>
              <p className="text-base sm:text-lg text-[#8b949e] max-w-xl leading-relaxed">
                PIRTM is a formally verified, runtime-enforced programming language for high-assurance systems and auditable software governance. Combining Phase-Indexed Recursive Tensor Mathematics with continuous Lyapunov small-gain enforcement.
              </p>
            </div>

            {/* CTAs */}
            <div className="flex flex-wrap gap-4">
              <button
                onClick={() => onNavigate('playground')}
                className="px-6 sm:px-8 py-3.5 sm:py-4 bg-[#238636] hover:bg-[#2ea043] text-white font-bold rounded-md shadow-lg shadow-[#238636]/20 transition-all flex items-center justify-center gap-2 cursor-pointer text-sm sm:text-base"
              >
                <Terminal className="w-4 h-4" />
                <span>Try the Playground</span>
                <ArrowRight className="w-4 h-4" />
              </button>
              <button
                onClick={() => onNavigate('docs', 'language')}
                className="px-6 sm:px-8 py-3.5 sm:py-4 bg-[#30363d] hover:bg-[#3d444d] text-[#c9d1d9] font-bold rounded-md border border-[#30363d] transition-all flex items-center justify-center gap-2 cursor-pointer text-sm sm:text-base"
              >
                <BookOpen className="w-4 h-4 text-[#58a6ff]" />
                <span>Read the Docs</span>
              </button>
            </div>

            {/* Two Quick Value Cards */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-2">
              <div className="p-4 bg-[#161b22] border border-[#30363d] rounded-lg">
                <div className="text-[#58a6ff] font-bold mb-1 flex items-center gap-2">
                  <ShieldCheck className="w-4 h-4" />
                  <span>Zero-Mathlib</span>
                </div>
                <p className="text-xs text-[#8b949e]">
                  Complete Lean 4 proofs without external mathlib dependencies.
                </p>
              </div>
              <div className="p-4 bg-[#161b22] border border-[#30363d] rounded-lg">
                <div className="text-[#3fb950] font-bold mb-1 flex items-center gap-2">
                  <Activity className="w-4 h-4" />
                  <span>Spectral Radius</span>
                </div>
                <p className="text-xs text-[#8b949e]">
                  Runtime enforcement via continuous Lyapunov small-gain theorem (ρ &lt; 1.0).
                </p>
              </div>
            </div>
          </div>

          {/* Right Column (5 cols): Live Governance Telemetry & Audit Stream */}
          <div className="lg:col-span-5 border-t lg:border-t-0 lg:border-l border-[#30363d] bg-[#0d1117] p-6 sm:p-8 flex flex-col justify-between space-y-6">
            {/* Header */}
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <h3 className="text-xs font-mono text-[#8b949e] uppercase tracking-widest">
                  Governance Dashboard
                </h3>
                <span className="text-[#3fb950] text-[10px] font-mono font-bold flex items-center gap-1.5">
                  <span className="w-2 h-2 rounded-full bg-[#3fb950] animate-pulse" />
                  LIVE TELEMETRY
                </span>
              </div>

              {/* Spectral Radius Trend Box */}
              <div className="h-44 w-full bg-[#080a0d] border border-[#30363d] rounded-lg p-4 relative flex flex-col justify-between overflow-hidden">
                <div className="flex justify-between items-center z-10">
                  <span className="text-[10px] font-mono text-[#8b949e]">SPECTRAL RADIUS (ρ) TREND</span>
                  <span className="text-[#3fb950] text-xs font-mono font-bold">ρ = 0.824</span>
                </div>

                {/* Visual Chart Bars */}
                <div className="flex items-end gap-1.5 h-20 px-1 py-1 z-10">
                  {[0.42, 0.55, 0.48, 0.62, 0.71, 0.68, 0.74, 0.79, 0.82, 0.76, 0.81, 0.824].map((val, i) => (
                    <div key={i} className="flex-1 flex flex-col justify-end h-full">
                      <div
                        className={`w-full rounded-t transition-all ${
                          val > 0.9 ? 'bg-[#d29922]' : 'bg-[#58a6ff]/40 hover:bg-[#58a6ff] border-t border-[#58a6ff]'
                        }`}
                        style={{ height: `${(val / 1.0) * 100}%` }}
                      />
                    </div>
                  ))}
                </div>

                {/* Warning threshold line */}
                <div className="absolute top-[18%] left-0 right-0 h-[1px] bg-[#d29922] opacity-40 shadow-[0_0_6px_#d29922]" />
                <div className="absolute top-[8%] right-2 text-[8px] font-mono text-[#d29922] bg-[#080a0d]/80 px-1 rounded">
                  WARN_THRESHOLD: 0.95
                </div>

                <div className="flex justify-between items-center text-[9px] font-mono text-[#8b949e] z-10">
                  <span>T-12s</span>
                  <span>T-6s</span>
                  <span>CURRENT (0.824 &lt; 1.0)</span>
                </div>
              </div>
            </div>

            {/* Live Audit Log */}
            <div className="space-y-2 flex-1 flex flex-col">
              <div className="flex items-center justify-between">
                <h3 className="text-xs font-mono text-[#8b949e] uppercase tracking-widest">
                  Live Audit Log
                </h3>
                <span className="text-[10px] font-mono text-[#8b949e]">Blake3 Merkle Chain</span>
              </div>
              <div className="flex-1 bg-[#080a0d] border border-[#30363d] rounded-lg font-mono text-[11px] p-3 space-y-1.5 overflow-hidden">
                <div className="flex items-center space-x-3">
                  <span className="text-[#8b949e]">14:22:01</span>
                  <span className="text-[#3fb950] font-bold text-[10px] px-1 py-0.2 bg-[#238636]/20 rounded">VALID</span>
                  <span className="text-[#c9d1d9] truncate">Receipt generated: 0x8f2c...e4</span>
                </div>
                <div className="flex items-center space-x-3">
                  <span className="text-[#8b949e]">14:22:15</span>
                  <span className="text-[#58a6ff] font-bold text-[10px] px-1 py-0.2 bg-[#58a6ff]/20 rounded">INFO</span>
                  <span className="text-[#c9d1d9] truncate">MLIR Lowering pass 4 completed</span>
                </div>
                <div className="flex items-center space-x-3">
                  <span className="text-[#8b949e]">14:23:42</span>
                  <span className="text-[#3fb950] font-bold text-[10px] px-1 py-0.2 bg-[#238636]/20 rounded">PASS</span>
                  <span className="text-[#c9d1d9] truncate">Theorem recursive_tensor_stability verified</span>
                </div>
                <div className="flex items-center space-x-3">
                  <span className="text-[#8b949e]">14:24:10</span>
                  <span className="text-[#d29922] font-bold text-[10px] px-1 py-0.2 bg-[#d29922]/20 rounded">WARN</span>
                  <span className="text-[#c9d1d9] truncate">Drift detected in MOC-WardMonitor (+2.1%)</span>
                </div>
                <div className="flex items-center space-x-3">
                  <span className="text-[#8b949e]">14:25:02</span>
                  <span className="text-[#3fb950] font-bold text-[10px] px-1 py-0.2 bg-[#238636]/20 rounded">SYNC</span>
                  <span className="text-[#c9d1d9] truncate">MCP Server session heart-beat OK</span>
                </div>
              </div>
            </div>

            {/* Governance Gate Status Banner */}
            <div className="p-4 border border-[#30363d] rounded-lg flex items-center justify-between bg-gradient-to-r from-[#161b22] to-transparent">
              <div>
                <div className="text-[10px] font-mono text-[#8b949e] uppercase mb-0.5">Governance Gate</div>
                <div className="text-sm font-bold text-[#3fb950] flex items-center gap-1.5">
                  <CheckCircle2 className="w-4 h-4 text-[#3fb950]" />
                  <span>NOMINAL ENFORCEMENT</span>
                </div>
              </div>
              <div className="text-right">
                <div className="text-[10px] font-mono text-[#8b949e] uppercase mb-0.5">WardMonitor</div>
                <div className="text-[10px] text-[#3fb950] bg-[#238636]/20 px-2 py-0.5 rounded border border-[#238636] font-mono font-bold">
                  ACTIVE
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* 2. Quick Stats Grid (4 Cards) */}
      <section className="py-16 bg-[#080a0d] border-b border-[#30363d]">
        <div className="max-w-7xl mx-auto px-4 sm:px-8">
          <div className="text-center max-w-2xl mx-auto mb-12">
            <h2 className="text-2xl sm:text-3xl font-bold text-[#e6edf3] tracking-tight">
              A Rigorous Foundation for Governed AI & Systems
            </h2>
            <p className="mt-2 text-sm text-[#8b949e]">
              Engineered from first mathematical principles to bridge formal verification with high-performance execution.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {/* Card 1 */}
            <div className="p-6 rounded-xl bg-[#161b22] border border-[#30363d] hover:border-[#58a6ff]/50 transition-all group flex flex-col justify-between">
              <div>
                <div className="w-10 h-10 rounded-lg bg-[#238636]/20 border border-[#238636]/40 text-[#3fb950] flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                  <ShieldCheck className="w-5 h-5" />
                </div>
                <h3 className="font-bold text-[#e6edf3] text-base mb-2">
                  Zero-Mathlib Lean Proofs
                </h3>
                <p className="text-xs text-[#8b949e] leading-relaxed">
                  Self-contained Lean 4 microkernel verification. Proves bounded Lyapunov stability and contractivity in under 3.2 seconds.
                </p>
              </div>
              <button
                onClick={() => onNavigate('docs', 'proofs')}
                className="mt-4 pt-3 border-t border-[#30363d] flex items-center justify-between text-xs text-[#3fb950] font-medium hover:underline cursor-pointer"
              >
                <span>Inspect Proof Ledger</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>

            {/* Card 2 */}
            <div className="p-6 rounded-xl bg-[#161b22] border border-[#30363d] hover:border-[#58a6ff]/50 transition-all group flex flex-col justify-between">
              <div>
                <div className="w-10 h-10 rounded-lg bg-[#58a6ff]/20 border border-[#58a6ff]/40 text-[#58a6ff] flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                  <Activity className="w-5 h-5" />
                </div>
                <h3 className="font-bold text-[#e6edf3] text-base mb-2">
                  Spectral Small-Gain Gate
                </h3>
                <p className="text-xs text-[#8b949e] leading-relaxed">
                  Continuous Lyapunov state monitor (WardMonitor) halts execution if spectral radius ρ ≥ 1.0, eliminating runaway loops.
                </p>
              </div>
              <button
                onClick={() => onNavigate('dashboard')}
                className="mt-4 pt-3 border-t border-[#30363d] flex items-center justify-between text-xs text-[#58a6ff] font-medium hover:underline cursor-pointer"
              >
                <span>Open WardMonitor</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>

            {/* Card 3 */}
            <div className="p-6 rounded-xl bg-[#161b22] border border-[#30363d] hover:border-[#58a6ff]/50 transition-all group flex flex-col justify-between">
              <div>
                <div className="w-10 h-10 rounded-lg bg-[#58a6ff]/20 border border-[#58a6ff]/40 text-[#58a6ff] flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                  <Layers className="w-5 h-5" />
                </div>
                <h3 className="font-bold text-[#e6edf3] text-base mb-2">
                  MLIR + LLVM Backend
                </h3>
                <p className="text-xs text-[#8b949e] leading-relaxed">
                  Lowers Multiplicity Operator Calculus (MOC) into affine polyhedral loops with AVX-512 and GPU tensor vectorization.
                </p>
              </div>
              <button
                onClick={() => onNavigate('docs', 'language')}
                className="mt-4 pt-3 border-t border-[#30363d] flex items-center justify-between text-xs text-[#58a6ff] font-medium hover:underline cursor-pointer"
              >
                <span>Read MLIR Specs</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>

            {/* Card 4 */}
            <div className="p-6 rounded-xl bg-[#161b22] border border-[#30363d] hover:border-[#58a6ff]/50 transition-all group flex flex-col justify-between">
              <div>
                <div className="w-10 h-10 rounded-lg bg-[#d29922]/20 border border-[#d29922]/40 text-[#d29922] flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
                  <Cpu className="w-5 h-5" />
                </div>
                <h3 className="font-bold text-[#e6edf3] text-base mb-2">
                  Live MCP Server for AI Agents
                </h3>
                <p className="text-xs text-[#8b949e] leading-relaxed">
                  JSON-RPC 2.0 Model Context Protocol tools enable Claude, Cursor, and agent runtimes to validate contractivity dynamically.
                </p>
              </div>
              <button
                onClick={() => onNavigate('mcp')}
                className="mt-4 pt-3 border-t border-[#30363d] flex items-center justify-between text-xs text-[#d29922] font-medium hover:underline cursor-pointer"
              >
                <span>Test MCP Tools</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        </div>
      </section>

      {/* 3. Featured Example: Side-by-Side Code & MLIR Lowering */}
      <section className="py-16 bg-[#0d1117] border-b border-[#30363d]">
        <div className="max-w-7xl mx-auto px-4 sm:px-8">
          <div className="flex flex-col md:flex-row md:items-end justify-between mb-8 gap-4">
            <div>
              <div className="flex items-center gap-2 text-xs font-mono text-[#58a6ff] mb-1 font-semibold">
                <FileCode className="w-4 h-4" />
                <span>CONTRACTIVITY PROVENANCE</span>
              </div>
              <h2 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
                From PIRTM Source to Verified MLIR Bytecode
              </h2>
              <p className="text-xs sm:text-sm text-[#8b949e] mt-1">
                Every tensor recursion is formally checked in Lean 4 and lowered into affine MLIR with cryptographic audit receipts.
              </p>
            </div>

            <div className="flex items-center gap-3">
              <button
                onClick={handleCopyCode}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-[#161b22] hover:bg-[#21262d] border border-[#30363d] text-xs text-[#e6edf3] transition-colors cursor-pointer"
              >
                {copied ? <Check className="w-3.5 h-3.5 text-[#3fb950]" /> : <Copy className="w-3.5 h-3.5 text-[#8b949e]" />}
                <span>{copied ? 'Copied' : 'Copy PIRTM Code'}</span>
              </button>

              <button
                onClick={() => onNavigate('playground')}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-[#238636] hover:bg-[#2ea043] text-white text-xs font-bold shadow-md transition-all cursor-pointer"
              >
                <span>Run in Playground</span>
                <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>

          {/* Split Pane Display */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {/* Left: PIRTM Source */}
            <div className="rounded-lg border border-[#30363d] bg-[#080a0d] overflow-hidden shadow-2xl flex flex-col">
              <div className="px-4 py-2.5 bg-[#161b22] border-b border-[#30363d] flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="flex gap-1.5">
                    <span className="w-2.5 h-2.5 rounded-full bg-[#f85149]" />
                    <span className="w-2.5 h-2.5 rounded-full bg-[#d29922]" />
                    <span className="w-2.5 h-2.5 rounded-full bg-[#3fb950]" />
                  </div>
                  <span className="text-xs font-mono text-[#8b949e] ml-2">compute_sum.pirtm</span>
                </div>
                <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#58a6ff]/10 text-[#58a6ff] border border-[#58a6ff]/30 font-semibold">
                  Phase-Indexed Source
                </span>
              </div>
              <div className="p-4 font-mono text-xs text-[#e6edf3] overflow-x-auto leading-relaxed">
                <pre className="text-[#58a6ff]">
                  <code>{samplePirtmCode}</code>
                </pre>
              </div>
              <div className="mt-auto px-4 py-2 bg-[#161b22] border-t border-[#30363d] flex items-center justify-between text-[11px] font-mono text-[#8b949e]">
                <span>Phase Manifold: π/4</span>
                <span className="text-[#3fb950] font-semibold">Spectral Radius: ρ = 0.420 &lt; 1.0</span>
              </div>
            </div>

            {/* Right: Generated MLIR & Governance Receipt */}
            <div className="rounded-lg border border-[#30363d] bg-[#080a0d] overflow-hidden shadow-2xl flex flex-col">
              <div className="px-4 py-2.5 bg-[#161b22] border-b border-[#30363d] flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="flex gap-1.5">
                    <span className="w-2.5 h-2.5 rounded-full bg-[#58a6ff]" />
                    <span className="w-2.5 h-2.5 rounded-full bg-[#58a6ff]/70" />
                    <span className="w-2.5 h-2.5 rounded-full bg-[#58a6ff]/40" />
                  </div>
                  <span className="text-xs font-mono text-[#8b949e] ml-2">lowered.mlir</span>
                </div>
                <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#238636]/10 text-[#3fb950] border border-[#238636]/30 font-bold">
                  Lean 4 Certified
                </span>
              </div>
              <div className="p-4 font-mono text-xs text-[#3fb950] overflow-x-auto leading-relaxed">
                <pre className="text-[#3fb950]/90">
                  <code>{sampleMlirCode}</code>
                </pre>
              </div>
              {/* Receipt Hash Footer */}
              <div className="mt-auto px-4 py-2.5 bg-[#161b22] border-t border-[#30363d] flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2 text-[11px] font-mono">
                <div className="flex items-center gap-2 text-[#8b949e]">
                  <ShieldCheck className="w-4 h-4 text-[#3fb950]" />
                  <span>Receipt: 0x8f2a991c4be29871fa093128</span>
                </div>
                <span className="px-2 py-0.5 rounded bg-[#238636]/20 text-[#3fb950] border border-[#238636] text-[10px] font-bold">
                  STATUS: CERTIFIED
                </span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* 4. Complete Execution Pipeline Architecture */}
      <section className="py-16 bg-[#080a0d] border-b border-[#30363d]">
        <div className="max-w-7xl mx-auto px-4 sm:px-8">
          <div className="text-center max-w-2xl mx-auto mb-12">
            <h2 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
              The Governed Computation Pipeline
            </h2>
            <p className="mt-2 text-sm text-[#8b949e]">
              A deterministic 4-stage pipeline that guarantees mathematical safety before emitting machine code.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-4 gap-4 relative">
            {/* Step 1 */}
            <div className="p-5 rounded-lg bg-[#161b22] border border-[#30363d] relative">
              <div className="text-xs font-mono text-[#58a6ff] font-bold mb-1">01. FRONTEND</div>
              <h3 className="font-bold text-[#e6edf3] text-sm mb-1">Phase AST Parsing</h3>
              <p className="text-xs text-[#8b949e] leading-relaxed">
                Constructs phase-indexed tensor manifolds and multiplicity operators $Ap(k)$.
              </p>
              <div className="mt-3 text-[11px] font-mono text-[#8b949e]">EBNF Grammar Check</div>
            </div>

            {/* Step 2 */}
            <div className="p-5 rounded-lg bg-[#161b22] border border-[#30363d] relative">
              <div className="text-xs font-mono text-[#3fb950] font-bold mb-1">02. PROOF KERNEL</div>
              <h3 className="font-bold text-[#e6edf3] text-sm mb-1">Lean 4 Verification</h3>
              <p className="text-xs text-[#8b949e] leading-relaxed">
                Evaluates matrix norm bounds and checks Lyapunov theorem certificates with zero-sorry.
              </p>
              <div className="mt-3 text-[11px] font-mono text-[#3fb950]">lake build: 3.1s</div>
            </div>

            {/* Step 3 */}
            <div className="p-5 rounded-lg bg-[#161b22] border border-[#30363d] relative">
              <div className="text-xs font-mono text-[#58a6ff] font-bold mb-1">03. COMPILER</div>
              <h3 className="font-bold text-[#e6edf3] text-sm mb-1">MLIR Lowering</h3>
              <p className="text-xs text-[#8b949e] leading-relaxed">
                Lowers `pirtm.moc` ops to affine dialect loops with polyhedral SIMD optimization.
              </p>
              <div className="mt-3 text-[11px] font-mono text-[#58a6ff]">LLVM IR Generation</div>
            </div>

            {/* Step 4 */}
            <div className="p-5 rounded-lg bg-[#161b22] border border-[#30363d] relative">
              <div className="text-xs font-mono text-[#d29922] font-bold mb-1">04. RUNTIME</div>
              <h3 className="font-bold text-[#e6edf3] text-sm mb-1">WardMonitor Enforcer</h3>
              <p className="text-xs text-[#8b949e] leading-relaxed">
                Tracks streaming drift in real-time. Automatically halts if spectral radius crosses 1.0.
              </p>
              <div className="mt-3 text-[11px] font-mono text-[#d29922]">Blake3 Merkle Receipt</div>
            </div>
          </div>
        </div>
      </section>

      {/* 5. User Personas Interactive Section */}
      <section className="py-16 bg-[#0d1117]">
        <div className="max-w-7xl mx-auto px-4 sm:px-8">
          <div className="text-center max-w-2xl mx-auto mb-10">
            <h2 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
              Tailored for Researchers, Developers &amp; Auditors
            </h2>
            <p className="mt-2 text-sm text-[#8b949e]">
              Select your persona to discover relevant tools, formal specifications, and integrations.
            </p>
          </div>

          {/* Persona selector tabs */}
          <div className="flex flex-wrap items-center justify-center gap-2 mb-8">
            {personas.map((p, idx) => (
              <button
                key={idx}
                onClick={() => setSelectedPersona(idx)}
                className={`px-4 py-2 rounded-md text-xs font-semibold transition-all cursor-pointer ${
                  selectedPersona === idx
                    ? 'bg-[#161b22] text-[#58a6ff] border border-[#58a6ff]/40 shadow-sm'
                    : 'bg-[#080a0d] text-[#8b949e] border border-[#30363d] hover:text-[#e6edf3]'
                }`}
              >
                {p.role}
              </button>
            ))}
          </div>

          {/* Persona active card */}
          {personas[selectedPersona] && (
            <div className="max-w-3xl mx-auto p-6 sm:p-8 rounded-xl bg-[#161b22] border border-[#30363d] shadow-xl">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-4">
                <div>
                  <span className="text-xs font-mono text-[#58a6ff] uppercase tracking-wider font-semibold">
                    {personas[selectedPersona].role}
                  </span>
                  <h3 className="text-xl font-bold text-[#e6edf3] mt-1">
                    {personas[selectedPersona].title}
                  </h3>
                </div>
                <button
                  onClick={() => onNavigate(personas[selectedPersona].actionTab, personas[selectedPersona].subSection)}
                  className="flex items-center gap-2 px-4 py-2 rounded-md bg-[#238636] hover:bg-[#2ea043] text-white text-xs font-bold shadow-md transition-all self-start sm:self-auto cursor-pointer"
                >
                  <span>{personas[selectedPersona].actionLabel}</span>
                  <ArrowRight className="w-3.5 h-3.5" />
                </button>
              </div>

              <p className="text-sm text-[#8b949e] mb-4 leading-relaxed">
                {personas[selectedPersona].goal}
              </p>

              <div className="p-4 rounded-md bg-[#080a0d] border border-[#30363d] flex items-center gap-3 text-xs">
                <Sparkles className="w-4 h-4 text-[#58a6ff] shrink-0" />
                <span className="text-[#c9d1d9] font-medium font-mono">
                  {personas[selectedPersona].highlight}
                </span>
              </div>
            </div>
          )}
        </div>
      </section>
    </div>
  );
}
