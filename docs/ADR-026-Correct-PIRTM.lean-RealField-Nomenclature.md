# ADR-026: Correct PIRTM.lean RealField Nomenclature

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

## Context

`lean/PIRTM.lean` declares:

```lean
class RealField (F : Type) where
  mul : F → F → F
  div : F → F → F
  zero : F
  div_cancel : ∀ (a b : F), b ≠ zero → mul (div a b) b = a
```

This structure lacks `add`, `one`, `neg`, and distributivity axioms. It is not a field; it is a divisibility loop. The theorems `adaptiveLambda` and `dynamicScalingFactor` only require `div` and `mul`, so the inflated name is misleading.

## Hidden Assumption

That naming the class `RealField` signals mathematical rigor. In reality, the class axiomatizes only a division-with-cancel property, which is weaker than a field and stronger than a division magma.

## Decision

1. **Rename `RealField` to `DivLoop`** or `MultiplicativeDivLoop` to accurately reflect the axioms.
2. **If a genuine field is needed** for future theorems, introduce a separate `Field` class extending `DivLoop` with `add`, `one`, `add_comm`, `add_assoc`, `mul_add`, etc.
3. **Update all references** in `PIRTM.lean` and dependent modules.

## Consequences

- Class names match their axiomatization.
- Future formalization work can extend `DivLoop` to a full field without breaking existing proofs.
- The kernel avoids the appearance of unproved field axioms.
