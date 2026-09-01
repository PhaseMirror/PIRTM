'use client';

import React from 'react';
import {
  Compass,
  ShieldCheck,
  Github,
  Award,
  BookOpen,
  CheckCircle2,
  Users,
  Code2,
  ExternalLink,
  Lock
} from 'lucide-react';

export function AboutView() {
  const teamGroups = [
    {
      group: 'Formal Methods Working Group',
      focus: 'Lean 4 microkernel verification, zero-Mathlib proof engineering, and Lyapunov stability proofs.'
    },
    {
      group: 'Compiler Architecture Team',
      focus: 'MLIR polyhedral lowering, affine dialect optimization passes, and AVX-512/GPU code generation.'
    },
    {
      group: 'Runtime Safety Guild',
      focus: 'WardMonitor real-time drift tracking, sub-microsecond kill-switch circuitry, and hardware integration.'
    },
    {
      group: 'AI Governance & MCP Integration',
      focus: 'Model Context Protocol server, Claude Desktop integration, and agent safety certification tooling.'
    }
  ];

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-10">
      {/* Header */}
      <div className="pb-6 border-b border-[#233147]">
        <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
          <Compass className="w-4 h-4" />
          <span>PROJECT ARCHITECTURE &amp; ORIGINS</span>
        </div>
        <h1 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
          About PIRTM &amp; Multiplicity Operator Calculus
        </h1>
        <p className="text-xs sm:text-sm text-[#8b949e] mt-1 max-w-3xl">
          An open formal methods and compiler initiative establishing guaranteed mathematical stability for autonomous AI agents and critical systems.
        </p>
      </div>

      {/* Core Mission & Philosophy */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div className="p-6 rounded-xl bg-[#0e1624] border border-[#202e42] space-y-4">
          <h2 className="text-base font-bold text-[#e6edf3] flex items-center gap-2">
            <ShieldCheck className="w-5 h-5 text-cyan-400" />
            The Core Philosophy
          </h2>
          <p className="text-xs sm:text-sm text-[#8b949e] leading-relaxed">
            Modern software engineering and AI agent loops increasingly operate over continuous, high-dimensional tensor state spaces. Traditional type systems verify only structural shapes (e.g., matrix dimensions), but fail to prevent exponential numerical amplification or feedback divergence.
          </p>
          <p className="text-xs sm:text-sm text-[#8b949e] leading-relaxed">
            PIRTM enforces the <strong>continuous small-gain condition (ρ &lt; 1.0)</strong> as a compile-time and runtime invariant, proving that every feedback loop strictly contracts Lyapunov energy towards deterministic fixed points.
          </p>
        </div>

        <div className="p-6 rounded-xl bg-[#0e1624] border border-[#202e42] space-y-4">
          <h2 className="text-base font-bold text-[#e6edf3] flex items-center gap-2">
            <Code2 className="w-5 h-5 text-emerald-400" />
            Technical Architecture
          </h2>
          <div className="space-y-2 text-xs font-mono">
            <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
              <span className="text-[#8b949e]">Formal Verifier:</span>
              <span className="text-cyan-300 font-semibold">Lean 4.8.0 (Zero-Mathlib)</span>
            </div>
            <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
              <span className="text-[#8b949e]">Intermediate Representation:</span>
              <span className="text-sky-300 font-semibold">MLIR (pirtm.moc Dialect)</span>
            </div>
            <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
              <span className="text-[#8b949e]">Cryptographic Receipts:</span>
              <span className="text-amber-300 font-semibold">Blake3 Merkle Trees</span>
            </div>
            <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c] flex items-center justify-between">
              <span className="text-[#8b949e]">AI Agent Protocol:</span>
              <span className="text-emerald-300 font-semibold">Model Context Protocol (MCP)</span>
            </div>
          </div>
        </div>
      </div>

      {/* Research Working Groups */}
      <div className="space-y-4">
        <h2 className="text-base font-bold text-[#e6edf3] flex items-center gap-2">
          <Users className="w-5 h-5 text-cyan-400" />
          Research &amp; Engineering Working Groups
        </h2>

        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          {teamGroups.map((g, i) => (
            <div key={i} className="p-5 rounded-xl bg-[#0e1624] border border-[#202e42]">
              <h3 className="font-bold text-[#e6edf3] text-sm mb-1.5">{g.group}</h3>
              <p className="text-xs text-[#8b949e] leading-relaxed">{g.focus}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Licensing & Governance Section */}
      <div className="p-6 rounded-xl bg-[#0e1624] border border-[#233147] space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 pb-3 border-b border-[#1e2c40]">
          <div>
            <h2 className="text-base font-bold text-[#e6edf3]">
              Prime Materia &amp; CC BY-SA 4.0 Licensing
            </h2>
            <p className="text-xs text-[#8b949e]">
              Open-source formal specifications and governed toolchain code.
            </p>
          </div>
          <a
            href="https://github.com/PhaseMirror/PIRTM"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-[#141e2e] hover:bg-[#1b283d] text-cyan-300 text-xs font-semibold border border-[#273852] transition-colors self-start sm:self-auto"
          >
            <Github className="w-4 h-4" />
            <span>GitHub Repository</span>
            <ExternalLink className="w-3 h-3 ml-1" />
          </a>
        </div>

        <div className="text-xs text-[#8b949e] leading-relaxed space-y-2">
          <p>
            The PIRTM formal mathematical specifications, Lean 4 proof ledgers, and Architecture Decision Records (ADRs) are released under the <strong>Creative Commons Attribution-ShareAlike 4.0 International License (CC BY-SA 4.0)</strong>.
          </p>
          <p>
            The compiler binaries, MLIR dialect lowers, and runtime WardMonitor components are distributed under the <strong>Prime Materia Open License</strong>, granting unrestricted research, deployment, and audit rights for verified autonomous systems.
          </p>
        </div>
      </div>
    </div>
  );
}
