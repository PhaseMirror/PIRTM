# ADR-063: GitHub Pages Automated Documentation Site Deployment

- **Status**: Proposed
- **Date**: 2026-09-03
- **Author**: Phase Mirror Formal Methods Engineering & Web Platform Engineering
- **Decider**: PIRTM Architectural Review Board

## Context

PIRTM contains machine-checked Lean 4 mathematical models, Architecture Decision Records (ADRs 001–062), spectral contractivity theorems, and Rust/WASM execution engines. Currently, these formal artifacts exist strictly as repository source files. To enable counsel, formal methods engineers, and external contributors to browse machine-checked proofs, search ADR decisions, and test spectral matrix contractivity interactively in the browser, PIRTM requires an automated, self-contained GitHub Pages site.

## Decision

1. **Automated Documentation Site Pipeline**:
   - Establish a dedicated GitHub Actions workflow (`.github/workflows/gh-pages.yml`) that builds and deploys the static documentation site to GitHub Pages on every push to `main` and release tag.
   - Combine three documentation layers into a unified site layout:
     1. **`mdBook` Core Documentation**: Architecture guides, PIRTM-lang specification, counsel playbooks, and EBNF packaging grammar.
     2. **Lean 4 `doc-gen4` API Reference**: Automatically generated hyperlinked API documentation for all zero-Mathlib core modules (`P2CCore.*`) and formal ADR decision records (`Foundations.ADR.*`).
     3. **Interactive WASM Spectral Playground**: WebAssembly build of `pirtm-engine` allowing users to input matrix tuples $A \in \mathbb{Q}^{n \times n}$ and spectral bounds $\lambda \in \mathbb{Q}^n$ to compute small-gain contractivity $\|A\|_1 < 1$ directly in client browsers.

2. **Machine-Checked ADR Search Index**:
   - `lake run generateDocs` exports a machine-readable `adrs.json` payload containing all Accepted/Proposed ADR records, their status, consequences, and linked Lean 4 theorem declarations (`lake_export_decls.json`).
   - The GitHub Pages frontend renders a searchable, client-side ADR matrix with real-time status badges and theorem verification links.

3. **Zero-Drift Production Mandate**:
   - The site build must execute inside CI directly against compiled `.olean` artifacts and Lake build outputs. Hand-edited HTML or detached documentation sources are strictly forbidden.

## Consequences

- Publishes interactive, machine-checked Lean 4 documentation and WASM contractivity tools to GitHub Pages (`PhaseMirror.github.io/PIRTM`).
- Guarantees zero documentation drift between Lean 4 proof files, Rust WASM engines, and published web documentation.
- Enables instant client-side audit of ADR lifecycle states, proof dependencies, and spectral contractivity bounds.
