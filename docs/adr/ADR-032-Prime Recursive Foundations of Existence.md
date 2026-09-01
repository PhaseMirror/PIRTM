## 📄 ADR-032: Prime‑Recursive Foundations Integration

**Status:** Proposed  
**Date:** 2026-09-01  
**Supersedes:** None  

---

### Context

The paper *The Prime‑Recursive Foundations of Mathematical Existence* (Citizen Gardens, 2026) provides a rigorous mathematical framework for prime‑indexed recursive tensor structures, stability guarantees, and universal construction. Our compiler (`PhaseMirror/PIRTM`) already implements many of its core mechanisms (contractivity receipts, spectral small‑gain, bounded iteration, MLIR lowering) but the connection between the theory and the implementation is not formally documented. Additionally, some aspects of the paper (tensor types, full axiomatic formalization, reinforcement learning integration) are not yet implemented.

**Key paper concepts:**
- Meta‑recursive function \(\mathcal{M}(P_N)\) with \(k < 1\).
- Prime‑indexed tensor recursion and fixed‑point convergence.
- Five axioms (Prime‑Indexed Basis, Recursive Feedback, Computation, Curvature, Entanglement).
- Four theorems (Eigenstructure, Stability, Computational Invariance, Hypercosmic Entanglement).
- Applications to AI/RL, quantum field theory, cryptography.

---

### Decision

We will formally integrate the paper’s theoretical framework into the compiler by:

1. **Documenting the alignment** – map every paper concept to an existing compiler component or proof, creating a traceability matrix.
2. **Extending the Lean proof suite** – formalize the paper’s five axioms and four theorems as machine‑checkable theorems in Lean 4, within `PhaseMirror/PIRTM/lean/`.
3. **Adding tensor types to the language** – introduce rank‑\((m,n)\) tensor types and operations (contraction, outer product, trace) with corresponding MLIR lowering and proof of contractivity.
4. **Extending the MCP server** – expose the new axioms/theorems and tensor operations as MCP tools, enabling external AI agents to query the theory and generate provably correct code.
5. **Creating a formal “theory snapshot”** – produce a Lean module that mirrors the paper’s definitions and proves its key results, effectively making the compiler a certified implementation of the theory.

**Non‑goals (for this ADR):**  
- Rewriting the entire compiler in a new paradigm.  
- Implementing quantum field theory or cryptography extensions (they are out of scope for the kernel language).

---

### Consequences

#### Benefits
- **Mathematical rigor:** The compiler becomes a certified implementation of a formal theory, with all major theorems machine‑checked.
- **Traceability:** Every compiler feature can be traced to a specific axiom/theorem, simplifying audits and external validation.
- **Future extensibility:** Tensor types and AI integration will unlock new application domains (RL, numerical simulation, etc.).
- **Community credibility:** The paper provides a high‑level narrative; the compiler provides the executable proof.

#### Drawbacks / Costs
- **Development effort:** Extending Lean proofs and adding tensor types will take 3–6 weeks of focused work.
- **Complexity:** Tensor types introduce a new dimension to the type system and MLIR lowering.
- **Performance:** Tensor operations may require additional runtime support (e.g., `llvm.matrix`), impacting JIT/AOT performance.

---

### Implementation Plan (Phased)

#### Phase 1: Traceability & Formalization (2 weeks)
- Create `docs/THEORY_ALIGNMENT.md` with a table mapping each paper axiom/theorem to compiler components (e.g., `SmallGainGate`, `BoundedIteration`, `ContractivityReceipt`).
- Create `lean/PrimeRecursiveAxioms.lean` defining the five axioms as formal statements.
- Create `lean/PrimeRecursiveTheorems.lean` proving the four theorems, relying on existing `BoundedIteration.lean` and `LoweringSoundness.lean`.
- Ensure all proofs are `sorry`‑free and Mathlib‑free.

#### Phase 2: Tensor Types (3 weeks)
- Extend `pirtm-parser` grammar (`pirtm.pest`) to support:
  - `tensor<type, dim1, dim2, ...>` (e.g., `tensor<f64, 3, 3>`).
  - Syntax for tensor literals and operations: `A ⊗ B`, `A · B`, `trace(A)`.
- Add AST nodes (`Type::Tensor`, `Expr::TensorContraction`, etc.).
- Extend type checker to validate tensor ranks and dimensions.
- Extend MLIR lowering to emit `llvm.matrix` or a custom `pirtm.tensor` dialect ops.
- Extend the runtime (FFI) with `nalgebra` for tensor operations when not compiled to LLVM.
- Add proof theorems for the contractivity of tensor operations (e.g., contraction preserves \(k<1\)).

#### Phase 3: MCP Tool Integration (1 week)
- Add new MCP tools:
  - `tensor_info` – describe a tensor type and its properties.
  - `contractivity_proof` – return the Lean proof for a given tensor operation.
  - `generate_rl_policy` – use PIRTM to synthesize an RL policy (per paper Section 3.4).
- Ensure all tools return receipts and audit logs.

#### Phase 4: Documentation & Release (1 week)
- Update `README.md` and `PIRTM-README-Claim-Table.md` to reflect the new tensor features and formal alignment.
- Publish the Lean proof suite as a standalone artifact.
- Tag a new release (`v2.0.0-theory`) if the implementation is stable.

---

### Links

- [Paper: The Prime‑Recursive Foundations of Mathematical Existence](./docs/paper.pdf) (local reference)
- [Existing Lean proofs](./lean/)
- [ADR‑016](./docs/adr/ADR-016-Bounded-Iteration-Control-Flow.md)
- [ADR‑017](./docs/adr/ADR-017-Lowering-Soundness.md)

---

### Sign‑off

**Approved by:** Governance Board (pending)  
**Date:** TBD  

*This ADR will be considered accepted once the Phase 1 artifacts (traceability table and Lean axioms/theorems) are merged into the tree.*