# Contributing to ΛProof

Thank you for your interest in contributing to the ΛProof / MTPI / Web4 project! This document provides guidelines for contributing to ensure consistency, quality, and alignment with our core principles.

## 📜 Code of Conduct

### Core Principles

ΛProof operates under the **Ξ-Constitution** and **MTPI Framework**. All contributions must:

1. **Preserve Lawful Recursion**: Ξ(t+1) = Ψ(Ξ(t)) must hold
2. **Maintain Zero Surveillance**: No telemetry, tracking, or profiling
3. **Respect Sovereignty**: Analog life is sovereign; no coerced digitization
4. **Enforce Prime-Lawfulness**: Identity must be provably prime-indexed
5. **Control Semantic Drift**: δ(t) must remain below ε(t) or lawful fork required

### Expected Behavior

- ✅ Be respectful and inclusive
- ✅ Provide constructive feedback
- ✅ Focus on code quality and security
- ✅ Document your changes thoroughly
- ✅ Follow established patterns and conventions
- ✅ Prioritize user privacy and agency

### Unacceptable Behavior

- ❌ Introducing surveillance or tracking code
- ❌ Bypassing security mechanisms
- ❌ Exposing PII or sensitive data
- ❌ Harassment or discriminatory language
- ❌ Malicious code or backdoors

## 🛠 Development Environment Setup

### Prerequisites

- **Node.js** ≥ 20.18.0 (managed via package.json `engines`)
- **pnpm** 10.20.0 (enabled via `corepack enable`)
- **Rust** 1.81.0 (specified in `rust-toolchain.toml`)
- **Circom** 2.1.8 (installed via `pnpm toolchain:bootstrap`)
- **Git** (for version control)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/CitizenGardens-org/Lambda-Proof.git
cd Lambda-Proof

# Enable corepack (for pnpm)
corepack enable

# Install dependencies
pnpm install

# Install circom and other toolchain components
pnpm toolchain:bootstrap

# Run preflight checks
pnpm preflight:quick
```

### Environment Variables

Copy the example environment files:

```bash
cp .env.example .env
cp .env.sepolia.example .env.sepolia
```

Edit these files with your local configuration. **Never commit real secrets**.

### Building the Project

```bash
# Build all packages
pnpm build

# Build circuits (requires circom)
pnpm circuits:build

# Generate Solidity verifiers
pnpm circuits:verifiers

# Compile smart contracts
pnpm compile
```

### Running Tests

```bash
# Run all tests
pnpm test

# Run circuit tests
pnpm circuits:test

# Run quick circuit tests (smoke tests)
pnpm circuits:test:quick

# Run smart contract tests
pnpm -F @mtpi/mtpi-contracts test

# Run with coverage
pnpm test:coverage
```

## 📋 Minimal, Auditable Change Strategy

ΛProof follows a **minimal-change philosophy**:

### Before Making Changes

1. **Understand the system**: Read relevant documentation in `docs/`
2. **Identify scope**: Determine the minimal set of files to modify
3. **Check existing patterns**: Look for similar changes in git history
4. **Plan your approach**: Outline changes before implementation

### Making Changes

1. **Keep changes small**: One logical change per PR
2. **Preserve existing behavior**: Don't break working code unless necessary
3. **Document your rationale**: Explain why in commit messages and code comments
4. **Use existing helpers**: Don't reinvent functionality that already exists
5. **Follow file conventions**: Match existing code style

### After Making Changes

1. **Run linters**: `pnpm lint`
2. **Run tests**: Verify all tests pass
3. **Run preflight**: `pnpm preflight:quick` for full validation
4. **Review your diff**: Use `git diff` to ensure only intended changes are included
5. **Self-review**: Read your own code as if reviewing someone else's PR

## 🔀 Pull Request Guidelines

### Math-First PR Checklist

In accordance with [**ADR-001**](docs/adr/ADR-001-math-first-contract.md), every PR affecting core logic must satisfy the math-first contract. **A PR without a governing invariant is considered incomplete.**

- [ ] **Invariant Identified**: Name the mathematical invariant this change preserves or extends (e.g., "Spectral stability under prime reindexing").
- [ ] **Binding Module**: Reference the module in [`MATH_SPINE.md`](docs/MATH_SPINE.md) that owns this invariant.
- [ ] **Verification State**: State the current verification level for this change:
    - `[PROVEN]`: Includes/references a machine-checkable proof in `lean4/`.
    - `[TESTED/CI]`: Includes circuit tests or empirical benchmarks integrated into CI.
    - `[OPEN]`: Invariant is stated as a specification, but proof/test is pending. **Consult [docs/proof-obligations/](docs/proof-obligations/) for existing briefs.**

### PR Title Format

Use conventional commit format:

```
<type>(<scope>): <short description>

Examples:
feat(circuits): add recovery circuit with prime verification
fix(contracts): resolve reentrancy in MTPI_Core.withdraw
docs(readme): update installation instructions
chore(deps): bump circom from 2.1.7 to 2.1.8
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, no logic change)
- `refactor`: Code restructuring (no behavior change)
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, dependency updates
- `security`: Security fixes or improvements
- `ci`: CI/CD pipeline changes

### PR Description Template

```markdown
## 📝 Summary
Brief description of what this PR does.

## 🎯 Motivation
Why is this change needed? Link to issue if applicable.

## 🔧 Changes Made
- [ ] List specific changes
- [ ] One bullet per logical change
- [ ] Include file paths when helpful

## ✅ Testing
- [ ] Existing tests pass
- [ ] New tests added (if applicable)
- [ ] Manual testing performed
- [ ] Preflight checks pass

## 📚 Documentation
- [ ] README updated (if applicable)
- [ ] Inline comments added for complex logic
- [ ] API documentation updated (if applicable)

## 🔒 Security Considerations
- [ ] No PII exposure
- [ ] No new telemetry/tracking
- [ ] Secrets properly handled
- [ ] Reviewed for common vulnerabilities

## 🧩 Checklist
- [ ] Code follows project style guidelines
- [ ] Commit messages follow conventional format
- [ ] Self-reviewed my own code
- [ ] Requested review from relevant maintainers
```

### Review Process

1. **Automated Checks**: CI runs linters, tests, and security scans
2. **Peer Review**: At least one maintainer approval required
3. **Security Review**: Critical changes require security team review
4. **Final Checks**: Maintainer verifies compliance with MTPI principles

### Merge Requirements

- ✅ All CI checks passing (green)
- ✅ At least 1 approving review
- ✅ No unresolved conversations
- ✅ Up-to-date with target branch
- ✅ Conventional commit format
- ✅ Clean git history (squash if needed)

## 🧪 Testing Requirements

### Test Coverage Targets

- **Core Packages** (`@mtpi/*`): **≥ 80%** coverage
- **Experimental Packages**: **≥ 60%** coverage
- **Smart Contracts**: **≥ 90%** coverage (critical paths: 100%)
- **Circuits**: **100%** constraint coverage (all paths tested)

### Testing Pyramid

```
                    /\
                   /  \
                  / E2E \          <- 5% (smoke tests, integration)
                 /--------\
                /  Integ.  \       <- 15% (API, service layer)
               /------------\
              /     Unit      \    <- 80% (functions, components)
             /------------------\
```

**Unit Tests** (80%):
- Individual functions and methods
- Pure logic, no external dependencies
- Fast execution (< 100ms per test)
- Deterministic results

**Integration Tests** (15%):
- API endpoints
- Service interactions
- Database operations
- Circuit-contract integration

**E2E Tests** (5%):
- Complete user workflows
- Proof generation → verification → on-chain submission
- Critical paths only

### Testing Best Practices

```typescript
// ✅ Good: Descriptive, isolated, fast
describe('deriveUniquenessAnchor', () => {
  it('should produce deterministic hash for same inputs', () => {
    const anchor1 = deriveUniquenessAnchor('issuer', 'subject', 'context', 'salt');
    const anchor2 = deriveUniquenessAnchor('issuer', 'subject', 'context', 'salt');
    expect(anchor1).toBe(anchor2);
  });

  it('should produce different hashes for different subjects', () => {
    const anchor1 = deriveUniquenessAnchor('issuer', 'subject1', 'context', 'salt');
    const anchor2 = deriveUniquenessAnchor('issuer', 'subject2', 'context', 'salt');
    expect(anchor1).not.toBe(anchor2);
  });
});

// ❌ Bad: Vague, not isolated, slow
describe('identity stuff', () => {
  it('works', async () => {
    const result = await doEverything();
    expect(result).toBeTruthy();
  });
});
```

### Circuit Testing

All circuit changes require:

```bash
# Compile circuits
pnpm circuits:build

# Run constraint tests
pnpm circuits:test

# Verify soundness (test edge cases)
# Example: Test with maximum input sizes, boundary values, invalid inputs
```

See `docs/ops/docs/ops/CIRCUIT_TESTING_CHECKLIST.md` for comprehensive circuit testing guide.

## 📝 Code Style Guidelines

### TypeScript / JavaScript

```typescript
// ✅ Use explicit types
function deriveAnchor(issuer: string, subject: string): string { ... }

// ✅ Use const for immutable values
const MAX_DRIFT = 0.3;

// ✅ Use descriptive names
const uniquenessAnchor = deriveUniquenessAnchor(...);

// ✅ Avoid magic numbers
const SECONDS_PER_DAY = 86400;

// ✅ Comment complex logic
// Compute HMAC-SHA256 for irreversible identity commitment
const hmac = createHmac('sha256', salt);

// ❌ Avoid any types
function process(data: any) { ... } // Bad

// ❌ Don't mutate parameters
function addItem(list: string[]) {
  list.push('item'); // Bad - side effect
}
```

### Solidity

```solidity
// ✅ Use explicit visibility
function verifyProof(uint[2] memory a, ...) public view returns (bool) { ... }

// ✅ Follow naming conventions
contract MTPI_Core { ... }       // PascalCase for contracts
function _verifyInternal() private { ... }  // _prefix for private/internal
uint256 public constant MAX_DRIFT = 30;     // SCREAMING_SNAKE_CASE for constants

// ✅ Use natspec comments
/// @notice Verifies a zk-SNARK proof
/// @param proof The proof to verify
/// @return isValid True if proof is valid
function verify(Proof memory proof) public view returns (bool isValid) { ... }

// ✅ Check effects interactions (CEI pattern)
function withdraw() external {
  // Checks
  require(balance[msg.sender] > 0, "No balance");
  
  // Effects
  uint256 amount = balance[msg.sender];
  balance[msg.sender] = 0;
  
  // Interactions
  (bool success, ) = msg.sender.call{value: amount}("");
  require(success, "Transfer failed");
}
```

### Circom

```circom
// ✅ Use clear signal names
signal input identityHash;
signal output isValid;

// ✅ Document constraints
// Ensure drift is below threshold: drift <= MAX_DRIFT
component driftCheck = LessThan(8);

// ✅ Use templates for reusable logic
template PrimeGate() { ... }

// ✅ Add assertions for soundness
assert(constraint1 + constraint2 == expected);
```

## 🚀 Commit Message Conventions

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Examples

```
feat(circuits): add Miller-Rabin primality test circuit

Implements probabilistic primality testing with 40 rounds for
256-bit primes. Reduces proving time by 30% compared to trial
division approach.

Closes #123
```

```
fix(contracts): prevent reentrancy in MTPI_Core.claimMembership

Adds nonReentrant modifier to claimMembership function to prevent
reentrancy attacks during SBT minting.

BREAKING CHANGE: claimMembership now requires CEI pattern
```

### Commit Best Practices

- **One logical change per commit**: Easy to review and revert
- **Present tense**: "Add feature" not "Added feature"
- **Imperative mood**: "Fix bug" not "Fixes bug"
- **Reference issues**: Use "Closes #123" or "Refs #456"
- **Explain why**: Focus on motivation, not just what changed

## 🔐 Security Checklist

Before submitting, verify:

- [ ] No secrets or API keys committed
- [ ] No PII (names, emails, addresses) in code or tests
- [ ] No new telemetry or tracking code
- [ ] Input validation on all user-provided data
- [ ] Proper error handling (no sensitive info in error messages)
- [ ] Access control checks on privileged functions
- [ ] Reentrancy protection on state-changing functions
- [ ] Integer overflow/underflow checks (or use SafeMath)
- [ ] Gas optimizations don't compromise security
- [ ] Circuit constraints enforce all required properties

## 📚 Documentation Standards

### Code Comments

```typescript
/**
 * Derives an irreversible uniqueness anchor from verified identity.
 * 
 * This implements the commit phase of the commit-reveal identity model.
 * The anchor is computed as HMAC-SHA256(salt, issuer:subject:context),
 * ensuring that:
 * - Same identity always produces same anchor (deterministic)
 * - Different identities produce different anchors (collision-resistant)
 * - Anchor cannot be reversed to reveal identity (preimage-resistant)
 * 
 * @param issuerId - Stable issuer identifier (e.g., 'us-bank-abc')
 * @param subjectId - Verified subject ID from issuer (NOT raw PII)
 * @param context - Purpose context (e.g., 'membership')
 * @param salt - High-entropy secret (32+ bytes, base64-encoded)
 * @returns 0x-prefixed hex anchor hash (32 bytes)
 * 
 * @example
 * const anchor = deriveUniquenessAnchor(
 *   'us-bank-abc',
 *   'user-stable-id-123',
 *   'membership',
 *   process.env.UNIQUENESS_SALT_DEFAULT!
 * );
 */
export function deriveUniquenessAnchor(
  issuerId: string,
  subjectId: string,
  context: string,
  salt: string
): string {
  // Implementation...
}
```

### README Structure

Each package should have a README with:

1. **Overview**: What the package does (2-3 sentences)
2. **Installation**: How to install and set up
3. **Usage**: Basic examples
4. **API Reference**: Public functions and types
5. **Security**: Relevant security considerations
6. **License**: License information

## 🆘 Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/CitizenGardens-org/Lambda-Proof/discussions)
- **Bugs**: Report via [GitHub Issues](https://github.com/CitizenGardens-org/Lambda-Proof/issues)
- **Security**: Email security@citizengardens.org (do NOT open public issue)
- **Chat**: Join our community (link TBD)

## 📜 License

By contributing to ΛProof, you agree that your contributions will be licensed under the **Ξ-License v1.0** (see LICENSE.txt).

Key provisions:
- No deployment, modification, or use is lawful unless Ξ(t+1) = Ψ(Ξ(t))
- No surveillance, profiling, monetization, or behavioral manipulation
- Freely forkable and composable under Ξ-certification

---

**Thank you for contributing to ΛProof!** 🚀

*Last Updated: 2025-11-15*
