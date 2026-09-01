# Contributing to PIRTM/MOC

Thank you for your interest in contributing to the PIRTM/MOC (Phase Mirror / Multiplicity Object Code) project! This document provides guidelines for contributing to ensure consistency, quality, and alignment with our core principles.

## 📜 Code of Conduct

### Core Principles

PIRTM/MOC operates under the **Phase Mirror Governance** methodology. All contributions must:

1. **Preserve On-Tree Ground Truth**: Every claim in code or documentation must link to a physically existing, tested artifact on the current tree.
2. **Maintain Zero Tolerance for Simulation**: No `sorry` in proofs, no simulated telemetry in execution paths, no mock closures in production code.
3. **Enforce Contractivity Invariants**: All kernel operators must preserve the Small-Gain spectral radius bound ($\rho < 1.0$).
4. **Respect Grammar Quarantine**: Kernel tokens and application control-flow tokens must remain physically separated at the crate boundary.
5. **Follow Phase Mirror Methodology**: All non-trivial changes must reference an existing ADR or create one in `docs/`.

### Expected Behavior

- ✅ Be respectful and inclusive
- ✅ Provide constructive feedback
- ✅ Focus on soundness, minimality, and verifiability
- ✅ Document your changes thoroughly
- ✅ Follow established patterns and conventions
- ✅ Ensure all tests pass before requesting review
- ✅ Update ADRs and claim tables when changing behavior

### Unacceptable Behavior

- ❌ Introducing speculative claims without on-tree artifacts
- ❌ Using `sorry` or mock closures to bypass proof obligations
- ❌ Breaking the grammar quarantine (mixing kernel/app tokens)
- ❌ Bypassing security mechanisms
- ❌ Committing secrets or keys
- ❌ Harassment or discriminatory language
- ❌ Malicious code or backdoors

---

## 🛠 Development Environment Setup

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.81.0+ | Compiler toolchain for `rust/` crates |
| **Lean 4** | v4.33.0-rc2 | Formal verification kernel (managed by `elan`) |
| **Lake** | (bundled with Lean) | Build system for Lean modules |
| **LLVM** | 17+ (`mlir-translate`, `llc`, `clang`) | Real execution path in `pirtm-engine` |
| **Git** | 2.40+ | Version control |

### Initial Setup

```bash
# 1. Clone the repository
git clone https://github.com/PhaseMirror/PiLang.git
cd PiLang

# 2. Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install elan (Lean version manager)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
source $HOME/.elan/env

# 4. Install pinned Lean toolchain
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version

# 5. Build Rust workspace
cd rust
cargo build --workspace

# 6. Build Lean kernel
cd ..
./build.sh

# 7. Run tests
cd rust && cargo test --workspace
cd .. && lake build
```

### Environment Variables

Copy the example environment files if present:

```bash
cp .env.example .env
```

Edit with your local configuration. **Never commit real secrets**.

---

## 🔄 Development Workflow

### 1. Choose or Create an ADR

All non-trivial changes must reference an Architecture Decision Record.

- **Existing ADR**: Check `docs/` for relevant ADRs (018–030 currently active/resolved).
- **New ADR**: Create `docs/ADR-0XX-<topic>.md` with the standard format:
  ```markdown
  # ADR-0XX: <Title>
  - **Status**: Proposed
  - **Deciders**: Phase Mirror Governance, <role>
  - **Date**: YYYY-MM-DD
  ...
  ```

### 2. Make Changes

Follow the Phase Mirror methodology:

1. **Read the ADR** before changing code.
2. **Prove before claiming** — Lean proofs must discharge without `sorry`. Rust tests must pass without mocks.
3. **Update the claim table** — Any change affecting a subsystem's status must update `docs/PIRTM-README-Claim-Table.md` and recompute its SHA-256 hash.
4. **Sync `artifacts/` with `docs/`** — Changes to ADRs or claim tables must be reflected in both locations.

### 3. Verify Your Changes

Before opening a PR, run the full verification suite:

```bash
# Lean build (18 jobs must pass)
cd /path/to/PiLang
./build.sh

# Rust tests
cd rust
cargo test --workspace

# Clippy (Rust linting)
cargo clippy --workspace -- -D warnings

# Check for sorry in Lean
grep -r "sorry" lean/
# (should return no output)

# Check claim table is up to date
cd ..
sha256sum docs/PIRTM-README-Claim-Table.md
# Compare with the hash in the file header
```

### 4. Commit and Push

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Formatting, missing semicolons, etc. (no code change)
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Maintenance tasks (dependencies, build, etc.)
- `adr`: Architecture Decision Record changes

**Examples:**
```
feat(compiler): add admissibility validation for prime operators

Implements validate_prime check in AdmissibilityValidator::validate.
Rejects uncertified prime indices at compile time.

Closes #021
```

```
fix(kernel): restore ZenoController.lean after accidental rewrite

git checkout HEAD -- lean/ADR/ZenoController.lean

Fixes build failure in ADR.ZenoController target.
```

---

## 🧪 Testing Requirements

### Test Coverage Targets

| Component | Target | Notes |
|-----------|--------|-------|
| **Rust crates** | ≥ 80% | Unit + integration tests |
| **Lean modules** | 100% proof discharge | Zero `sorry` in `lean/` |
| **CI gates** | 100% pass rate | No flaky tests |

### Running Tests

```bash
# All Rust tests
cargo test --workspace

# Specific crate
cargo test -p pirtm-parser

# With output
cargo test -p pirtm-engine -- --nocapture

# Lean build verification
./build.sh
```

### Writing Tests

- **Rust**: Place tests in `tests/` directory or inline `#[cfg(test)]` modules.
- **Lean**: Add theorems to `lean/ADR/Test.lean` using the existing pattern (`by decide`, `constructor <;> rfl`, etc.).

---

## 📝 Code Style Guidelines

### Rust

```rust
// ✅ Use explicit types for public APIs
pub fn validate_prime(&self, n: u64) -> Result<(), String> { ... }

// ✅ Use descriptive names
let contractivity_receipt = spectral::validate_and_certify(&ensemble, 1e-6)?;

// ✅ Avoid magic numbers
const KILL_THRESHOLD: f64 = 1.05;

// ✅ Comment complex logic
// Compute spectral radius via power iteration for non-negative matrices
let rho = spectral_radius_power(&g, 1000, 1e-7)?;

// ❌ Avoid unwrap in production code
let x = data.unwrap(); // Bad

// ❌ Don't use simulated telemetry in non-dry-run paths
let metrics = telemetry::simulate_telemetry_collection(); // Bad outside dry_run
```

### Lean

```lean
-- ✅ Every definition has doc comments
/-! ## Local Order Lemmas for Rat (Zero-Mathlib) -/
theorem sub_nonneg_of_le {a b : Rat} (h : a ≤ b) : (0 : Rat) ≤ b - a := by ...

-- ✅ Use inductive for state machines
inductive ADRStatus where
  | Proposed | Accepted | Deprecated | Superseded

-- ✅ Use structure for records
structure ADR where
  id : ADRId
  title : String
  status : ADRStatus
  ...

-- ❌ No sorry in proofs
theorem foo : True := by
  sorry  -- Bad

-- ✅ Complete proofs only
theorem foo : True := by
  trivial
```

---

## 🔐 Security Checklist

Before submitting, verify:

- [ ] No secrets or API keys committed
- [ ] No PII in code or tests
- [ ] No new telemetry or tracking code
- [ ] Input validation on all user-provided data
- [ ] Proper error handling (no sensitive info in error messages)
- [ ] Access control checks on privileged functions
- [ ] No float literals used as stability proofs
- [ ] No unbounded loops without explicit bound annotations
- [ ] All prime operators validated with `validate_prime`
- [ ] `simulate_telemetry_collection` only in `--dry-run` branches

---

## 📚 Documentation Standards

### ADR Format

All ADRs must follow this structure:

```markdown
# ADR-0XX: <Title>

- **Status**: Proposed | Accepted | Resolved | Deprecated
- **Deciders**: Phase Mirror Governance, <role>
- **Date**: YYYY-MM-DD

## Context

...

## Hidden Assumption

...

## Decision

1. **<Action>**: ...

## Consequences

- ...
```

### Claim Table Updates

When changing a subsystem's status:

1. Update `docs/PIRTM-README-Claim-Table.md`
2. Recompute SHA-256: `sha256sum docs/PIRTM-README-Claim-Table.md`
3. Update the hash in the file header
4. Sync to `artifacts/PIRTM-README-Claim-Table.md`

---

## 🔀 Pull Request Process

1. **Ensure CI passes**: All checks in `.github/workflows/sedona_spine_ci.yml` must be green.
2. **Update documentation**: README, claim table, ADRs, and USER_GUIDE as needed.
3. **Request review**: At least one maintainer approval required.
4. **Address feedback**: Respond to all review comments before merge.
5. **Squash and merge**: Maintain a clean git history.

### Merge Requirements

- ✅ All CI checks passing (green)
- ✅ At least 1 approving review
- ✅ No unresolved conversations
- ✅ Up-to-date with target branch
- ✅ Conventional commit format
- ✅ ADR status updated if applicable

---

## 🆘 Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/PhaseMirror/PiLang/discussions)
- **Bugs**: Report via [GitHub Issues](https://github.com/PhaseMirror/PiLang/issues)
- **Security**: Email security@phasemirror.com (do NOT open public issue)
- **Documentation**: See [USER_GUIDE.md](USER_GUIDE.md) for comprehensive usage examples

---

## 📜 License

By contributing to PIRTM/MOC, you agree that your contributions will be licensed under the **Prime Materia Open Commons and Bound Works License v1.0** (see [LICENSE](LICENSE)).

Key provisions:
- No deployment, modification, or use is lawful unless Ξ(t+1) = Ψ(Ξ(t))
- No surveillance, profiling, monetization, or behavioral manipulation
- Freely forkable and composable under Ξ-certification

---

*Last Updated: 2026-09-01*
