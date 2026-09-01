# ADR-026: Correct PIRTM.lean RealField Nomenclature

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **Renamed `RealField` to `DivLoop`** in `lean/PIRTM.lean`:
   - `class DivLoop (F : Type)` now declares `mul`, `div`, `zero`, and `div_cancel`.
   - All references (`dynamicScalingFactor`, `adaptiveLambda`, `k_equals_kappa`) updated to use `DivLoop`.
2. **No other files required changes** — `RealField` was not imported or referenced outside `PIRTM.lean`.
3. **Future extensibility preserved** — `DivLoop` can be extended with `add`, `one`, distributivity, etc. to form a genuine field class without breaking existing proofs.

## Validation

```bash
$ lake build --rehash
Build completed successfully (18 jobs).
```

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
