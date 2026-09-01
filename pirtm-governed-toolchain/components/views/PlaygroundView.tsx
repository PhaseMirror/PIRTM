'use client';

import React, { useState, useEffect } from 'react';
import {
  Play,
  RotateCcw,
  Share2,
  Download,
  ShieldCheck,
  ShieldAlert,
  Terminal,
  FileCode,
  ListTree,
  Activity,
  Check,
  Sparkles,
  Sliders,
  AlertTriangle,
  Zap,
  Info
} from 'lucide-react';
import { PLAYGROUND_PRESETS, PlaygroundPreset } from '@/lib/pirtm-data';
import { compileAndRunPirtm, CompilationResult } from '@/lib/compiler-engine';
import { loadPirtmWasm } from '@/lib/wasm-loader';

interface PlaygroundViewProps {
  initialPresetId?: string;
}

export function PlaygroundView({ initialPresetId }: PlaygroundViewProps) {
  const defaultPreset = PLAYGROUND_PRESETS.find(p => p.id === initialPresetId) || PLAYGROUND_PRESETS[0];
  const [selectedPreset, setSelectedPreset] = useState<PlaygroundPreset>(defaultPreset);
  const [sourceCode, setSourceCode] = useState<string>(defaultPreset.code);
  const [activeTab, setActiveTab] = useState<'mlir' | 'audit' | 'logs' | 'ast'>('mlir');
  const [isCompiling, setIsCompiling] = useState(false);
  const [isWasmLoaded, setIsWasmLoaded] = useState(false);
  const [copiedShare, setCopiedShare] = useState(false);
  const [result, setResult] = useState<CompilationResult>(() => compileAndRunPirtm(defaultPreset.code));

  const [prevPresetId, setPrevPresetId] = useState(initialPresetId);

  useEffect(() => {
    loadPirtmWasm().then(() => setIsWasmLoaded(true));
  }, []);

  if (initialPresetId !== prevPresetId) {
    setPrevPresetId(initialPresetId);
    const p = PLAYGROUND_PRESETS.find(item => item.id === initialPresetId) || PLAYGROUND_PRESETS[0];
    setSelectedPreset(p);
    setSourceCode(p.code);
    setResult(compileAndRunPirtm(p.code));
  }

  const handleSelectPreset = (preset: PlaygroundPreset) => {
    setSelectedPreset(preset);
    setSourceCode(preset.code);
    setResult(compileAndRunPirtm(preset.code));
  };

  const handleCompileAndRun = async () => {
    setIsCompiling(true);
    const wasm = await loadPirtmWasm();
    setTimeout(() => {
      const res = wasm.compile(sourceCode);
      setResult(res);
      setIsCompiling(false);
    }, 120);
  };

  const handleSimulateViolation = () => {
    const tripPreset = PLAYGROUND_PRESETS.find(p => p.id === 'ward_violation_demo') || PLAYGROUND_PRESETS[2];
    handleSelectPreset(tripPreset);
  };

  const handleReset = () => {
    setSourceCode(selectedPreset.code);
    setResult(compileAndRunPirtm(selectedPreset.code));
  };

  const handleShare = () => {
    navigator.clipboard.writeText(window.location.href);
    setCopiedShare(true);
    setTimeout(() => setCopiedShare(false), 2000);
  };

  const handleExport = () => {
    const blob = new Blob([activeTab === 'mlir' ? result.mlirCode : sourceCode], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = activeTab === 'mlir' ? 'governed_tensor.mlir' : 'program.pirtm';
    a.click();
    URL.revokeObjectURL(url);
  };

  const lines = sourceCode.split('\n');

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
      {/* Top Controls Toolbar */}
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 mb-4 border-b border-[#233147]">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-cyan-950/80 border border-cyan-800/50 flex items-center justify-center text-cyan-400">
            <Terminal className="w-4 h-4" />
          </div>
          <div>
            <h1 className="text-lg sm:text-xl font-bold text-[#e6edf3]">
              PIRTM Compiler &amp; Sandbox Playground
            </h1>
            <p className="text-xs text-[#8b949e]">
              Real-time MLIR lowering, Lean small-gain proof checks, and WardMonitor execution.
            </p>
          </div>
        </div>

        {/* Action Controls */}
        <div className="flex flex-wrap items-center gap-2">
          {/* Preset Selector */}
          <div className="relative">
            <select
              value={selectedPreset.id}
              onChange={(e) => {
                const found = PLAYGROUND_PRESETS.find(p => p.id === e.target.value);
                if (found) handleSelectPreset(found);
              }}
              className="bg-[#0e1624] text-xs font-mono text-[#e6edf3] border border-[#233147] rounded-lg px-3 py-2 outline-none focus:border-cyan-500/50 cursor-pointer pr-8"
            >
              {PLAYGROUND_PRESETS.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </select>
          </div>

          <button
            onClick={handleSimulateViolation}
            className="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-rose-950/50 hover:bg-rose-900/60 border border-rose-800/50 text-rose-300 text-xs font-semibold transition-colors cursor-pointer"
            title="Simulate WardMonitor Kill-Switch Violation"
          >
            <AlertTriangle className="w-3.5 h-3.5" />
            <span className="hidden sm:inline">Trip Kill-Switch</span>
          </button>

          <button
            onClick={handleReset}
            className="p-2 rounded-lg bg-[#0e1624] hover:bg-[#172336] border border-[#233147] text-[#8b949e] hover:text-[#e6edf3] transition-colors cursor-pointer"
            title="Reset to default code"
          >
            <RotateCcw className="w-4 h-4" />
          </button>

          <button
            onClick={handleShare}
            className="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-[#0e1624] hover:bg-[#172336] border border-[#233147] text-[#8b949e] hover:text-[#e6edf3] text-xs transition-colors cursor-pointer"
          >
            {copiedShare ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Share2 className="w-3.5 h-3.5" />}
            <span className="hidden sm:inline">{copiedShare ? 'Copied' : 'Share'}</span>
          </button>

          <button
            onClick={handleExport}
            className="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-[#0e1624] hover:bg-[#172336] border border-[#233147] text-[#8b949e] hover:text-[#e6edf3] text-xs transition-colors cursor-pointer"
          >
            <Download className="w-3.5 h-3.5" />
            <span className="hidden sm:inline">Export</span>
          </button>

          <button
            onClick={handleCompileAndRun}
            disabled={isCompiling}
            className="flex items-center gap-2 px-5 py-2 rounded-lg bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-white text-xs font-bold shadow-lg shadow-cyan-950/60 transition-all cursor-pointer"
          >
            <Play className={`w-3.5 h-3.5 ${isCompiling ? 'animate-spin' : ''}`} />
            <span>{isCompiling ? 'Verifying...' : 'Compile & Run'}</span>
          </button>
        </div>
      </div>

      {/* Two-Pane Editor Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 min-h-[580px]">
        {/* Left Pane: Code Editor */}
        <div className="lg:col-span-6 rounded-xl border border-[#233147] bg-[#0c121c] flex flex-col overflow-hidden shadow-2xl">
          <div className="px-4 py-2.5 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="w-2.5 h-2.5 rounded-full bg-emerald-500/80" />
              <span className="text-xs font-mono text-[#e6edf3] font-semibold">
                pirtm_editor.pirtm
              </span>
            </div>
            <div className="text-[11px] font-mono text-[#6e7681]">
              {lines.length} lines • UTF-8
            </div>
          </div>

          {/* Interactive Code Editor Area */}
          <div className="relative flex-1 bg-[#0a0f18] p-4 font-mono text-xs overflow-auto flex">
            {/* Line numbers */}
            <div className="select-none pr-4 text-right text-[#485263] border-r border-[#1a2538] mr-3 space-y-1 font-mono text-xs">
              {lines.map((_, i) => (
                <div key={i}>{i + 1}</div>
              ))}
            </div>

            {/* Textarea Code Input */}
            <textarea
              value={sourceCode}
              onChange={(e) => {
                setSourceCode(e.target.value);
              }}
              spellCheck={false}
              className="flex-1 bg-transparent text-[#e6edf3] outline-none resize-none font-mono text-xs leading-relaxed selection:bg-cyan-500/30 whitespace-pre"
              rows={Math.max(16, lines.length)}
            />
          </div>

          {/* Preset Notes */}
          <div className="p-3 bg-[#090d14] border-t border-[#233147] text-[11px] text-[#8b949e] flex items-center gap-2">
            <Info className="w-3.5 h-3.5 text-cyan-400 shrink-0" />
            <span className="line-clamp-1">{selectedPreset.notes}</span>
          </div>
        </div>

        {/* Right Pane: Multi-Tab Output & Diagnostics */}
        <div className="lg:col-span-6 rounded-xl border border-[#233147] bg-[#0c121c] flex flex-col overflow-hidden shadow-2xl">
          {/* Tabs header */}
          <div className="px-4 py-2 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
            <div className="flex items-center gap-1">
              <button
                onClick={() => setActiveTab('mlir')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-mono transition-colors ${
                  activeTab === 'mlir'
                    ? 'bg-[#1b2738] text-cyan-300 border border-cyan-800/40'
                    : 'text-[#8b949e] hover:text-[#e6edf3]'
                }`}
              >
                <FileCode className="w-3.5 h-3.5" />
                <span>MLIR Dialect</span>
              </button>

              <button
                onClick={() => setActiveTab('audit')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-mono transition-colors ${
                  activeTab === 'audit'
                    ? 'bg-[#1b2738] text-emerald-300 border border-emerald-800/40'
                    : 'text-[#8b949e] hover:text-[#e6edf3]'
                }`}
              >
                <ShieldCheck className="w-3.5 h-3.5" />
                <span>Audit Receipt</span>
              </button>

              <button
                onClick={() => setActiveTab('logs')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-mono transition-colors ${
                  activeTab === 'logs'
                    ? 'bg-[#1b2738] text-sky-300 border border-sky-800/40'
                    : 'text-[#8b949e] hover:text-[#e6edf3]'
                }`}
              >
                <Activity className="w-3.5 h-3.5" />
                <span>Compiler Trace</span>
              </button>

              <button
                onClick={() => setActiveTab('ast')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-mono transition-colors ${
                  activeTab === 'ast'
                    ? 'bg-[#1b2738] text-amber-300 border border-amber-800/40'
                    : 'text-[#8b949e] hover:text-[#e6edf3]'
                }`}
              >
                <ListTree className="w-3.5 h-3.5" />
                <span>AST Tree</span>
              </button>
            </div>

            {/* Status indicator badge */}
            <span
              className={`px-2 py-0.5 rounded text-[10px] font-mono font-bold border ${
                result.passed
                  ? 'bg-emerald-950/80 text-emerald-300 border-emerald-800/60'
                  : 'bg-rose-950/80 text-rose-300 border-rose-800/60'
              }`}
            >
              {result.status}
            </span>
          </div>

          {/* Tab Content Body */}
          <div className="flex-1 p-4 bg-[#0a0f18] overflow-auto font-mono text-xs">
            {/* Tab 1: MLIR Output */}
            {activeTab === 'mlir' && (
              <div className="space-y-4">
                <pre className="text-emerald-300/95 leading-relaxed overflow-x-auto">
                  <code>{result.mlirCode}</code>
                </pre>
              </div>
            )}

            {/* Tab 2: Audit Receipt */}
            {activeTab === 'audit' && (
              <div className="space-y-4">
                <div className="p-4 rounded-lg bg-[#0e1624] border border-[#202e42] space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-[#8b949e] text-[11px] font-semibold">GOVERNANCE RECEIPT PROVENANCE</span>
                    <span className={`px-2 py-0.5 rounded text-[10px] font-bold ${
                      result.passed ? 'bg-emerald-950 text-emerald-300 border border-emerald-800/40' : 'bg-rose-950 text-rose-300 border border-rose-800/40'
                    }`}>
                      {result.status}
                    </span>
                  </div>

                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
                    <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c]">
                      <span className="text-[#6e7681] text-[10px] block">Blake3 Proof Receipt Hash</span>
                      <span className="text-cyan-300 font-bold text-[11px] break-all">{result.receiptHash}</span>
                    </div>

                    <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c]">
                      <span className="text-[#6e7681] text-[10px] block">Lean 4 Machine Theorem</span>
                      <span className="text-emerald-400 font-bold text-[11px]">{result.leanTheoremUsed}</span>
                    </div>

                    <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c]">
                      <span className="text-[#6e7681] text-[10px] block">Effective Spectral Radius (ρ)</span>
                      <span className={`font-bold text-sm ${result.spectralRadius < 1.0 ? 'text-emerald-400' : 'text-rose-400'}`}>
                        {result.spectralRadius.toFixed(3)} {result.spectralRadius < 1.0 ? '✅ (Nominal)' : '❌ (Breach)'}
                      </span>
                    </div>

                    <div className="p-2.5 rounded bg-[#090d14] border border-[#1e2a3c]">
                      <span className="text-[#6e7681] text-[10px] block">Lyapunov Energy Level V(x)</span>
                      <span className="text-sky-300 font-bold text-sm">{result.lyapunovEnergy.toFixed(4)}</span>
                    </div>
                  </div>
                </div>

                {/* Execution output box */}
                <div className="p-3 bg-[#090d14] rounded-lg border border-[#1e2a3c]">
                  <span className="text-[#6e7681] text-[10px] block mb-1">Sandbox Execution Log</span>
                  <pre className="text-[#e6edf3] whitespace-pre-wrap leading-relaxed">
                    {result.executionOutput}
                  </pre>
                </div>
              </div>
            )}

            {/* Tab 3: Compiler Trace Logs */}
            {activeTab === 'logs' && (
              <div className="space-y-2">
                {result.logs.map((log, lIdx) => (
                  <div key={lIdx} className="text-[11px] leading-relaxed text-[#8b949e]">
                    {log.includes('✅') || log.includes('PROVEN') ? (
                      <span className="text-emerald-400">{log}</span>
                    ) : log.includes('❌') || log.includes('FAILED') || log.includes('Kill-switch') ? (
                      <span className="text-rose-400 font-semibold">{log}</span>
                    ) : log.includes('⚠️') ? (
                      <span className="text-amber-400">{log}</span>
                    ) : log.includes('LEAN-GATE') ? (
                      <span className="text-cyan-300">{log}</span>
                    ) : (
                      <span>{log}</span>
                    )}
                  </div>
                ))}
              </div>
            )}

            {/* Tab 4: AST Tree */}
            {activeTab === 'ast' && (
              <div className="space-y-3">
                <div className="p-3 rounded-lg bg-[#090d14] border border-[#1e2a3c]">
                  <div className="text-cyan-300 font-bold mb-2">AST Node Hierarchy</div>
                  {result.astTree.map((rootNode) => (
                    <div key={rootNode.id} className="space-y-2">
                      <div className="font-semibold text-emerald-400">{rootNode.label}</div>
                      <div className="pl-4 border-l border-[#202e42] space-y-2">
                        {rootNode.children?.map((c) => (
                          <div key={c.id} className="space-y-1">
                            <div className="text-sky-300">{c.label}</div>
                            {c.children && (
                              <div className="pl-4 border-l border-[#202e42] space-y-1">
                                {c.children.map((sub) => (
                                  <div key={sub.id} className="text-xs text-[#94a3b8]">
                                    • <span className="text-[#e6edf3]">{sub.label}</span>
                                    {sub.details && <span className="text-[#6e7681] ml-2">({sub.details})</span>}
                                  </div>
                                ))}
                              </div>
                            )}
                          </div>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Bottom Diagnostics Footer */}
          <div className="p-3 bg-[#090d14] border-t border-[#233147] flex flex-wrap items-center justify-between gap-3 text-[11px] font-mono text-[#8b949e]">
            <div className="flex items-center gap-3">
              <span className="flex items-center gap-1.5">
                <span className={`w-2 h-2 rounded-full ${result.passed ? 'bg-emerald-400' : 'bg-rose-500'}`} />
                <span className="font-bold text-[#e6edf3]">
                  Governance Gate: {result.passed ? 'PASS' : 'HALTED'}
                </span>
              </span>
              <span>•</span>
              <span>Spectral ρ: <strong className={result.spectralRadius < 1 ? 'text-emerald-400' : 'text-rose-400'}>{result.spectralRadius.toFixed(3)}</strong></span>
            </div>

            <div className="flex items-center gap-4">
              <span>Comp: {result.compilationTimeMs}ms</span>
              <span>•</span>
              <span>Exec: {result.executionTimeUs}µs</span>
              <span>•</span>
              <span className="text-emerald-400">0 Leaks</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
