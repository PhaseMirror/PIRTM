'use client';

import React, { useState } from 'react';
import {
  BookOpen,
  FileText,
  ShieldCheck,
  GraduationCap,
  Search,
  CheckCircle2,
  ExternalLink,
  Code2,
  ChevronRight,
  Copy,
  Check,
  X,
  Layers,
  ArrowUpRight,
  Info
} from 'lucide-react';
import { ADR_LIST, LEAN_THEOREMS, STDLIB_FUNCTIONS, TUTORIALS, ADRItem, LeanTheorem, TutorialItem } from '@/lib/pirtm-data';

interface DocsViewProps {
  initialSubSection?: string;
  onNavigateToPlayground?: (codePresetId?: string) => void;
}

export function DocsView({ initialSubSection = 'language', onNavigateToPlayground }: DocsViewProps) {
  const [activeSection, setActiveSection] = useState<'language' | 'adrs' | 'proofs' | 'tutorials'>(
    (initialSubSection as any) || 'language'
  );
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedAdr, setSelectedAdr] = useState<ADRItem | null>(null);
  const [selectedTheorem, setSelectedTheorem] = useState<LeanTheorem | null>(LEAN_THEOREMS[0]);
  const [selectedTutorial, setSelectedTutorial] = useState<TutorialItem>(TUTORIALS[0]);
  const [copiedTheoremId, setCopiedTheoremId] = useState<string | null>(null);

  const sidebarItems = [
    { id: 'language', label: 'Language Reference', icon: BookOpen, count: `${STDLIB_FUNCTIONS.length} APIs` },
    { id: 'adrs', label: 'ADR Index (Decisions)', icon: FileText, count: `${ADR_LIST.length} ADRs` },
    { id: 'proofs', label: 'Lean 4 Proofs Ledger', icon: ShieldCheck, count: 'Zero-Sorry' },
    { id: 'tutorials', label: 'Tutorials & Guides', icon: GraduationCap, count: `${TUTORIALS.length} Guides` },
  ];

  const handleCopyProof = (thm: LeanTheorem) => {
    navigator.clipboard.writeText(thm.proofSnippet);
    setCopiedTheoremId(thm.id);
    setTimeout(() => setCopiedTheoremId(null), 2000);
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Top Header & Section Switcher on Mobile */}
      <div className="flex flex-col md:flex-row md:items-center justify-between pb-6 mb-6 border-b border-[#233147] gap-4">
        <div>
          <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
            <BookOpen className="w-4 h-4" />
            <span>PIRTM FORMAL SPECIFICATION</span>
          </div>
          <h1 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
            Documentation &amp; Verification Ledger
          </h1>
          <p className="text-xs sm:text-sm text-[#8b949e] mt-1">
            Grammar reference, Lean 4 machine proofs, and Architecture Decision Records (ADRs).
          </p>
        </div>

        {/* Global Search within Docs */}
        <div className="relative w-full md:w-72">
          <Search className="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-[#6e7681]" />
          <input
            type="text"
            placeholder="Filter documentation..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-9 pr-4 py-2 bg-[#0e1624] border border-[#233147] rounded-lg text-xs text-[#e6edf3] placeholder-[#6e7681] outline-none focus:border-cyan-500/50"
          />
        </div>
      </div>

      {/* Main Grid: Sidebar + Content */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Left Sidebar */}
        <div className="lg:col-span-3 space-y-4">
          <div className="p-3 bg-[#0e1624] border border-[#233147] rounded-xl">
            <span className="text-[11px] font-mono text-[#8b949e] uppercase px-2 py-1 block">
              Sections
            </span>
            <div className="space-y-1 mt-1">
              {sidebarItems.map((item) => {
                const Icon = item.icon;
                const isActive = activeSection === item.id;
                return (
                  <button
                    key={item.id}
                    onClick={() => setActiveSection(item.id as any)}
                    className={`w-full flex items-center justify-between px-3 py-2.5 rounded-lg text-xs font-medium transition-all text-left cursor-pointer ${
                      isActive
                        ? 'bg-[#1a2638] text-cyan-300 border border-cyan-500/30 shadow-sm'
                        : 'text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#131d2c]'
                    }`}
                  >
                    <div className="flex items-center gap-2.5">
                      <Icon className={`w-4 h-4 ${isActive ? 'text-cyan-400' : 'text-[#8b949e]'}`} />
                      <span>{item.label}</span>
                    </div>
                    <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-[#090d14] text-[#6e7681] border border-[#1e2a3c]">
                      {item.count}
                    </span>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Microkernel Status Card */}
          <div className="p-4 bg-gradient-to-br from-[#0c1320] to-[#090d14] border border-[#233147] rounded-xl text-xs space-y-2">
            <div className="flex items-center justify-between text-emerald-400 font-mono text-[11px]">
              <span className="flex items-center gap-1.5">
                <CheckCircle2 className="w-3.5 h-3.5" />
                lake build: passing
              </span>
              <span className="text-[10px] text-[#8b949e]">Lean v4.8.0</span>
            </div>
            <p className="text-[#8b949e] text-[11px] leading-relaxed">
              Self-contained Lean 4 algebraic microkernel with 0 mathlib axioms and 0 unresolved sorries.
            </p>
          </div>
        </div>

        {/* Right Content View */}
        <div className="lg:col-span-9">
          {/* 1. Language Reference Section */}
          {activeSection === 'language' && (
            <div className="space-y-8">
              {/* Grammar & Type System Overview */}
              <div className="p-6 bg-[#0e1624] border border-[#233147] rounded-xl">
                <h2 className="text-lg font-bold text-[#e6edf3] mb-2 flex items-center gap-2">
                  <Code2 className="w-5 h-5 text-cyan-400" />
                  PIRTM Grammar &amp; Phase Type System
                </h2>
                <p className="text-xs text-[#8b949e] leading-relaxed mb-4">
                  The PIRTM type system equips multi-dimensional tensors with static phase parameters and multiplicity algebra indices. Contractions and feedback loops are bounded by the continuous small-gain condition.
                </p>

                <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 font-mono text-xs">
                  <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c]">
                    <span className="text-cyan-400 font-bold block mb-1">PhaseTensor&lt;Dim, Phase&gt;</span>
                    <span className="text-[#8b949e] text-[11px]">Orthogonal tensor carrying invariant phase tag.</span>
                  </div>
                  <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c]">
                    <span className="text-sky-400 font-bold block mb-1">MultiplicityOp&lt;K&gt;</span>
                    <span className="text-[#8b949e] text-[11px]">Multiplicity ring convolution operator Ap(k).</span>
                  </div>
                  <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c]">
                    <span className="text-emerald-400 font-bold block mb-1">SpectralGate&lt;ρ&gt;</span>
                    <span className="text-[#8b949e] text-[11px]">Bounded contractivity assertion (ρ &lt; 1.0).</span>
                  </div>
                </div>
              </div>

              {/* Standard Library Table */}
              <div>
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-base font-bold text-[#e6edf3]">
                    Standard Library Functions
                  </h3>
                  <span className="text-xs text-[#8b949e] font-mono">
                    Showing {STDLIB_FUNCTIONS.length} Core APIs
                  </span>
                </div>

                <div className="divide-y divide-[#202e42] border border-[#233147] rounded-xl bg-[#0e1624] overflow-hidden">
                  {STDLIB_FUNCTIONS.filter((fn) =>
                    fn.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                    fn.description.toLowerCase().includes(searchQuery.toLowerCase())
                  ).map((fn, idx) => (
                    <div key={idx} className="p-5 hover:bg-[#121c2d] transition-colors space-y-3">
                      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                        <div className="flex items-center gap-2.5">
                          <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-[#1b2738] text-cyan-300 border border-cyan-800/40">
                            {fn.category}
                          </span>
                          <span className="font-mono text-sm font-bold text-[#e6edf3]">
                            {fn.name}
                          </span>
                        </div>
                        <div className="flex items-center gap-2 text-xs font-mono">
                          <span className="text-emerald-400 flex items-center gap-1">
                            <ShieldCheck className="w-3.5 h-3.5" />
                            {fn.leanTheorem}
                          </span>
                          <span className="text-[#6e7681]">•</span>
                          <span className="text-sky-400">{fn.adrRef}</span>
                        </div>
                      </div>

                      <div className="p-2.5 bg-[#090d14] rounded-lg border border-[#1e2a3c] font-mono text-xs text-[#88c0d0] overflow-x-auto">
                        <code>{fn.signature}</code>
                      </div>

                      <p className="text-xs text-[#8b949e]">
                        {fn.description}
                      </p>

                      <div className="text-[11px] font-mono text-[#a3be8c] bg-[#0b1019] p-2 rounded border border-[#1a2538] overflow-x-auto">
                        <span className="text-[#6e7681]">Example: </span>
                        <code>{fn.example}</code>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {/* 2. ADR Index Section */}
          {activeSection === 'adrs' && (
            <div className="space-y-6">
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                <div>
                  <h2 className="text-lg font-bold text-[#e6edf3]">
                    Architecture Decision Records (ADR Index)
                  </h2>
                  <p className="text-xs text-[#8b949e]">
                    Formal log of architectural, mathematical, and cryptographic decisions shaping PIRTM.
                  </p>
                </div>
              </div>

              {/* Table */}
              <div className="border border-[#233147] rounded-xl bg-[#0e1624] overflow-x-auto">
                <table className="w-full text-left text-xs">
                  <thead className="bg-[#090d14] text-[#8b949e] border-b border-[#233147] font-mono text-[11px]">
                    <tr>
                      <th className="py-3 px-4">ADR ID</th>
                      <th className="py-3 px-4">Title &amp; Summary</th>
                      <th className="py-3 px-4">Status</th>
                      <th className="py-3 px-4">Date</th>
                      <th className="py-3 px-4 text-right">Action</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-[#1e2c40]">
                    {ADR_LIST.filter((adr) =>
                      adr.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
                      adr.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
                      adr.summary.toLowerCase().includes(searchQuery.toLowerCase())
                    ).map((adr) => (
                      <tr key={adr.id} className="hover:bg-[#121c2d] transition-colors">
                        <td className="py-3.5 px-4 font-mono font-bold text-cyan-400 whitespace-nowrap">
                          {adr.id}
                        </td>
                        <td className="py-3.5 px-4 max-w-md">
                          <div className="font-semibold text-[#e6edf3] mb-0.5">{adr.title}</div>
                          <div className="text-[#8b949e] line-clamp-1">{adr.summary}</div>
                        </td>
                        <td className="py-3.5 px-4 whitespace-nowrap">
                          <span className={`px-2 py-0.5 rounded text-[10px] font-mono font-bold border ${
                            adr.status === 'Accepted'
                              ? 'bg-emerald-950/80 text-emerald-300 border-emerald-800/60'
                              : adr.status === 'Implemented'
                              ? 'bg-cyan-950/80 text-cyan-300 border-cyan-800/60'
                              : 'bg-amber-950/80 text-amber-300 border-amber-800/60'
                          }`}>
                            {adr.status}
                          </span>
                        </td>
                        <td className="py-3.5 px-4 font-mono text-[#8b949e] whitespace-nowrap">
                          {adr.date}
                        </td>
                        <td className="py-3.5 px-4 text-right whitespace-nowrap">
                          <button
                            onClick={() => setSelectedAdr(adr)}
                            className="px-2.5 py-1 rounded bg-[#1b2738] hover:bg-[#25364e] text-cyan-300 text-[11px] font-medium border border-[#2d3f58] transition-colors cursor-pointer"
                          >
                            Inspect ADR
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* 3. Lean 4 Proofs Ledger Section */}
          {activeSection === 'proofs' && (
            <div className="space-y-6">
              <div className="p-6 bg-[#0e1624] border border-[#233147] rounded-xl flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                <div>
                  <div className="flex items-center gap-2 text-xs font-mono text-emerald-400 mb-1">
                    <ShieldCheck className="w-4 h-4" />
                    <span>ZERO-MATHLIB MACHINE-CHECKED AXIOMS</span>
                  </div>
                  <h2 className="text-xl font-bold text-[#e6edf3]">
                    Lean 4 Microkernel Proof Ledger
                  </h2>
                  <p className="text-xs text-[#8b949e] mt-1">
                    All matrix contractivity theorems are formally proved with 0 sorry markers and verified in CI.
                  </p>
                </div>
                <div className="flex items-center gap-3">
                  <div className="px-3 py-1.5 rounded-lg bg-emerald-950/60 border border-emerald-800/60 text-emerald-300 font-mono text-xs">
                    0 Unresolved Axioms
                  </div>
                </div>
              </div>

              {/* Theorem Selector & Code Viewer */}
              <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
                {/* Theorem List */}
                <div className="lg:col-span-5 space-y-2">
                  {LEAN_THEOREMS.map((thm) => {
                    const isSelected = selectedTheorem?.id === thm.id;
                    return (
                      <button
                        key={thm.id}
                        onClick={() => setSelectedTheorem(thm)}
                        className={`w-full p-3.5 rounded-xl border text-left transition-all cursor-pointer ${
                          isSelected
                            ? 'bg-[#1a2638] border-cyan-500/50 shadow-md shadow-cyan-950/20'
                            : 'bg-[#0e1624] border-[#233147] hover:bg-[#121d2e]'
                        }`}
                      >
                        <div className="flex items-center justify-between text-xs mb-1">
                          <span className="font-mono text-cyan-400 font-semibold">{thm.name}</span>
                          <span className="text-[10px] font-mono text-emerald-400 flex items-center gap-1">
                            <CheckCircle2 className="w-3 h-3" />
                            {thm.sorryCount} sorry
                          </span>
                        </div>
                        <div className="text-[11px] font-mono text-[#8b949e] mb-1">{thm.module}</div>
                        <p className="text-xs text-[#94a3b8] line-clamp-2">{thm.doc}</p>
                      </button>
                    );
                  })}
                </div>

                {/* Proof Code Display */}
                <div className="lg:col-span-7">
                  {selectedTheorem && (
                    <div className="rounded-xl border border-[#233147] bg-[#0c121c] overflow-hidden flex flex-col h-full shadow-2xl">
                      <div className="px-4 py-3 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
                        <div>
                          <span className="text-xs font-mono text-[#e6edf3] font-bold block">
                            {selectedTheorem.name}
                          </span>
                          <span className="text-[10px] font-mono text-[#6e7681]">
                            Module: {selectedTheorem.module}
                          </span>
                        </div>
                        <button
                          onClick={() => handleCopyProof(selectedTheorem)}
                          className="flex items-center gap-1 px-2.5 py-1 rounded bg-[#172336] hover:bg-[#1f2f48] border border-[#273852] text-xs text-[#e6edf3] transition-colors cursor-pointer"
                        >
                          {copiedTheoremId === selectedTheorem.id ? (
                            <>
                              <Check className="w-3.5 h-3.5 text-emerald-400" />
                              <span className="text-emerald-400">Copied</span>
                            </>
                          ) : (
                            <>
                              <Copy className="w-3.5 h-3.5 text-[#8b949e]" />
                              <span>Copy Proof</span>
                            </>
                          )}
                        </button>
                      </div>

                      {/* Lean code display */}
                      <div className="p-4 font-mono text-xs text-[#e6edf3] overflow-x-auto leading-relaxed bg-[#090d14]/60">
                        <pre className="text-cyan-200">
                          <code>{selectedTheorem.proofSnippet}</code>
                        </pre>
                      </div>

                      <div className="mt-auto p-4 bg-[#0a0f18] border-t border-[#1e2a3c] text-xs text-[#8b949e] space-y-1">
                        <div className="font-semibold text-[#cbd5e1]">Formal Verification Context:</div>
                        <p>{selectedTheorem.doc}</p>
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* 4. Tutorials Section */}
          {activeSection === 'tutorials' && (
            <div className="space-y-6">
              <div className="flex flex-wrap gap-2 mb-4">
                {TUTORIALS.map((tut) => (
                  <button
                    key={tut.id}
                    onClick={() => setSelectedTutorial(tut)}
                    className={`px-4 py-2 rounded-lg text-xs font-semibold transition-all cursor-pointer ${
                      selectedTutorial.id === tut.id
                        ? 'bg-[#1b2738] text-cyan-300 border border-cyan-500/40'
                        : 'bg-[#0e1624] text-[#8b949e] border border-[#233147] hover:text-[#e6edf3]'
                    }`}
                  >
                    {tut.title}
                  </button>
                ))}
              </div>

              {/* Active Tutorial Content */}
              <div className="p-6 bg-[#0e1624] border border-[#233147] rounded-xl space-y-6">
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 pb-4 border-b border-[#1e2c40]">
                  <div>
                    <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
                      <span>{selectedTutorial.level}</span>
                      <span>•</span>
                      <span>{selectedTutorial.duration}</span>
                    </div>
                    <h2 className="text-xl font-bold text-[#e6edf3]">{selectedTutorial.title}</h2>
                  </div>
                  {onNavigateToPlayground && (
                    <button
                      onClick={() => onNavigateToPlayground('contractive_loop')}
                      className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white text-xs font-semibold transition-all cursor-pointer self-start sm:self-auto"
                    >
                      <Code2 className="w-3.5 h-3.5" />
                      <span>Open in Playground</span>
                    </button>
                  )}
                </div>

                <p className="text-xs sm:text-sm text-[#8b949e]">
                  {selectedTutorial.summary}
                </p>

                {/* Steps */}
                <div className="space-y-6">
                  {selectedTutorial.steps.map((step, sIdx) => (
                    <div key={sIdx} className="space-y-3">
                      <h3 className="text-sm font-bold text-[#e6edf3]">{step.heading}</h3>
                      <p className="text-xs text-[#8b949e]">{step.explanation}</p>
                      <div className="p-4 bg-[#090d14] border border-[#1e2a3c] rounded-lg font-mono text-xs text-[#88c0d0] overflow-x-auto">
                        <pre>
                          <code>{step.code}</code>
                        </pre>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* ADR Detailed Modal Sheet */}
      {selectedAdr && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm animate-in fade-in duration-150">
          <div className="w-full max-w-2xl bg-[#0e1624] border border-[#2d3f58] rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[85vh]">
            <div className="px-6 py-4 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
              <div>
                <span className="text-xs font-mono font-bold text-cyan-400">{selectedAdr.id}</span>
                <h3 className="text-base font-bold text-[#e6edf3] mt-0.5">{selectedAdr.title}</h3>
              </div>
              <button
                onClick={() => setSelectedAdr(null)}
                className="p-1.5 rounded-lg text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#1b283d] transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            <div className="p-6 overflow-y-auto space-y-4 text-xs">
              <div className="flex items-center gap-4 text-[#8b949e] font-mono text-[11px]">
                <span>Author: {selectedAdr.author}</span>
                <span>•</span>
                <span>Date: {selectedAdr.date}</span>
                <span>•</span>
                <span className="text-emerald-400 font-semibold">{selectedAdr.status}</span>
              </div>

              <div>
                <h4 className="font-bold text-[#e6edf3] mb-1 uppercase tracking-wider text-[11px]">Context</h4>
                <p className="text-[#8b949e] leading-relaxed bg-[#090d14] p-3 rounded-lg border border-[#1e2a3c]">
                  {selectedAdr.context}
                </p>
              </div>

              <div>
                <h4 className="font-bold text-[#e6edf3] mb-1 uppercase tracking-wider text-[11px]">Decision</h4>
                <p className="text-[#8b949e] leading-relaxed bg-[#090d14] p-3 rounded-lg border border-[#1e2a3c]">
                  {selectedAdr.decision}
                </p>
              </div>

              <div>
                <h4 className="font-bold text-[#e6edf3] mb-1 uppercase tracking-wider text-[11px]">Consequences</h4>
                <ul className="list-disc pl-5 space-y-1 text-[#8b949e]">
                  {selectedAdr.consequences.map((c, i) => (
                    <li key={i}>{c}</li>
                  ))}
                </ul>
              </div>

              {selectedAdr.leanTheoremRef && (
                <div className="pt-2 border-t border-[#1e2c40] flex items-center justify-between text-[#8b949e]">
                  <span>Lean Theorem: <code className="text-cyan-300 font-mono">{selectedAdr.leanTheoremRef}</code></span>
                </div>
              )}
            </div>

            <div className="px-6 py-3 bg-[#090d14] border-t border-[#233147] flex justify-end">
              <button
                onClick={() => setSelectedAdr(null)}
                className="px-4 py-1.5 rounded-lg bg-[#1b2738] hover:bg-[#25364e] text-xs font-semibold text-[#e6edf3] transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
