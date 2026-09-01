'use client';

import React, { useState, useEffect, useRef } from 'react';
import {
  Menu,
  X,
  Compass,
  Search,
  Github,
  Terminal,
  BookOpen,
  Activity,
  Cpu,
  Newspaper,
  Info,
  ChevronRight,
  ShieldCheck,
  ExternalLink,
} from 'lucide-react';

interface NavbarProps {
  currentTab: string;
  onTabChange: (tab: string) => void;
  onOpenSearch: () => void;
}

export function Navbar({ currentTab, onTabChange, onOpenSearch }: NavbarProps) {
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);
  const buttonRef = useRef<HTMLButtonElement>(null);

  const navItems = [
    {
      id: 'home',
      label: 'Home',
      description: 'System overview, Lyapunov stability, architecture',
      icon: Compass,
      badge: 'Main',
    },
    {
      id: 'docs',
      label: 'Documentation',
      description: 'Language specs, EBNF grammar, Lean 4 proofs & ADRs',
      icon: BookOpen,
      badge: 'Proofs',
    },
    {
      id: 'playground',
      label: 'Playground',
      description: 'Interactive IDE, MLIR compiler, AST visualizer',
      icon: Terminal,
      badge: 'REPL',
    },
    {
      id: 'dashboard',
      label: 'Dashboard',
      description: 'WardMonitor runtime, live telemetry, spectral gates',
      icon: Activity,
      badge: 'Real-time',
    },
    {
      id: 'mcp',
      label: 'MCP Server',
      description: 'Model Context Protocol tools & JSON-RPC bridge',
      icon: Cpu,
      badge: 'AI Bridge',
    },
    {
      id: 'blog',
      label: 'Blog',
      description: 'Research papers, release notes & theory updates',
      icon: Newspaper,
      badge: 'Research',
    },
    {
      id: 'about',
      label: 'About',
      description: 'The PIRTM / MOC foundation, license & community',
      icon: Info,
      badge: 'License',
    },
  ];

  // Close when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        menuRef.current &&
        !menuRef.current.contains(event.target as Node) &&
        buttonRef.current &&
        !buttonRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    }

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen]);

  // Close on Escape key
  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.key === 'Escape' && isOpen) {
        setIsOpen(false);
      }
    }

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen]);

  const handleItemClick = (id: string) => {
    onTabChange(id);
    setIsOpen(false);
  };

  return (
    <header className="sticky top-0 z-40 w-full border-b border-[#30363d] bg-[#0d1117]/95 backdrop-blur-md">
      <div className="max-w-7xl mx-auto px-4 sm:px-8 h-16 flex items-center justify-between gap-4">
        {/* Left: Brand / Logo */}
        <div className="flex items-center space-x-3">
          <button
            onClick={() => handleItemClick('home')}
            className="flex items-center space-x-2.5 text-left group focus:outline-none cursor-pointer"
            id="brand-logo-button"
            aria-label="PIRTM Home"
          >
            <div className="w-8 h-8 bg-[#58a6ff] rounded flex items-center justify-center font-bold text-black text-xs italic shadow-sm group-hover:bg-[#79b8ff] transition-colors">
              P
            </div>
            <div className="flex items-baseline space-x-2">
              <span className="text-lg font-bold tracking-tight text-[#e6edf3] group-hover:text-white transition-colors">
                PIRTM
              </span>
              <span className="font-mono font-normal opacity-50 text-xs text-[#8b949e]">
                v1.0.4-rc
              </span>
            </div>
          </button>
        </div>

        {/* Right: Telemetry + Search + GitHub + Hamburger Menu (Far Right) */}
        <div className="flex items-center space-x-2.5 sm:space-x-3">
          {/* Status Indicator Pill */}
          <div className="hidden sm:flex items-center px-3 py-1 bg-[#238636]/10 border border-[#238636]/30 text-[#3fb950] text-[10px] uppercase tracking-widest font-bold rounded-full">
            <span className="w-1.5 h-1.5 rounded-full bg-[#3fb950] mr-1.5 animate-pulse" />
            System Online
          </div>

          {/* Search Trigger */}
          <button
            onClick={onOpenSearch}
            className="flex items-center gap-2 px-3 py-1.5 rounded-md bg-[#161b22] hover:bg-[#21262d] border border-[#30363d] text-xs text-[#8b949e] hover:text-[#e6edf3] transition-colors shadow-sm cursor-pointer"
            id="search-open-button"
            title="Search documentation and commands"
          >
            <Search className="w-3.5 h-3.5 text-[#58a6ff]" />
            <span className="hidden md:inline">Search...</span>
            <kbd className="hidden sm:inline-block font-mono text-[10px] px-1.5 py-0.5 bg-[#080a0d] border border-[#30363d] rounded text-[#8b949e]">
              ⌘K
            </kbd>
          </button>

          {/* Quick Playground CTA */}
          <button
            onClick={() => handleItemClick('playground')}
            className="hidden lg:flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-[#238636] hover:bg-[#2ea043] text-white text-xs font-bold shadow-md shadow-[#238636]/20 transition-all cursor-pointer"
            id="quick-playground-button"
          >
            <Terminal className="w-3.5 h-3.5" />
            <span>Playground</span>
          </button>

          {/* GitHub External */}
          <a
            href="https://github.com/PhaseMirror/PIRTM"
            target="_blank"
            rel="noopener noreferrer"
            className="w-8 h-8 rounded-full bg-[#30363d] hover:bg-[#3d444d] flex items-center justify-center text-white transition-colors cursor-pointer"
            title="View PIRTM on GitHub"
            id="github-link-button"
          >
            <Github className="w-4 h-4 fill-current" />
          </a>

          {/* Hamburger Menu Toggle Button (Far Right) */}
          <div className="relative">
            <button
              ref={buttonRef}
              onClick={() => setIsOpen(!isOpen)}
              className={`p-2 rounded-md border transition-all cursor-pointer flex items-center justify-center ${
                isOpen
                  ? 'bg-[#161b22] border-[#58a6ff] text-[#58a6ff] shadow-sm ring-1 ring-[#58a6ff]/30'
                  : 'bg-[#161b22] hover:bg-[#21262d] border-[#30363d] text-[#e6edf3]'
              }`}
              aria-label="Toggle navigation menu"
              aria-expanded={isOpen}
              id="hamburger-menu-toggle"
            >
              {isOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
            </button>

            {/* Dropdown Menu Flyout */}
            {isOpen && (
              <div
                ref={menuRef}
                className="absolute right-0 mt-3 w-80 sm:w-96 rounded-xl border border-[#30363d] bg-[#0d1117] shadow-2xl shadow-black/80 overflow-hidden z-50 animate-in fade-in slide-in-from-top-2 duration-150"
                role="dialog"
                aria-label="Navigation Menu"
                id="main-hamburger-dropdown"
              >
                {/* Menu Header */}
                <div className="px-4 py-3 bg-[#161b22] border-b border-[#30363d] flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-mono font-bold text-[#8b949e] uppercase tracking-wider">
                      Navigation Menu
                    </span>
                    <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-[#58a6ff]/10 text-[#58a6ff] border border-[#58a6ff]/30">
                      7 Modules
                    </span>
                  </div>
                  <span className="text-[11px] font-mono text-[#8b949e]">
                    Press <kbd className="px-1 py-0.5 bg-[#080a0d] border border-[#30363d] rounded text-[10px]">ESC</kbd>
                  </span>
                </div>

                {/* Navigation Items List */}
                <div className="p-2 space-y-1 max-h-[calc(100vh-140px)] overflow-y-auto">
                  {navItems.map((item) => {
                    const Icon = item.icon;
                    const isActive = currentTab === item.id;
                    return (
                      <button
                        key={item.id}
                        onClick={() => handleItemClick(item.id)}
                        className={`w-full text-left p-2.5 rounded-lg flex items-center justify-between group transition-all cursor-pointer ${
                          isActive
                            ? 'bg-[#161b22] border border-[#58a6ff]/40 text-[#e6edf3]'
                            : 'hover:bg-[#161b22] border border-transparent text-[#8b949e] hover:text-[#e6edf3]'
                        }`}
                        id={`nav-item-${item.id}`}
                      >
                        <div className="flex items-start space-x-3 min-w-0">
                          <div
                            className={`w-8 h-8 rounded-md flex items-center justify-center shrink-0 mt-0.5 transition-colors ${
                              isActive
                                ? 'bg-[#58a6ff]/20 text-[#58a6ff] border border-[#58a6ff]/40'
                                : 'bg-[#161b22] text-[#8b949e] group-hover:text-[#58a6ff] group-hover:bg-[#21262d] border border-[#30363d]'
                            }`}
                          >
                            <Icon className="w-4 h-4" />
                          </div>
                          <div className="min-w-0 flex-1">
                            <div className="flex items-center space-x-2">
                              <span
                                className={`text-sm font-semibold truncate ${
                                  isActive ? 'text-[#58a6ff]' : 'text-[#e6edf3]'
                                }`}
                              >
                                {item.label}
                              </span>
                              {isActive && (
                                <span className="w-1.5 h-1.5 rounded-full bg-[#58a6ff]" />
                              )}
                            </div>
                            <p className="text-xs text-[#8b949e] line-clamp-1 mt-0.5 font-normal">
                              {item.description}
                            </p>
                          </div>
                        </div>

                        <div className="flex items-center space-x-1 shrink-0 ml-2">
                          <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-[#080a0d] border border-[#30363d] text-[#8b949e]">
                            {item.badge}
                          </span>
                          <ChevronRight
                            className={`w-4 h-4 transition-transform group-hover:translate-x-0.5 ${
                              isActive ? 'text-[#58a6ff]' : 'text-[#30363d] group-hover:text-[#8b949e]'
                            }`}
                          />
                        </div>
                      </button>
                    );
                  })}
                </div>

                {/* Footer of Menu */}
                <div className="p-3 bg-[#080a0d] border-t border-[#30363d] flex items-center justify-between text-xs font-mono text-[#8b949e]">
                  <div className="flex items-center space-x-1.5">
                    <ShieldCheck className="w-3.5 h-3.5 text-[#3fb950]" />
                    <span>Lean 4 Proof Microkernel</span>
                  </div>
                  <a
                    href="https://github.com/PhaseMirror/PIRTM"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center space-x-1 text-[#58a6ff] hover:underline"
                  >
                    <span>GitHub</span>
                    <ExternalLink className="w-3 h-3" />
                  </a>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}
