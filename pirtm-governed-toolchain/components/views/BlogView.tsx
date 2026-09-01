'use client';

import React, { useState } from 'react';
import {
  Newspaper,
  Calendar,
  Clock,
  Tag,
  ArrowRight,
  ChevronLeft,
  Share2,
  CheckCircle2,
  BookOpen,
  X
} from 'lucide-react';
import { BLOG_POSTS, BlogPost } from '@/lib/pirtm-data';

export function BlogView() {
  const [selectedTag, setSelectedTag] = useState<string>('All');
  const [activePost, setActivePost] = useState<BlogPost | null>(null);

  const allTags = ['All', 'Formal Methods', 'Lean 4', 'Compiler', 'AI Safety', 'MOC', 'Release', 'MLIR'];

  const filteredPosts = BLOG_POSTS.filter((post) => {
    if (selectedTag === 'All') return true;
    return post.tags.includes(selectedTag);
  });

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
      {/* Header */}
      <div className="flex flex-col md:flex-row md:items-center justify-between pb-6 border-b border-[#233147] gap-4">
        <div>
          <div className="flex items-center gap-2 text-xs font-mono text-cyan-400 mb-1">
            <Newspaper className="w-4 h-4" />
            <span>RESEARCH PAPERS &amp; RELEASES</span>
          </div>
          <h1 className="text-2xl sm:text-3xl font-bold text-[#e6edf3]">
            PIRTM Publications &amp; Technical Updates
          </h1>
          <p className="text-xs sm:text-sm text-[#8b949e] mt-1">
            Formal verification papers, release notes, and mathematical foundations of Multiplicity Operator Calculus.
          </p>
        </div>

        {/* Tag Filter Pills */}
        <div className="flex flex-wrap items-center gap-1.5">
          {allTags.map((tag) => (
            <button
              key={tag}
              onClick={() => setSelectedTag(tag)}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition-colors cursor-pointer ${
                selectedTag === tag
                  ? 'bg-[#1b2738] text-cyan-300 border border-cyan-500/40'
                  : 'bg-[#0e1624] text-[#8b949e] border border-[#233147] hover:text-[#e6edf3]'
              }`}
            >
              {tag}
            </button>
          ))}
        </div>
      </div>

      {/* Posts Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredPosts.map((post) => (
          <article
            key={post.id}
            className="p-6 rounded-xl bg-[#0e1624] border border-[#202e42] hover:border-cyan-500/50 transition-all flex flex-col justify-between group shadow-xl"
          >
            <div>
              {/* Meta */}
              <div className="flex items-center gap-3 text-xs text-[#8b949e] font-mono mb-3">
                <span className="flex items-center gap-1">
                  <Calendar className="w-3 h-3 text-cyan-400" />
                  {post.date}
                </span>
                <span>•</span>
                <span className="flex items-center gap-1">
                  <Clock className="w-3 h-3 text-[#6e7681]" />
                  {post.readTime}
                </span>
              </div>

              {/* Title */}
              <h2 className="text-base font-bold text-[#e6edf3] group-hover:text-cyan-300 transition-colors mb-3 leading-snug">
                {post.title}
              </h2>

              {/* Excerpt */}
              <p className="text-xs text-[#8b949e] leading-relaxed mb-4">
                {post.excerpt}
              </p>

              {/* Tags */}
              <div className="flex flex-wrap gap-1.5 mb-4">
                {post.tags.map((t) => (
                  <span
                    key={t}
                    className="px-2 py-0.5 rounded text-[10px] font-mono bg-[#090d14] text-cyan-400 border border-[#1e2a3c]"
                  >
                    {t}
                  </span>
                ))}
              </div>
            </div>

            {/* Read CTA */}
            <button
              onClick={() => setActivePost(post)}
              className="pt-3 border-t border-[#1b2738] flex items-center justify-between text-xs text-cyan-400 font-semibold group-hover:text-cyan-300 cursor-pointer"
            >
              <span>Read Full Paper</span>
              <ArrowRight className="w-3.5 h-3.5 group-hover:translate-x-1 transition-transform" />
            </button>
          </article>
        ))}
      </div>

      {/* Article Reader Modal */}
      {activePost && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md animate-in fade-in duration-150">
          <div className="w-full max-w-3xl bg-[#0e1624] border border-[#2d3f58] rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
            {/* Header */}
            <div className="px-6 py-4 bg-[#090d14] border-b border-[#233147] flex items-center justify-between">
              <div>
                <span className="text-xs font-mono text-cyan-400 font-bold">{activePost.date}</span>
                <h2 className="text-base sm:text-lg font-bold text-[#e6edf3] mt-0.5 leading-snug">
                  {activePost.title}
                </h2>
              </div>
              <button
                onClick={() => setActivePost(null)}
                className="p-1.5 rounded-lg text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#1b283d] transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Author info */}
            <div className="px-6 py-2.5 bg-[#0a0f18] border-b border-[#1e2a3c] flex items-center justify-between text-xs text-[#8b949e] font-mono">
              <span>Author: {activePost.author}</span>
              <span>{activePost.readTime}</span>
            </div>

            {/* Article Content */}
            <div className="p-6 overflow-y-auto space-y-4 text-xs sm:text-sm text-[#cbd5e1] leading-relaxed">
              <div className="whitespace-pre-line font-sans">
                {activePost.content}
              </div>
            </div>

            {/* Footer */}
            <div className="px-6 py-3 bg-[#090d14] border-t border-[#233147] flex items-center justify-between text-xs">
              <span className="text-[#8b949e] font-mono">PIRTM Research Archive</span>
              <button
                onClick={() => setActivePost(null)}
                className="px-4 py-1.5 rounded-lg bg-[#1b2738] hover:bg-[#25364e] font-semibold text-[#e6edf3] transition-colors"
              >
                Close Article
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
