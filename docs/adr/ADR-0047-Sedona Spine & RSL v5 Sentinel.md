# ADR-047: Sedona Spine & RSL v5 Sentinel Integration

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

This phase embeds the **dual‑layer contractivity gates** into the runtime core, ensuring that every execution session is validated against both static (registration‑time) and dynamic (empirical stress) bounds, and that any violation triggers an immediate **SIG_GOV_KILL** (fail‑closed halt).

### Current State (Before Phase 2)

- **Small‑Gain gate** (ADR‑013) enforces \(\rho(|A|\operatorname{diag}(\lambda)) < 1\) at link time, before execution starts.
- **WardMonitor** continuously tracks drift metrics (ρ, δ, λ·L) and triggers warnings, but the kill‑switch is only activated when \(\rho \ge \rho_{\text{halt}}\) (1.0). It does not yet validate the full contractivity certificate or empirical stress bounds.
- **Contractivity receipts** are generated at compilation time but are not re‑verified at runtime.
- **`SIG_GOV_KILL`** is implemented as `std::process::exit(1)` in the WardMonitor but not yet integrated into all validation paths.

**Goal:**  
Implement `validate_and_seal` – a runtime gate that combines:

1. **Registration‑time checks (Rule‑HO‑01)**: Verify that the compiled operator chain satisfies \(\Vert \Lambda_m \mathcal{U} \Vert < 1\) (i.e., the contractivity bound) by re‑computing the spectral radius from the receipt or from the loaded IR.
2. **Empirical stress bounds**: Monitor the actual drift (ρ, δ, λ·L) from the WardMonitor and ensure they stay within the certified margins (e.g., \(\rho < 0.85\) for warning, \(\rho < 1.0\) for halt, and \(\delta < 10^{-4}\)).
3. **Sealing**: If all checks pass, emit a fresh `ContractivityReceipt` and anchor it to the Archivum ledger; if any check fails, trigger `SIG_GOV_KILL` immediately.

---

### 🔧 Implementation Steps

#### 2.1 Define `validate_and_seal` in `pirtm-engine`

**File:** `pirtm-engine/src/governance.rs` (new module)

```rust
use crate::spectral::{Ensemble, check_small_gain};
use pirtm_monitor::{MonitorConfig, WardMonitor, ManifoldStateProvider};
use antigrav_audit::record_event;
use serde_json::json;
use std::sync::Arc;

/// Runtime validation gate that combines static and dynamic checks.
pub struct Sentinel {
    config: MonitorConfig,
    monitor: WardMonitor<Box<dyn ManifoldStateProvider>>,
    // We may also need a reference to the loaded ensemble.
}

impl Sentinel {
    pub fn new(provider: Box<dyn ManifoldStateProvider>, config: MonitorConfig) -> Self {
        let monitor = WardMonitor::new(config.clone(), provider);
        Self { config, monitor }
    }

    /// Validate the current state against the contractivity certificate.
    /// Returns Ok(receipt_hash) if all checks pass, otherwise triggers SIG_GOV_KILL.
    pub fn validate_and_seal(&mut self, ensemble: &Ensemble) -> Result<String, String> {
        // 1. Registration-time check (Rule-HO-01): re-verify the spectral radius.
        match check_small_gain(ensemble) {
            Ok(()) => {
                // passes static check
            }
            Err(e) => {
                // fail-closed: kill immediately
                self.trigger_kill(&format!("Registration-time contractivity violation: {}", e));
                unreachable!();
            }
        }

        // 2. Empirical stress bounds: fetch current drift from the monitor.
        let state = self.monitor.provider.fetch_state()
            .map_err(|e| format!("Failed to fetch state: {}", e))?;

        // Check threshold: ρ < ρ_warn (0.85) for warning; if ρ >= ρ_halt (1.0) kill.
        // Also check delta (Finton bound) and lambda_l_product.
        if state.rho >= self.config.rho_halt {
            self.trigger_kill(&format!("Drift exceeded halt threshold: ρ = {}", state.rho));
        } else if state.rho >= self.config.rho_warn {
            // Log warning only (do not kill).
            record_event("sentinel_warning", json!({ "rho": state.rho, "delta": state.delta }));
        }

        if state.delta >= self.config.delta_max {
            self.trigger_kill(&format!("Liquidity pool drift exceeded: δ = {}", state.delta));
        }

        if state.lambda_l_product >= 1.0 {
            self.trigger_kill(&format!("Stability product exceeded: λ·L = {}", state.lambda_l_product));
        }

        // 3. Seal: generate a fresh receipt and anchor it.
        let receipt_hash = self.generate_receipt(ensemble, &state);
        record_event("sentinel_sealed", json!({
            "receipt": receipt_hash,
            "rho": state.rho,
            "delta": state.delta,
            "lambda_l_product": state.lambda_l_product
        }));

        Ok(receipt_hash)
    }

    fn trigger_kill(&self, reason: &str) -> ! {
        eprintln!("💀 SIG_GOV_KILL: {}", reason);
        record_event("sentinel_kill", json!({ "reason": reason }));
        std::process::exit(1);
    }

    fn generate_receipt(&self, ensemble: &Ensemble, state: &ManifoldState) -> String {
        // Use SHA-256 of ensemble + state as receipt hash.
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", ensemble).as_bytes());
        hasher.update(format!("{:?}", state).as_bytes());
        hex::encode(hasher.finalize())
    }
}
```

**Integration with existing `pirtm-engine`:**  
Modify `pirtm-engine/src/lib.rs` to include the new module and expose a public API that uses `Sentinel` before executing the loaded program.

---

#### 2.2 Embed the Sentinel into the Execution Loop

**File:** `pirtm-engine/src/lib.rs` (add/modify `Runtime::run`)

```rust
pub struct Runtime {
    // ... existing fields ...
    sentinel: Sentinel,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        // ... existing initialization ...
        let provider = monitor::RuntimeStateProvider::default(); // provide a real provider
        let sentinel = Sentinel::new(
            Box::new(provider),
            config.monitor_config.clone(),
        );
        Self { ... }
    }

    pub fn run(&mut self) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
        // 1. Load ensemble (if any) and validate through sentinel.
        // For now, we assume the ensemble is already loaded.
        if let Some(ensemble) = self.ensemble.as_ref() {
            self.sentinel.validate_and_seal(ensemble)?;
        }

        // 2. Execute the program (existing JIT logic).
        // ...

        Ok(receipt)
    }
}
```

We also need to modify the `run` subcommand in the CLI to pass the ensemble configuration (or load it) so that the sentinel has access to it. This is already done in ADR‑013.

---

#### 2.3 Wire WardMonitor to Sentinel

**File:** `pirtm-monitor/src/lib.rs` – the `WardMonitor` already polls the provider. We'll add a callback so that when a threshold is crossed, the monitor calls `Sentinel::validate_and_seal` (or directly triggers a kill if the sentinel is not available). Since the sentinel is in `pirtm-engine`, we can have the monitor accept a `kill_callback` closure that is invoked on halt conditions.

Alternatively, we can keep the monitor independent and have the runtime check the monitor state periodically. Simpler: the runtime can query the provider and run the sentinel before each iteration (or at session start). Since the sentinel already uses the provider, we can just call it once at start.

Given the design, we can make the sentinel read the latest state from the provider (which the WardMonitor also uses) and thus they share state.

---

#### 2.4 Update `SIG_GOV_KILL` Handling

The existing `trigger_kill` method uses `std::process::exit(1)`. This is a hard abort, which is acceptable for fail‑closed. We could also add a panic with a specific message, but exit is more deterministic.

We should ensure that all exit paths in the runtime go through a centralized kill function so that audit logs are always emitted.

---

#### 2.5 Integration Test

Add a new test in `pirtm-engine/tests/governance_integration.rs`:

- Compile a simple program (e.g., `Ap(2) + 3`).
- Run it with an ensemble that is stable (ρ < 1) – should pass.
- Then, artificially inject a drift violation (e.g., set the provider to return ρ = 1.1) and verify that `SIG_GOV_KILL` is triggered.

We can use the `MockStateProvider` from `pirtm-monitor` to control the state.

---

#### 2.6 Documentation & ADR Update

- Update ADR‑013 to reference the sentinel and the dual‑layer checks.
- Add a new section to the README explaining the runtime governance flow.

---

### 📦 Deliverables

| Artifact | Location | Purpose |
|----------|----------|---------|
| `governance.rs` | `pirtm-engine/src/` | `Sentinel` struct and `validate_and_seal` |
| Updated `lib.rs` | `pirtm-engine/src/` | Integrate sentinel into `Runtime::run` |
| Integration test | `pirtm-engine/tests/governance_integration.rs` | End‑to‑end validation of kill‑switch |
| ADR‑013 update | `docs/adr/ADR-013-PIRTM-MOC-Language-Scope.md` | Document the sentinel gates |

---

### ✅ Verification

- Run `cargo test --workspace` to ensure all existing tests pass.
- Run the integration test to verify that `validate_and_seal` correctly identifies stable and unstable states.
- Manually test a program with a known unstable ensemble and confirm the process exits with `SIG_GOV_KILL` and an audit log is written.

---

### ⏱️ Timeline

| Task | Owner | Effort |
|------|-------|--------|
| 2.1 Implement `Sentinel` | Runtime team | 2 days |
| 2.2 Integrate into `Runtime` | Runtime team | 1 day |
| 2.3 Wire with WardMonitor | Runtime team | 1 day |
| 2.4 Integration test | QA team | 1 day |
| 2.5 Documentation | Governance | 0.5 day |

**Total:** ~1 week.

---

Once Phase 2 is complete, the runtime will be **self‑governing**: it will enforce both the static contractivity certificate and the dynamic drift bounds before any execution, and will abort cleanly on any violation, leaving a verifiable audit trail. This is the final piece of the **Sedona Spine** mandate.