## 📄 ADR-045: UI/UX Integration for PIRTM

**Status:** Accepted  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The PIRTM compiler and runtime are fully verified (ADRs 001–044), with a working CLI, MCP server, and formal proof suite. To make the system accessible to a broader audience (developers, researchers, auditors), we need a web-based user interface (`pirtm-governed-toolchain`) that provides:

- A **playground** for writing and compiling PIRTM code, with live MLIR preview and contractivity receipts.
- A **governance dashboard** showing real-time spectral radius $\rho$, active sessions, and WardMonitor status.
- **Documentation** and **ADR index** automatically synchronized with the formal proof suite.
- An **MCP integration** playground for developers to experiment with the MCP server API.

The UI must be **governance-aware**: every compilation and tool execution must pass through the same contractivity and spectral gates as the CLI.

---

### Decision

We adopt the **Next.js Governed Toolchain Framework (`pirtm-governed-toolchain`)**:

1. **Frontend Architecture**: Next.js App Router with React, Tailwind CSS, Lucide icons, and Recharts for spectral metrics.
2. **Compiler & MCP Integration**: Provide native endpoints and WebAssembly/MCP bindings for in-browser PIRTM code evaluation, MLIR rendering, and contractivity verification.
3. **Governance Compliance**: Display read-only WardMonitor telemetry, live $\rho < 1.0$ contractivity limits, and deterministic receipt hashes for all actions.

---

### Consequences

#### Benefits
- **Full Spectrum Visibility**: Exposes real-time compiler receipts, MLIR lowering stages, and WardMonitor telemetry in a unified web interface.
- **Auditable Execution**: Ensures browser-side actions are bound by the exact same contractivity constraints as the CLI.

#### Costs / Risks
- **Frontend State Maintenance**: Syncing live WebSocket / REST telemetry across client components requires state management via React hooks and query caches.

---

### Links

- [ADR-038: Governance-Manifold-Fail-Closed-Control](./ADR-038-Governance-Manifold-Fail-Closed-Control.md)
- [ADR-042: Prime-Constitutional-Order-CSL](./ADR-042-Prime-Constitutional-Order-CSL.md)
- [pirtm-governed-toolchain](../../pirtm-governed-toolchain/)
