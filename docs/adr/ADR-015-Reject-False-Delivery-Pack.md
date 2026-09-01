# ADR-015: Rejection of False Delivery Packs and Simulated Completion

- **Status**: Accepted
- **Deciders**: Phase Mirror Governance, Compiler Engineering
- **Date**: 2026-08-31

## Context
Previous iterations generated speculative delivery "packs" and claimed completion markers (e.g., CI enforcement, link-time spectral radius validation, and full mathematical theorem discharge) that were not physically mirrored on the repository tree (`d5963ac`). This created a divergence between aspirational README claims and actual compiler capabilities (the "README pattern").

## Decision
1. **Refusal of Off-Tree Speculative Packs**: Delivery packs that do not directly correspond to physically committed, executable, and tested artifacts on the current tree are rejected.
2. **Strict On-Tree Grounding**:
   - Every claim in `PIRTM-README-Claim-Table.md` must link to an existing, verifiable test or physical artifact on tree.
   - Link-time spectral radius $\rho(|A|\,\mathrm{diag}(\lambda))$ remains marked "In Progress" until the true non-negative matrix small-gain gate is physically implemented in Rust.
   - The grammar authority established in ADR-014 (`tree-sitter-pirtm` for kernel tokens, Pest for package envelopes) is bound without atticing or destructive rewrites.
3. **No Phantom CI Claims**: CI workflows (`.github/workflows/sedona_spine_ci.yml`) must physically exist and execute real gates rather than existing purely as documentation text.

## Consequences
- The governance and documentation substrate is locked to ground truth.
- Zero tolerance for simulated or speculative completion claims.
- The next development milestone (Stream R spectral small-gain gate) will be implemented directly in runtime code without modifying the established grammar.
