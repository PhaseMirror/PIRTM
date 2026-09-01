# ADR-021: Implement Real Admissibility Validation

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`AdmissibilityValidator` reimplemented** in `rust/pirtm-compiler/src/lib.rs` and `rust/pirtm-compiler/src/main.rs`:
   - `validate(&self, ast: &Expr) -> Result<ProofReceipt, String>` now recursively walks the AST.
   - Rejects `Expr::FloatLit` with error: `"L0 Invariant Violation: floating-point literal used as stability proof is forbidden"`.
   - Rejects `Expr::Atom { prime: n }` where `n` is not certified prime via `validate_prime`.
   - Rejects `Stmt::Loop { cond: None, .. }` with error: `"L0 Invariant Violation: unbounded loop without explicit bound annotation"`.
2. **`ProofReceipt` emitted on successful validation**:
   - `MlirModule` extended with `proof_receipt: Option<ProofReceipt>`.
   - Receipt hash is anchored to the validated source via SHA-256.
   - Signature field set to `"admissibility_validator"` to distinguish from Lean proof receipts.
3. **Compiler tests added** in `rust/pirtm-compiler/src/lib.rs`:
   - `test_admissibility_rejects_float_literal`
   - `test_admissibility_rejects_non_prime_atom`
   - `test_admissibility_accepts_prime_atom`
   - `test_admissibility_rejects_unbounded_loop`
   - `test_admissibility_proof_receipt_anchored_to_ast`

## Validation

```bash
$ cargo test -p pirtm-compiler
running 16 tests
test tests::test_admissibility_accepts_prime_atom ... ok
test tests::test_admissibility_rejects_float_literal ... ok
test tests::test_admissibility_rejects_non_prime_atom ... ok
test tests::test_admissibility_proof_receipt_anchored_to_ast ... ok
test tests::test_admissibility_rejects_unbounded_loop ... ok
...
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured
```

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
