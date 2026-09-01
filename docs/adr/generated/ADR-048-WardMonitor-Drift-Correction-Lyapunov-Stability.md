# Formal WardMonitor Drift Correction & Lyapunov Stability

- **ID**: 48
- **Status**: Accepted
- **Context**: The runtime drift detector applies dynamic Zeno-Finton gain to attenuate spectral drift; a machine-checked proof is required to guarantee Lyapunov stability.
- **Decision**: Formalize Zeno attenuation in Lean 4 and prove Lyapunov stability V(\rho_{\text{att}}) <= V(\rho).
- **Consequences**:
- Machine-check Zeno attenuation boundedness \rho_{\text{att}} <= \rho.
- Prove Lyapunov energy strict non-increase under gain application.
- Close final runtime governance proof gap in Lean 4 core.
- **Supersedes**: none
- **Links**:
- [ADR-048 Document](../docs/adr/ADR-048-WardMonitor-Drift-Correction-Lyapunov-Stability.md)
