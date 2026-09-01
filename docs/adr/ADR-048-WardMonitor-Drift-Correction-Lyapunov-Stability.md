# ADR-048: Formal WardMonitor Drift Correction & Lyapunov Stability

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

The WardMonitor runtime drift detector applies dynamic Zeno-Finton corrective gain $\kappa(t) = \kappa_0 e^{-\alpha t}$ to attenuate empirical spectral radius drift $\rho$. To ensure mathematically sound fail-closed governance, we formalize the WardMonitor's attenuation logic in Lean 4 and prove Lyapunov stability.

## Decision

1. **Formal Model (`lean/Foundations/ADR/WardMonitorStability.lean`)**:
   - Define integer-scaled drift metrics and `ZenoGain` structure.
   - Prove `zeno_attenuation_bounded` ($\rho_{\text{att}} \le \rho$).
   - Prove `ward_monitor_lyapunov_stable` ($V(\rho_{\text{att}}) \le V(\rho)$ for $V(\rho) = \rho^2$).

2. **Rust / Kani Verification (`rust/adr_rust/src/ward_monitor_proof.rs`)**:
   - Model `apply_zeno_gain` and `lyapunov_energy`.
   - Add `verify_adr048_ward_monitor_lyapunov_stability` Kani proof harness.

## Consequences

- The final proof gap in the WardMonitor drift-correction loop is formally closed with zero `sorry` warnings.
- Machine-checked attestation that Zeno-Finton gain application never increases Lyapunov energy.
