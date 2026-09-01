# Security Policy

## Supported Versions

| Version | Supported | Notes |
| ------- | --------- | ----- |
| 1.0.x-mvp | :white_check_mark: | Current MVP release |
| < 1.0.0 | :x: | Pre-release development |

## Reporting a Vulnerability

The Phase Mirror project takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please report security vulnerabilities by emailing:

📧 **security@phasemirror.com**

Include the following information in your report:

1. **Description**: A clear description of the vulnerability
2. **Impact**: The potential security impact (data exposure, privilege escalation, etc.)
3. **Reproduction Steps**: Detailed steps to reproduce the issue
4. **Affected Components**: Which parts of PIRTM/MOC are affected (kernel lexer, runtime, compiler, monitor, etc.)
5. **Suggested Fix**: If you have one (optional but appreciated)

### What to Expect

- **Acknowledgment**: Within 48 hours of your report
- **Initial Assessment**: Within 7 business days
- **Resolution Timeline**: Dependent on severity (see below)
- **Credit**: Public acknowledgment in release notes (unless you prefer anonymity)

### Severity Classifications

| Severity | Response Time | Examples |
|----------|--------------|----------|
| **Critical** | 24-48 hours | RCE, authentication bypass, contractivity proof bypass, kill-switch tampering |
| **High** | 7 days | Privilege escalation, sensitive data exposure, MLIR lowering corruption |
| **Medium** | 30 days | Information disclosure, CSRF, telemetry leakage |
| **Low** | 90 days | Minor information leaks, best practice violations |

## Security Architecture

### Threat Model

PIRTM/MOC operates as a formally governed compilation substrate that processes:

- **Governance Policies**: Sedona Spine constraints and contractivity invariants
- **Mathematical Kernels**: Prime-indexed tensor contractions and spectral radius computations
- **Execution Artifacts**: MLIR modules, LLVM IR, and native binaries
- **Telemetry**: Runtime drift metrics (ρ, δ) and Zeno damping state
- **Proof Receipts**: SHA-256 anchored validation artifacts

### Security Controls

#### Formal Verification
- **Lean 4 Core**: All kernel theorems are machine-checked in the zero-mathlib core
- **Admissibility Validation**: `AdmissibilityValidator` rejects float literals, unbounded loops, and uncertified primes at compile time
- **Spectral Radius Gate**: Small-Gain theorem enforced via `pirtm-engine::spectral`
- **Grammar Quarantine**: Kernel and application lexers are physically separated crates

#### Data Protection
- **Proof Anchoring**: All receipts are SHA-256 hashed and linked to validated ASTs or execution artifacts
- **No Telemetry Leakage**: Runtime metrics are local-only unless explicitly exported
- **Deterministic Execution**: Governed runtime follows a strict compile → validate → execute pipeline

#### Access Control
- **CI Gate Enforcement**: `.github/workflows/sedona_spine_ci.yml` enforces toolchain lock and build verification
- **Axiom Ledger**: All proof debts (`AX-*`) and enforcement gaps (`ENF-*`) are publicly tracked
- **Claim Table Fidelity**: `docs/PIRTM-README-Claim-Table.md` is the canonical ground-truth matrix

#### Infrastructure Security
- **Toolchain Pinning**: `lake-manifest.json` has `fixedToolchain: true`; CI verifies `lean --version` matches `lean-toolchain`
- **Reproducible Builds**: Lake and Cargo builds are deterministic and auditable
- **No External Dependencies**: Lean core is mathlib-free; Rust dependencies are pinned in `Cargo.lock`

### Known Security Boundaries

PIRTM/MOC's security model explicitly acknowledges:

1. **Trust Boundary**: The Lean formalization is trusted as ground truth; any bug in the Lean compiler itself could invalidate proofs.
2. **Blast Radius**: Runtime execution failures are isolated to the `pirtm-engine` process; the `SIG_GOV_KILL` tripwire ensures fail-closed behavior.
3. **Supply Chain**: Rust crates are audited via `cargo audit`; Lean packages are locked via `lake-manifest.json`.

## Security Best Practices for Deployers

### Compilation Pipeline

```bash
# Always verify the Lean kernel before deployment
./build.sh

# Verify Rust dependencies have no known vulnerabilities
cd rust
cargo audit

# Ensure spectral contractivity is validated
cargo run -p pirtm-engine -- validate-and-certify --ensemble your-ensemble.json
```

### Runtime Hardening

- **Never disable `SIG_GOV_KILL`**: The kill-switch is the final fail-closed guarantee.
- **Pin the Lean toolchain**: Do not upgrade Lean without corresponding proof re-verification.
- **Audit proof receipts**: Every compiled module should have a verifiable `ProofReceipt`.

### Incident Response

If you discover a vulnerability:

1. **Contain**: Isolate the affected component (e.g., disable compromised kernel operator).
2. **Assess**: Determine if the violation affects contractivity proofs or governance invariants.
3. **Remediate**: Apply the fix and regenerate affected proof receipts.
4. **Disclose**: Follow the coordinated disclosure process outlined above.

---

*Last Updated: 2026-09-01*
