# ADR-021: Implement Real Admissibility Validation

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-09-01

## Context

`rust/pirtm-compiler/src/lib.rs` contains `AdmissibilityValidator::validate`, which unconditionally returns `Ok(())` for every AST expression:

```rust
pub fn validate(&self, _ast: &pirtm_parser::ast::Expr) -> Result<(), String> {
    Ok(())
}
```

The README claims the compiler performs "AdmissibilityValidator" checks. ADR-013 mandates that "user-declared floating-point manifests as proof of Small-Gain stability" are rejected.

## Hidden Assumption

That validation is performed elsewhere in the pipeline. In reality, no admissibility check exists between parsing and MLIR emission.

## Decision

1. **Implement `validate`** to reject:
   - Floating-point literals used as stability proofs.
   - Unbounded loop constructs without an explicit bound annotation.
   - Prime operators (`Ap(n)`) where `n` is not certified prime.
2. **Emit `ProofReceipt`** on successful validation, anchoring the receipt hash to the validated AST.
3. **Add compiler tests** that assert invalid manifests are rejected before lowering.

## Consequences

- The compiler enforces the L0 Scope Invariant from ADR-013.
- Validation failures produce deterministic error messages, not silent acceptance.
- The `AdmissibilityValidator` becomes a first-class governance gate, not a stub.
