'use client';

import React, { useState, useEffect } from 'react';
import { Navbar } from '@/components/Navbar';
import { Footer } from '@/components/Footer';
import { CommandPalette } from '@/components/CommandPalette';
import { HomeView } from '@/components/views/HomeView';
import { DocsView } from '@/components/views/DocsView';
import { PlaygroundView } from '@/components/views/PlaygroundView';
import { DashboardView } from '@/components/views/DashboardView';
import { McpView } from '@/components/views/McpView';
import { BlogView } from '@/components/views/BlogView';
import { AboutView } from '@/components/views/AboutView';
import { motion, AnimatePresence } from 'motion/react';

export default function Page() {
  const [currentTab, setCurrentTab] = useState<string>('home');
  const [docsSubSection, setDocsSubSection] = useState<string>('language');
  const [playgroundPresetId, setPlaygroundPresetId] = useState<string | undefined>(undefined);
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  // Sync with browser hash if available
  useEffect(() => {
    const handleHash = () => {
      const hash = window.location.hash.replace('#', '');
      if (['home', 'docs', 'playground', 'dashboard', 'mcp', 'blog', 'about'].includes(hash)) {
        setCurrentTab(hash);
      }
    };

    handleHash();
    window.addEventListener('hashchange', handleHash);
    return () => window.removeEventListener('hashchange', handleHash);
  }, []);

  const handleNavigate = (tab: string, subSection?: string) => {
    setCurrentTab(tab);
    if (subSection) {
      setDocsSubSection(subSection);
    }
    window.location.hash = tab;
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  const handleNavigateToPlayground = (presetId?: string) => {
    setPlaygroundPresetId(presetId);
    handleNavigate('playground');
  };

  return (
    <div className="min-h-screen flex flex-col bg-[#080a0d] text-[#e6edf3] font-sans antialiased">
      {/* Top Navigation */}
      <Navbar
        currentTab={currentTab}
        onTabChange={(tab) => handleNavigate(tab)}
        onOpenSearch={() => setIsSearchOpen(true)}
      />

      {/* Main Content with Smooth Motion Transitions */}
      <main className="flex-1">
        <AnimatePresence mode="wait">
          <motion.div
            key={currentTab}
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -6 }}
            transition={{ duration: 0.18, ease: 'easeOut' }}
          >
            {currentTab === 'home' && (
              <HomeView onNavigate={handleNavigate} />
            )}

            {currentTab === 'docs' && (
              <DocsView
                initialSubSection={docsSubSection}
                onNavigateToPlayground={handleNavigateToPlayground}
              />
            )}

            {currentTab === 'playground' && (
              <PlaygroundView initialPresetId={playgroundPresetId} />
            )}

            {currentTab === 'dashboard' && (
              <DashboardView />
            )}

            {currentTab === 'mcp' && (
              <McpView />
            )}

            {currentTab === 'blog' && (
              <BlogView />
            )}

            {currentTab === 'about' && (
              <AboutView />
            )}
          </motion.div>
        </AnimatePresence>
      </main>

      {/* Footer */}
      <Footer onTabChange={handleNavigate} />

      {/* Command Palette (Cmd+K) */}
      <CommandPalette
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
        onNavigate={handleNavigate}
      />
    </div>
  );
}
