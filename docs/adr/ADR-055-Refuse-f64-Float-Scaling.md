# ADR-055: Refuse f64 Float Scaling & Establish Exact Rational Constructor Membrane

- **Status**: Accepted
- **Date**: 2026-09-02
- **Author**: Phase Mirror Formal Methods & Language Steward
- **Decider**: PIRTM Architectural Review Board
- **Replaces**: `0092f80e` (Reverted Float Scaling & Unresolved Merge Conflict)

---

## Executive Summary

ADR-055 ratifies the complete elimination of IEEE 754 floating-point scaling membranes ($10^6$) and un-anchored `author_declared_lambda` defaults from the PIRTM small-gain spectral verification pipeline. It establishes `Ensemble::from_rationals` as the canonical 100% exact rational constructor operating over GCD-reduced rational pairs `PosRat(p/q)` in $\mathbb{Q}$, backed by machine-checked matrix 1-norm dominance $\|G\|_1 < 1$.

---

## Decision Contract

### 1. Phase 1 (Deprecation & Kernel Cutover)
- **`Ensemble::new(f64)` Deprecation**: Mark `Ensemble::new(f64)` with `#[deprecated]`. `Ensemble::new` remains available for SDK signature compatibility during Phase 1.
- **Canonical Constructor (`from_rationals`)**: Introduce `Ensemble::from_rationals()` accepting exact rational pair tuples `(u64, u64)` for both matrix entries $A_{ij}$ and gain vector $\lambda_j$, along with a mandatory Lean `theorem_name` anchor.
- **Fail-Closed Anchor Verification**: Empty or invalid `theorem_name` identifiers hard-fail with `EnsembleError::MissingTheoremAnchor`. No `author_declared_lambda` default fallback is permitted in production code paths.
- **Internal Kernel Enforcement**: `pirtm-compiler`, `linker.rs`, `governance.rs` (Sentinel), and `pirtm-mcp` tools cut over to `from_rationals()` exclusively.

### 2. Phase 2 (Hard Sunset Clocks)
`Ensemble::new(f64)` will be completely purged from the codebase upon reaching whichever of the following two triggers occurs first:
- **Hard Calendar Bound**: `2026-10-01T00:00:00Z` (30 days post-ratification).
- **Hard Tag Bound**: Release tag **`v1.0.1-mvp`** (or `v1.1.0-mvp`).
- *Note*: `v1.0.0-Stable` remains strictly forbidden per ADR-012 until 100% of claim-table items are formally verified.

### 3. Boundary & Error Conditions
- **Zero Denominator ($q = 0$)**: Any rational pair $(p, 0)$ or $(0, 0)$ hard-fails with `EnsembleError::InvalidRational`.
- **Zero Gain ($\lambda_j = 0/1$)**: Non-negative zero gain $\lambda_j = 0$ is ALLOWED ($\lambda_j \ge 0$). In matrix $G = |A| \cdot \mathrm{diag}(\lambda)$, setting $\lambda_j = 0$ zeroes out column $j$, contributing $0$ to the column sum and trivially satisfying $\|G\|_1 < 1$.
- **Negative Gain ($\lambda_j < 0$)**: Hard-fails with `EnsembleError::InvalidGain`.

---

## Mathematical Formalization

Let $A \in \mathbb{Q}_{\ge 0}^{n \times n}$ be the interconnection matrix where $A_{ij} = \frac{p_{ij}}{q_{ij}}$, and let $\lambda \in \mathbb{Q}_{\ge 0}^n$ be the gain vector where $\lambda_j = \frac{p_j}{q_j}$.

The exact rational 1-norm spectral gain matrix $G \in \mathbb{Q}_{\ge 0}^{n \times n}$ has entries:

$$G_{ij} = A_{ij} \cdot \lambda_j = \left( \frac{p_{ij}}{q_{ij}} \right) \cdot \left( \frac{p_j}{q_j} \right) \in \mathbb{Q}_{\ge 0}$$

The production contractivity gate enforces:

$$\|G\|_1 = \max_{1 \le j \le n} \sum_{i=1}^n G_{ij} < 1 \qquad \text{in } \mathbb{Q}$$

Since $\rho(G) \le \|G\|_1$, exact rational norm contractivity $\|G\|_1 < 1$ guarantees spectral contractivity $\rho(G) < 1$ with zero floating-point approximation drift.
