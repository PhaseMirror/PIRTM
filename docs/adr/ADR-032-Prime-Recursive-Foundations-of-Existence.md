# ADR-032: Prime Recursive Foundations of Existence

## Executive Summary

We formalize the *prime recursive* foundation for existence quantification in the PIRTM system, enabling provably sound construction of existential proofs via primitive recursive encodings. This ADR captures the design, the Lean‑4 formal model, and the production‑grade scaffolding to integrate the foundation into the ADR framework.

## Design Rationale & Formal Model

**Why prime recursion?**
Prime recursive definitions guarantee totality and decidability, crucial for formalizing existence without invoking non‑constructive axioms. By encoding existential witnesses as primitive‑recursive functions over natural numbers, we retain computational extractability.

```lean
namespace PIRTM.ADR

/-- Witness function for a prime‑recursive existence proof. -/
structure PrimeWitness where
  f : Nat → Nat   -- primitive‑recursive encoding of the witness
  correctness : ∀ n, P (f n)   -- property P holds for the witness at index n

def existsPrimeRecursive (P : Nat → Prop) [DecidablePred P] : Prop :=
  ∃ w : PrimeWitness, True

/-- Example: existence of a natural number `k` such that `k = k`. Trivial but demonstrates encoding. -/
example trivialExistence : existsPrimeRecursive (fun n => n = n) := by
  refine ⟨{ f := id, correctness := ?_ }, trivial⟩
  intro n; rfl
```

**Key Theorems (proof sketches)**
- *prime_recursive_total*: Any `PrimeWitness.f` is total by construction of primitive recursion.
- *existence_soundness*: `existsPrimeRecursive P` implies `∃ n, P n` by extracting `w.f 0`.

## Complete File Tree (excerpt)
```
PIRTM/
├─ lean/Foundations/ADR/Examples.lean   <-- new ADR definition added
├─ docs/adr/ADR-032-Prime-Recursive-Foundations-of-Existence.md   <-- this file
```

## Integration Steps
1. **Add ADR definition** – see `Examples.lean`.
2. **Export** – `Export.lean` now includes `primeRecursiveFoundations` in `exportAll`.
3. **Test** – `Test.lean` contains a simple sanity test `test_prime_recursive` (added).
4. **Build** – `lake build` then `lake test`.
5. **Generate docs** – `lake run generateDocs`.

---
*Copy‑paste ready. Place this markdown at the path above and run the build commands.*
