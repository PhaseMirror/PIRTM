# PIRTM Grounded Status & Claim Table

This table reflects the ground-truth status of all PIRTM/MOC components, replacing aspirational statements with verifiable status indicators.

| Subsystem / Feature | Claimed Status | Verified On-Tree Status | Verifying Test / Artifact |
|---|---|---|---|
| **Lexer & Parser** | Production-Grade | ✅ Complete | `pirtm-parser/tests/test_json_parser.rs` (17/17 top-level constructs) |
| **MLIR Dialect & Lowering** | Production-Grade | ✅ Complete | `examples/json_parser.mlir` generated via `pirtm compile` |
| **Mutable State (`let mut`, `=`)** | Verified | ✅ Complete | `pirtm-mlir/src/pirtm/transpiler/visitor.rs` (Stack alloca/store/load) |
| **Method Calls & Postfix Chaining** | Verified | ✅ Complete | FFI built-in dispatch to `string_len`, `vec_push`, `map_insert` |
| **WardMonitor Runtime Drift** | Verified | ✅ Complete | `pirtm-monitor` unit tests (Zeno damping & kill-switch) |
| **Small-Gain Spectral Radius ($\rho < 1$)** | Formal Invariant | ⚠️ In Progress (Scalar Mock in `linker.rs`) | Blocked on Stream R matrix computation refactor |
| **Lean Axiom-Clean Core** | Mathlib-Free | ✅ Complete | `lean/lakefile.toml` verified self-contained |
| **Sedona Spine CI Gate** | Continuous Enforcement | ✅ Complete | `.github/workflows/sedona_spine_ci.yml` on-tree |
