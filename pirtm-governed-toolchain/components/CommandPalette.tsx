'use client';

import React, { useState, useEffect, useMemo } from 'react';
import { Search, FileText, Code2, ShieldCheck, Cpu, BookOpen, X, ArrowRight } from 'lucide-react';
import { ADR_LIST, LEAN_THEOREMS, STDLIB_FUNCTIONS, TUTORIALS, MCP_TOOLS } from '@/lib/pirtm-data';

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  onNavigate: (tab: string, subSection?: string) => void;
}

interface SearchResultItem {
  type: string;
  title: string;
  subtitle?: string;
  icon: React.ComponentType<{ className?: string }>;
  action: () => void;
}

export function CommandPalette({ isOpen, onClose, onNavigate }: CommandPaletteProps) {
  const [query, setQuery] = useState('');

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        if (isOpen) onClose();
        else setQuery('');
      }
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  const searchResults: SearchResultItem[] = useMemo(() => {
    if (!query.trim()) {
      return [
        { type: 'Quick Link', title: 'Interactive Playground', icon: Code2, action: () => onNavigate('playground') },
        { type: 'Quick Link', title: 'Governance & WardMonitor Dashboard', icon: ShieldCheck, action: () => onNavigate('dashboard') },
        { type: 'Quick Link', title: 'Model Context Protocol (MCP) Server', icon: Cpu, action: () => onNavigate('mcp') },
        { type: 'Quick Link', title: 'Documentation & Language Reference', icon: BookOpen, action: () => onNavigate('docs', 'language') },
        { type: 'Quick Link', title: 'Lean 4 Proofs Ledger (Zero-Sorry)', icon: ShieldCheck, action: () => onNavigate('docs', 'proofs') },
        { type: 'Quick Link', title: 'Architecture Decision Records (ADR Index)', icon: FileText, action: () => onNavigate('docs', 'adrs') }
      ];
    }

    const q = query.toLowerCase();
    const results: SearchResultItem[] = [];

    // Search ADRs
    for (const adr of ADR_LIST) {
      if (adr.id.toLowerCase().includes(q) || adr.title.toLowerCase().includes(q) || adr.summary.toLowerCase().includes(q)) {
        results.push({
          type: 'ADR',
          title: `${adr.id}: ${adr.title}`,
          subtitle: adr.summary,
          icon: FileText,
          action: () => onNavigate('docs', 'adrs')
        });
      }
    }

    // Search Lean Theorems
    for (const thm of LEAN_THEOREMS) {
      if (thm.name.toLowerCase().includes(q) || thm.doc.toLowerCase().includes(q)) {
        results.push({
          type: 'Lean Theorem',
          title: thm.name,
          subtitle: thm.doc,
          icon: ShieldCheck,
          action: () => onNavigate('docs', 'proofs')
        });
      }
    }

    // Search Standard Library
    for (const fn of STDLIB_FUNCTIONS) {
      if (fn.name.toLowerCase().includes(q) || fn.description.toLowerCase().includes(q)) {
        results.push({
          type: 'StdLib',
          title: fn.name,
          subtitle: fn.description,
          icon: Code2,
          action: () => onNavigate('docs', 'language')
        });
      }
    }

    // Search MCP Tools
    for (const tool of MCP_TOOLS) {
      if (tool.name.toLowerCase().includes(q) || tool.description.toLowerCase().includes(q)) {
        results.push({
          type: 'MCP Tool',
          title: `MCP: ${tool.name}`,
          subtitle: tool.description,
          icon: Cpu,
          action: () => onNavigate('mcp')
        });
      }
    }

    // Search Tutorials
    for (const tut of TUTORIALS) {
      if (tut.title.toLowerCase().includes(q) || tut.summary.toLowerCase().includes(q)) {
        results.push({
          type: 'Tutorial',
          title: tut.title,
          subtitle: tut.summary,
          icon: BookOpen,
          action: () => onNavigate('docs', 'tutorials')
        });
      }
    }

    return results.slice(0, 8);
  }, [query, onNavigate]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4 bg-black/75 backdrop-blur-sm animate-in fade-in duration-150">
      <div className="w-full max-w-2xl bg-[#0f1724] border border-[#233147] rounded-xl shadow-2xl overflow-hidden flex flex-col">
        {/* Search Input */}
        <div className="flex items-center px-4 py-3.5 border-b border-[#233147] gap-3 bg-[#0a0f18]">
          <Search className="w-5 h-5 text-[#58a6ff]" />
          <input
            type="text"
            placeholder="Search ADRs, Lean theorems, stdlib, MCP tools, tutorials (or press ESC to close)..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            autoFocus
            className="flex-1 bg-transparent text-sm text-[#e6edf3] placeholder-[#6e7681] outline-none font-sans"
          />
          <button
            onClick={onClose}
            className="p-1 rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#1f293d]"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Results List */}
        <div className="max-h-[380px] overflow-y-auto p-2 divide-y divide-[#1e293b]/50">
          {searchResults.length === 0 ? (
            <div className="p-8 text-center text-sm text-[#8b949e]">
              No matching specifications, theorems, or tools found for &quot;{query}&quot;.
            </div>
          ) : (
            searchResults.map((res, i) => {
              const Icon = res.icon;
              return (
                <button
                  key={i}
                  onClick={() => {
                    res.action();
                    onClose();
                  }}
                  className="w-full flex items-center justify-between p-3 rounded-lg text-left hover:bg-[#172336] transition-colors group cursor-pointer"
                >
                  <div className="flex items-start gap-3 min-w-0">
                    <div className="p-2 rounded-md bg-[#0a0f18] border border-[#233147] text-[#58a6ff] shrink-0 mt-0.5">
                      <Icon className="w-4 h-4" />
                    </div>
                    <div className="min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-xs font-mono px-1.5 py-0.5 rounded bg-[#1e293b] text-[#8b949e] border border-[#334155]">
                          {res.type}
                        </span>
                        <span className="text-sm font-semibold text-[#e6edf3] truncate group-hover:text-[#58a6ff]">
                          {res.title}
                        </span>
                      </div>
                      {res.subtitle && (
                        <p className="text-xs text-[#8b949e] mt-1 line-clamp-1">
                          {res.subtitle}
                        </p>
                      )}
                    </div>
                  </div>
                  <ArrowRight className="w-4 h-4 text-[#8b949e] group-hover:text-[#58a6ff] group-hover:translate-x-0.5 transition-all shrink-0 ml-2" />
                </button>
              );
            })
          )}
        </div>

        {/* Footer */}
        <div className="px-4 py-2 bg-[#0a0f18] border-t border-[#233147] flex items-center justify-between text-xs text-[#8b949e]">
          <span>Navigate with mouse or Tab</span>
          <div className="flex items-center gap-2">
            <span className="px-1.5 py-0.5 rounded bg-[#1e293b] border border-[#334155] font-mono text-[10px]">ESC</span>
            <span>to dismiss</span>
          </div>
        </div>
      </div>
    </div>
  );
}
