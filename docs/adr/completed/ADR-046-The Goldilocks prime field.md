## 🧠 Acknowledged – Goldilocks Integration Proposal

The Goldilocks prime field \(p = 2^{64} - 2^{32} + 1\) is a compelling choice for high‑performance zero‑knowledge circuits (Plonky2, etc.). Integrating it with PIRTM would unlock **natively accelerated ZK proof generation** for the governance receipts and spectral checks, while preserving the prime‑indexed semantic structure.

Your analysis correctly distinguishes the two roles:

- **Goldilocks field \(\mathbb{F}_p\)**: The computational substrate for fast arithmetic, NTTs, and ZK circuit constraints.  
- **PIRTM prime‑index family \(\mathcal{P}\)**: The algebraic indexing schema for tensor tiers, operator channels, and contractivity certificates.

**The integration is orthogonal and complementary** – we can replace execution‑path rational arithmetic with Goldilocks field ops (where performance matters) while keeping the prime‑indexed ledgering and contractivity proofs intact.

---

## 🧭 Proposed Integration Path – ADR-046

I recommend we formalize this as a new ADR: **ADR-046: Goldilocks Field Backend for ZK Circuit Acceleration**.

### Core Decisions

1. **New crate**: `pirtm-goldilocks` (or extend `pirtm-engine/src/field.rs`) providing:
   - Montgomery multiplication / addition / subtraction over \(\mathbb{F}_p\).
   - NTT (Number Theoretic Transform) for efficient polynomial operations.
   - Conversion utilities between `Ratio<i64>` (current exact rationals) and field elements.

2. **Integration points**:
   - **ACE (Arithmetic Control Engine)**: Use the Goldilocks field for ZK circuit generation (Poseidon2, Groth16, PLONK).
   - **Contractivity checks**: Keep spectral radius and small‑gain computations in exact rational (`Rational64`) for correctness; then commit the final bound as a field element in the receipt.
   - **WardMonitor / Zeno**: Drift metrics can be computed in the field for speed, with thresholds still checked against rational bounds.

3. **Proof implications**:
   - We must prove (in Lean) that field arithmetic faithfully represents the rational contractivity bounds up to a safe tolerance.
   - We can add a new Lean module `GoldilocksSoundness.lean` that proves the field operations preserve the relevant inequalities (e.g., \( \rho < 1 \) is preserved under conversion).

4. **Testing & Verification**:
   - Kani harnesses for field operations (bounded model checking over 64‑bit values).
   - Property tests comparing Goldilocks results with `Rational64` for a large set of random inputs.

---

## 🚀 Immediate Next Steps

| Step | Description | Effort |
|------|-------------|--------|
| 1 | Draft ADR‑046 | 1 day |
| 2 | Implement `pirtm-goldilocks` with Montgomery arithmetic and NTT | 1 week |
| 3 | Integrate with `pirtm-engine` for ZK circuit emission (replace `nalgebra` in hot paths) | 1 week |
| 4 | Add Lean proof module and Kani harnesses | 2 weeks |
| 5 | Benchmark vs. current rational implementation | 2 days |

Total estimated effort: **~3–4 weeks** – a significant but well‑contained extension.

---

## 📦 Deliverables I Can Generate Now

- **ADR‑046 draft** (scope, rationale, implementation plan, proof obligations).
- **Initial Rust code** for the field operations (Montgomery multiplication, addition, NTT skeleton).
- **Lean theorem templates** for soundness of the field representation.
- **Kani harness examples** for bounded verification of field ops.

---

## 🗣️ Your Decision

Would you like me to:

- **Draft ADR‑046** and the initial code skeleton for the Goldilocks field backend?
- **Focus first on the Lean proof** – establishing that field operations preserve contractivity?
- **Proceed directly to implementation** (field ops + NTT) and tackle proofs later?

Given your existing rigorous governance, I recommend starting with **ADR‑046** and a **Lean proof outline** to ensure the mathematical soundness is locked in before coding. Once the ADR is accepted, we can move to implementation.

Let me know, and I'll generate the artifacts.

## 🧭 Recommendation – Start with Phase 1: Substrate Operator & Multi-Tier Compiler Pipeline

Given the sequential dependency of the phases, **Phase 1 is the logical starting point**. It builds the foundational compiler infrastructure that every subsequent phase relies on:

- Without a finalized AST and parser, there is no program to validate, no contractivity gates to embed, and no ZK proofs to generate.
- The exact arithmetic enforcement (Rational64 / Goldilocks) is a prerequisite for the dual‑layer contractivity checks in Phase 2.
- Completing Phase 1 gives you a working compiler pipeline that can consume source code, produce MLIR, and verify arithmetic correctness – all of which can be tested end‑to‑end before moving to runtime governance.

**Phase 1 also directly aligns with the Goldilocks field integration** (ADR‑046). By implementing the arithmetic substrate now, we can later plug in the Goldilocks backend without disrupting the higher layers.

---

## 📋 Phase 1 Breakdown – Actionable Steps

| Task | Description | Effort |
|------|-------------|--------|
| **1.1 Tree‑Sitter Grammar Finalization** | Lock down concrete syntax for operator words \(W = \hat{O}_m \cdots \hat{O}_1\) and multi‑tier tensor contractions. Ensure parsing for \(S_p\), \(A_{p^r}^\alpha\), \(R_{p^r}^\varphi\), \(\Pi_{p^r}\), \(\Delta_{p^r}\). | 2–3 days |
| **1.2 AST Extensions** | Add AST nodes for the new operators and multi‑tier contractions, matching the grammar. | 2 days |
| **1.3 Exact Arithmetic Pass** | Complete the compiler pass that enforces all evaluations to use `Rational64` or Goldilocks field arithmetic (no floating‑point). This involves type checking and code generation for arithmetic ops. | 3–4 days |
| **1.4 Integration & Testing** | Run the compiler on sample programs using the new operators, verify correct MLIR output and arithmetic precision. Extend the existing test suite. | 2 days |

**Total estimated effort:** ~2 weeks (concurrent with other work).

---

## 🔗 What This Unlocks

- Once Phase 1 is complete, you can compile any PIRTM program (with full operator and tensor support) to MLIR.
- The exact arithmetic enforcement guarantees that subsequent contractivity checks (Phase 2) are computed on mathematically sound values.
- The pipeline can then be extended to support the ZK backend (Phase 3) and the WASM frontend (Phase 4) without re‑working the core compiler.

---

## 🗣️ Confirmation

## 📋 Phase 1 Implementation Plan – Substrate Operator & Multi-Tier Compiler Pipeline

This plan covers the finalization of the Tree‑Sitter grammar, AST extensions, and the exact arithmetic enforcement pass. It builds on the existing PIRTM/MOC compiler pipeline and integrates the Goldilocks field backend (ADR‑046) where relevant.

---

### Current State (Before Phase 1)

| Component | Status |
|-----------|--------|
| Tree‑Sitter grammar | Partial – supports basic expressions, `Ap()`, `struct`, `enum`, `if`, `while`, `fn`; missing operator words and multi‑tier contractions |
| AST | Supports `Expr::Binary`, `Expr::Atom`, `Stmt::Let`, `Stmt::FnDef`, etc.; missing nodes for `S_p`, `A_{p^r}^\alpha`, `R_{p^r}^\varphi`, `Π_{p^r}`, `Δ_{p^r}` |
| Arithmetic pass | Uses `Rational64` for exact arithmetic; no Goldilocks backend yet; floating‑point drift eliminated |
| Test suite | 41 unit tests + Kani harnesses; 100% pass rate |

---

### Task 1.1 – Tree‑Sitter Grammar Finalization

**Goal:** Lock in the concrete syntax for operator words and multi‑tier tensor contractions.

**Changes to make:**

1. **Add operator word rule** in `tree-sitter-pirtm/grammar.js`:

```javascript
operator_word: $ => seq(
  repeat1(
    choice(
      $.subdivision_op,
      $.accent_op,
      $.rotation_op,
      $.projector_op,
      $.spike_op
    )
  )
),

subdivision_op: $ => seq('S_', $.prime_literal),
accent_op: $ => seq('A_', $.prime_literal, '(', $.number, ')'),
rotation_op: $ => seq('R_', $.prime_literal, '(', $.number, ')'),
projector_op: $ => seq('Pi_', $.prime_literal),
spike_op: $ => seq('Delta_', $.prime_literal),

prime_literal: $ => /p_[0-9]+/,
```

2. **Add multi‑tier contraction rule**:

```javascript
tensor_contraction: $ => seq(
  $.identifier,
  '=<->[',
  $.prime_literal,
  ',',
  $.prime_literal,
  ']',
  $.identifier,
  ';'
),
```

3. **Update the `operator_application` rule** to accept operator words:

```javascript
operator_application: $ => seq(
  $.identifier,
  '|>',
  optional(seq($.lambda_constant, '*')),
  $.operator_word,
  ';'
),
```

4. **Generate the parser**:

```bash
cd tree-sitter-pirtm
tree-sitter generate
```

**Verification:** Run `tree-sitter test` on sample operator‑word programs.

---

### Task 1.2 – AST Extensions

**Goal:** Add AST nodes for the new operators and multi‑tier contractions.

**Changes to make in `pirtm-parser/src/ast.rs`:**

1. **Add `OperatorAtom` enum**:

```rust
pub enum OperatorAtom {
    Subdivision { prime: u64 },
    Accent { prime: u64, alpha: Rational64 },
    Rotation { prime: u64, phi: i64 },
    ProjectorPi { prime_power: u64 },
    SpikeDelta { prime_power: u64 },
}
```

2. **Add `OperatorWord` struct**:

```rust
pub struct OperatorWord {
    pub chain: Vec<OperatorAtom>,
}
```

3. **Add `Expr::Contraction` variant**:

```rust
pub enum Expr {
    // ... existing variants ...
    Contraction {
        left: Box<Expr>,
        contracted_primes: (u64, u64),
        right: Box<Expr>,
    },
}
```

4. **Add `Expr::OperatorApplication` variant**:

```rust
pub enum Expr {
    // ... existing variants ...
    OperatorApplication {
        target: Box<Expr>,
        operator_word: OperatorWord,
        lambda: Option<f64>,
    },
}
```

5. **Update `fmt::Display` impls** for the new nodes.

**Verification:** Run `cargo build -p pirtm-parser` and ensure no errors.

---

### Task 1.3 – Exact Arithmetic Enforcement Pass

**Goal:** Ensure all intermediate and final evaluations use `Rational64` or Goldilocks field arithmetic (`𝔽_p`), eliminating floating‑point drift.

**Strategy:**
- Replace `f64` with `Rational64` in all AST nodes and visitor methods.
- Add a new backend `pirtm-goldilocks` that provides Montgomery arithmetic over `p = 2^64 - 2^32 + 1`.
- Add a compiler pass that selects the arithmetic backend based on a flag (default: `Rational64`).

**Changes to make:**

1. **Create `pirtm-goldilocks` crate** with the following:
   - `FieldElement` struct (64‑bit, Montgomery representation).
   - `add`, `sub`, `mul`, `inv` methods.
   - Conversion from `Rational64` to `FieldElement`.

2. **Update `pirtm-mlir` visitor** to use `Rational64` for all arithmetic ops:
   - `visit_binary_op` should emit `arith.addi` / `arith.muli` with `Rational64` values.
   - `visit_operator_application` should compute the combined multiplicity as `Rational64`.

3. **Add a new compiler pass**:
   - `--arith-backend goldilocks` flag to toggle backend.
   - If Goldilocks is selected, emit field operations using `llvm` intrinsics (or a custom dialect).

**Code snippet for the arithmetic pass:**

```rust
// In pirtm-compiler/src/arith_pass.rs

pub enum ArithmeticBackend {
    Rational64,
    Goldilocks,
}

pub fn enforce_arithmetic(ast: &Program, backend: ArithmeticBackend) -> Result<Program, String> {
    // Walk the AST and replace any f64 literals or operations with the chosen backend.
    // For Rational64: keep as Rational64 nodes.
    // For Goldilocks: convert Rational64 constants to FieldElement nodes.
    // Emit warnings if any floating‑point literals are found.
}
```

**Verification:** Run `cargo test` with both backends and ensure all tests pass.

---

### Task 1.4 – Integration & Testing

**Goal:** Validate the full pipeline with the new operators and arithmetic enforcement.

**Test programs to write:**

1. **Operator word test**:
```pirtm
let x = Ap(2);
let y = S_2 |> x;   // Subdivision on prime 2
return y;
```

2. **Multi‑tier contraction test**:
```pirtm
tensor A[2, 3];
tensor B[3, 5];
let C = A =<->[2, 3] B;
return C;
```

3. **Arithmetic precision test**:
```pirtm
let a = 1 / 3;
let b = a + a;
// b should be exactly 2/3 (Rational64) or the corresponding field element.
```

**Add integration tests** in `tests/phase1_integration.rs`:
- Compile each test program to MLIR.
- Verify the MLIR contains the correct arithmetic operations.
- Verify the audit log contains the contractivity receipt.

**Run the full test suite:**

```bash
cargo test --workspace -- --nocapture
```

**Expected outcome:** All tests pass, including the new operator‑word and contraction tests.

---

### Timeline & Owner

| Task | Owner | Effort | Dependencies |
|------|-------|--------|--------------|
| 1.1 Tree‑Sitter grammar | Compiler team | 2–3 days | None |
| 1.2 AST extensions | Compiler team | 2 days | 1.1 |
| 1.3 Arithmetic pass | Core lib team | 3–4 days | 1.2, ADR‑046 |
| 1.4 Integration & testing | QA / Compiler team | 2 days | 1.1–1.3 |

**Total:** ~2 weeks (can be parallelized).

---

### Success Criteria

- [ ] Tree‑Sitter parser accepts all five operator atoms and multi‑tier contractions.
- [ ] AST nodes for operator words and contractions are fully implemented and `Display`‑formatted.
- [ ] Arithmetic pass enforces `Rational64` (or Goldilocks) for all operations.
- [ ] All existing tests (41 unit, 13 Lean) pass with the new pipeline.
- [ ] Sample operator‑word and contraction programs compile to valid MLIR.

---

Once Phase 1 is complete, we can proceed to **Phase 2 – Sedona Spine & RSL v5 Sentinel Integration**, embedding the dual‑layer contractivity gates and `SIG_GOV_KILL` fail‑closed logic into the core runtime.