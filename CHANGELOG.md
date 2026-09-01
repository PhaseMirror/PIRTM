# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Phase Mirror methodology enforcement via `AGENTS.md`
- Axiom Ledger (`docs/PIRTM-axiom-ledger.md`) for tracking proof debts and enforcement gaps
- Ground-truth claim table (`docs/PIRTM-README-Claim-Table.md`) with SHA-256 audit hash
- `build.sh` validated build entrypoint
- `USER_GUIDE.md` comprehensive end-user documentation
- `INSTALL.md` detailed installation guide
- `SECURITY.md` security policy and vulnerability reporting
- `CONTRIBUTING.md` contribution guidelines and development workflow
- `CHANGELOG.md` version history

### Changed
- Updated `README.md` with table of contents, installation, and usage sections
- Renamed `RealField` to `DivLoop` in `lean/PIRTM.lean` (ADR-026)
- Replaced `"dummy"` conclusion in `fromADR` with real consequences (ADR-029)
- Updated `docs/PIRTM-README-Claim-Table.md` to reflect all resolved ADRs (018–030)

### Fixed
- Repaired broken Lean kernel imports (`prime_tensors` modules) (ADR-018)
- Eliminated all `sorry` from canonical core proofs (ADR-019)
- Fixed undefined `adr0999` reference in `Export.lean` (ADR-020)
- Implemented real admissibility validation (reject float literals, unbounded loops, uncertified primes) (ADR-021)
- Moved simulated telemetry behind `--dry-run` flag only (ADR-022)
- Enforced grammar quarantine via separate `pirtm-kernel-lexer` and `pirtm-app-lexer` crates (ADR-023)
- Locked Lean toolchain with `fixedToolchain: true` and CI version verification (ADR-024)
- Unified metric units between Lean `Nat` scaling and Rust `f64` constants (ADR-025)
- Fixed README build instructions (`cd lean` → `./build.sh`) (ADR-027)
- Reconciled claim table with Axiom Ledger and added SHA-256 audit hash (ADR-030)

## [1.0.0-mvp] - 2026-09-01

### Added
- Initial MVP release of PIRTM/MOC
- Lean 4 Axiom-Clean core (`lean/ADR/*.lean`, `lean/PIRTM.lean`)
- Rust compiler pipeline (`pirtm-parser`, `pirtm-mlir`, `pirtm-compiler`)
- Runtime execution engine (`pirtm-engine`) with real LLVM IR path
- WardMonitor drift detection and Zeno controller (`pirtm-monitor`)
- Standard library primitives (`pirtm-stdlib`)
- MLIR lowering for control flow, structs, enums, and FFI
- JSON parser end-to-end example (`examples/json_parser.pirtm`)
- Sedona Spine CI workflow with zero-drift toolchain locking
- 13 Architecture Decision Records (ADR-018 through ADR-030)
- Defensive publication whitepaper (`docs/DEFENSIVE_PUBLICATION_GOVERNANCE_AS_COMPILATION.md`)
- Prime Materia Open Commons License v1.0

### Security
- AdmissibilityValidator rejects float literals and uncertified primes
- Grammar quarantine enforced at crate boundary
- Proof receipts SHA-256 anchored to validated ASTs
- `SIG_GOV_KILL` fail-closed tripwire implemented

---

*For older releases, see [git tags](https://github.com/PhaseMirror/PiLang/tags).*
