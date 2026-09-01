'use client';

import React from 'react';
import { Compass, Github, Shield, CheckCircle2, Cpu, ExternalLink } from 'lucide-react';

interface FooterProps {
  onTabChange: (tab: string, subSection?: string) => void;
}

export function Footer({ onTabChange }: FooterProps) {
  return (
    <footer className="w-full border-t border-[#30363d] bg-[#0d1117] text-[#8b949e] text-xs">
      <div className="max-w-7xl mx-auto px-4 sm:px-8 py-12">
        <div className="grid grid-cols-1 md:grid-cols-5 gap-8 mb-12">
          {/* Brand Col */}
          <div className="md:col-span-2 space-y-3">
            <div className="flex items-center space-x-2.5">
              <div className="w-7 h-7 bg-[#58a6ff] rounded flex items-center justify-center font-bold text-black text-xs italic">
                P
              </div>
              <span className="font-bold text-sm text-[#e6edf3] tracking-tight">
                PIRTM / MOC Toolchain
              </span>
            </div>
            <p className="text-xs text-[#8b949e] leading-relaxed max-w-sm">
              Phase-Indexed Recursive Tensor Mathematics and Multiplicity Operator Calculus. Formally verified in Lean 4 with continuous small-gain runtime enforcement.
            </p>
            <div className="flex flex-wrap gap-2 pt-2">
              <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded bg-[#238636]/10 text-[#3fb950] border border-[#238636]/30 font-mono text-[10px]">
                <CheckCircle2 className="w-3 h-3" />
                lake build: passing (Lean 4.8.0)
              </span>
              <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded bg-[#58a6ff]/10 text-[#58a6ff] border border-[#58a6ff]/30 font-mono text-[10px]">
                <Cpu className="w-3 h-3" />
                dialect: pirtm.moc
              </span>
            </div>
          </div>

          {/* Documentation Links */}
          <div>
            <h4 className="font-semibold text-[#e6edf3] mb-3 uppercase tracking-wider text-[11px]">
              Documentation
            </h4>
            <ul className="space-y-2">
              <li>
                <button
                  onClick={() => onTabChange('docs', 'language')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Language Reference
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('docs', 'adrs')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  ADR Index (Decisions)
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('docs', 'proofs')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Lean 4 Proofs Ledger
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('docs', 'tutorials')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Tutorials & Guides
                </button>
              </li>
            </ul>
          </div>

          {/* Tools & Runtime */}
          <div>
            <h4 className="font-semibold text-[#e6edf3] mb-3 uppercase tracking-wider text-[11px]">
              Toolchain & Governance
            </h4>
            <ul className="space-y-2">
              <li>
                <button
                  onClick={() => onTabChange('playground')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Web Playground
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('dashboard')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  WardMonitor Dashboard
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('mcp')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Model Context Protocol (MCP)
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('blog')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Research Papers & News
                </button>
              </li>
            </ul>
          </div>

          {/* Project & Legal */}
          <div>
            <h4 className="font-semibold text-[#e6edf3] mb-3 uppercase tracking-wider text-[11px]">
              Community & Specs
            </h4>
            <ul className="space-y-2">
              <li>
                <a
                  href="https://github.com/PhaseMirror/PIRTM"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center gap-1 hover:text-[#58a6ff] transition-colors cursor-pointer"
                >
                  GitHub Repository <ExternalLink className="w-3 h-3 ml-0.5" />
                </a>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('about')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  About & Team
                </button>
              </li>
              <li>
                <button
                  onClick={() => onTabChange('about')}
                  className="hover:text-[#58a6ff] transition-colors text-left cursor-pointer"
                >
                  Prime Materia License
                </button>
              </li>
              <li>
                <a
                  href="https://creativecommons.org/licenses/by-sa/4.0/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center gap-1 hover:text-[#58a6ff] transition-colors cursor-pointer"
                >
                  CC BY-SA 4.0 Specs <ExternalLink className="w-3 h-3 ml-0.5" />
                </a>
              </li>
            </ul>
          </div>
        </div>

        {/* Immersive UI Bottom bar */}
        <div className="pt-8 border-t border-[#30363d] flex flex-col sm:flex-row items-center justify-between gap-4 font-mono text-[11px]">
          <div className="flex flex-wrap items-center gap-4 sm:gap-6">
            <span
              onClick={() => onTabChange('docs', 'adrs')}
              className="hover:text-[#58a6ff] cursor-pointer transition-colors"
            >
              ADR-INDEX: 0x4B3
            </span>
            <span
              onClick={() => onTabChange('about')}
              className="hover:text-[#58a6ff] cursor-pointer transition-colors"
            >
              LICENSE: PRIME-MATERIA
            </span>
            <span
              onClick={() => onTabChange('mcp')}
              className="hover:text-[#58a6ff] cursor-pointer transition-colors"
            >
              MCP_URL: mcp.pirtm.dev
            </span>
          </div>
          <div className="flex flex-wrap items-center gap-4 text-[#8b949e]">
            <div className="flex items-center">
              <span className="w-2 h-2 rounded-full bg-[#3fb950] mr-2 animate-pulse" />
              NODE_01: SYNC
            </div>
            <div>LATENCY: 4ms</div>
            <div className="text-[#e6edf3] font-bold">© 2026 PhaseMirror / PIRTM Foundation</div>
          </div>
        </div>
      </div>
    </footer>
  );
}
