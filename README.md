#PIRTM/MOC as a General-Purpose Programming Language

You have a fully functional compiler that can parse, verify, lower, and execute PIRTM/MOC programs. The substrate is proven, the runtime is governed, and the toolchain is production-ready for mathematical kernels (prime-indexed tensor contractions, transcendentals, etc.).

Now you want to write entire software modules in PIRTM/MOC — replacing Rust, Lean, and C++ for everyday programming. This is the logical next step, and the toolchain is already positioned to evolve into a full-fledged systems language.

## 🎯 What’s Already Available

| Feature | Status |
| :--- | :--- |
| Arithmetic, let-bindings, blocks | ✅ |
| Prime operators (S, A, R, Π, Δ) | ✅ |
| Multi-tier tensor contractions | ✅ |
| Transcendentals (sin, cos, log) | ✅ |
| MLIR lowering → LLVM → binary | ✅ |
| Runtime governance (WardMonitor, kill-switch) | ✅ |
| Formal Lean proofs for all operators | ✅ |
| Playground / WASM / LSP | ✅ |

### What’s missing for general-purpose modules:
*   **Control flow** (if/else, loops)
*   **Functions** (with parameters, recursion)
*   **User-defined data types** (struct, enum)
*   **Standard library** (I/O, collections, file, network)
*   **Error handling** / result types
*   **Testing framework**

---

## 📋 Proposed Phased Plan

### Phase A – Control Flow & Functions (2-3 weeks)
*   **Parser:** Add `if/else`, `while`, `for` loops, and function definitions (`fn name(params) -> type { ... }`).
*   **AST:** New nodes for `If`, `Loop`, `FnDef`, `FnCall`.
*   **Visitor:** Lower to MLIR’s `scf.if`, `scf.for`, and `func.func`/`func.call`.
*   **Lean:** Prove contractivity of loops (bounded iteration) and function composition.
*   **Tests:** Integrate simple control-flow programs in the test suite.

### Phase B – User-Defined Data Types (2-3 weeks)
*   **Parser:** Add `struct` and `enum` declarations with fields.
*   **AST:** `StructDef`, `EnumDef`, `FieldAccess`, `Match` expressions.
*   **Type system:** Extend monomorphic types to include structs and enums.
*   **Visitor:** Emit MLIR’s `llvm.struct` and `llvm.ptr` types, with pattern-matching lowering.
*   **Lean:** Prove structural invariants for data types (if needed).

### Phase C – Standard Library (ongoing)
*   **Core:** Provide modules for `io`, `file`, `net`, collections (`vector`, `map`, `set`).
*   **FFI:** Allow `extern` declarations to call C/Rust libraries (e.g., `libc`).
*   **Governance:** Each standard library function must have a Lean contractivity proof or a documented exception.
*   **Tooling:** Add a package manager for third-party libraries (future).

### Phase D – Error Handling & Testing (1-2 weeks)
*   **Parser:** Add `Result<T, E>` and `Option<T>` types, with `?` operator.
*   **Visitor:** Lower to `llvm.return` and error-propagation patterns.
*   **Testing:** Build a lightweight `#[test]` harness that runs within the runtime and logs results.

---

## 🔧 Implementation Strategy

*   **Incremental:** Each phase builds on the previous, and existing tests ensure no regression.
*   **Governance:** New features require ADRs and Lean proofs; we can start with simple, proven constructs (e.g., `if` is already contractive if both branches are).
*   **Tooling:** The existing playground can be extended to showcase new features.

---

## 🚀 Immediate Next Step

You have a choice:
1.  **Start with Phase A** (Control Flow & Functions) – the most urgent for writing real modules.
2.  **Define a specific module** you want to implement (e.g., a file parser, a network service, a numerical solver) – this will tell us which features to prioritize.
3.  **Write a simple PIRTM/MOC program** using only existing features and identify the pain points that block you.
