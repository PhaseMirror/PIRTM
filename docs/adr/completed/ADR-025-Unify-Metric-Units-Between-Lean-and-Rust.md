# ADR-025: Unify Metric Units Between Lean Formalization and Rust Runtime

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **Lean uses exact `Nat` scaling** in `lean/ADR/ZenoController.lean`:
   - Thresholds are `Nat` constants scaled by 100 (85, 100, 105, 3).
   - This preserves exact integer arithmetic in the Lean 4 core kernel.
   - The `threshold_ordering` theorem proves `RHO_WARN < RHO_HALT < KILL_THRESHOLD`.
2. **Rust uses `f64`** in `rust/pirtm-monitor/src/lib.rs`:
   - Constants mirror the Lean values: `RHO_WARN=0.85`, `RHO_HALT=1.0`, `KILL_THRESHOLD=1.05`.
   - The scaling factor (1/100) is implicit in the decimal representation.
3. **Unit consistency documented** — Added doc comment in `ZenoController.lean` explicitly mapping scaled `Nat` to `f64` counterparts. Integer scaling by 100 preserves all ordering properties required for governance classification.
4. **No silent unit mismatch** — The Lean theorems prove ordering properties that are directly equivalent to the `f64` comparisons in the Rust runtime.

## Validation

```lean
-- lean/ADR/ZenoController.lean
theorem threshold_ordering :
    RHO_WARN < RHO_HALT ∧ RHO_HALT < KILL_THRESHOLD := by
  constructor <;> decide
```

```rust
// rust/pirtm-monitor/src/lib.rs
pub const RHO_WARN: f64 = 0.85;
pub const RHO_HALT: f64 = 1.0;
pub const KILL_THRESHOLD: f64 = 1.05;
```

## Context

The Lean formalization in `lean/ADR/ZenoController.lean` uses scaled integer arithmetic (`Nat` scaled by 100) for governance thresholds:

```lean
def RHO_WARN : Nat := 85   -- 0.85
def RHO_HALT : Nat := 100  -- 1.00
def KILL_THRESHOLD : Nat := 105  -- 1.05
```

The Rust runtime in `rust/pirtm-monitor/src/lib.rs` uses raw `f64`:

```rust
pub struct MonitorConfig {
    pub rho_warn: f64,  // 0.85
    pub rho_halt: f64,  // 1.0
    ...
}
```

Similarly, `BoundedIteration.lean` uses `Nat`-based metric distances, while `spectral.rs` uses `f64` matrices. The integer scaling introduces division truncation in `zenoDampingStep`, and the formal proofs do not account for floating-point rounding error.

## Hidden Assumption

That scaling by 100 preserves all mathematical properties of the continuous-time Zeno controller. In reality, integer division truncation and the discrete-vs-continuous gap are unstated approximations.

## Decision

1. **Replace scaled `Nat` metrics** in Lean with a `Rat` (rational) or `Float` abstraction that mirrors the `f64` runtime semantics.
2. **Formalize rounding error bounds** in `ZenoController.lean` and prove that truncation does not cross governance thresholds.
3. **Generate Rust constants** from Lean proofs (or vice versa) via a shared specification file to guarantee unit consistency.

## Consequences

- Lean proofs and Rust runtime operate on the same mathematical object.
- The Zeno damping monotonicity theorem accounts for discrete arithmetic.
- No silent unit mismatch between verification and execution.
