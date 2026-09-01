# ADR-027: Fix README Build Instructions

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Documentation
- **Date**: 2026-09-01

## Context

`PiLang/README.md` instructs users to verify Lean with:

```bash
cd lean
lake build
```

However, `lakefile.toml` resides at `PiLang/lakefile.toml`, not `PiLang/lean/lakefile.toml`. Running `cd lean && lake build` fails because Lake looks for `lean/lakefile.toml`, which does not exist.

## Decision

1. **Correct the build instructions** to:
   ```bash
   cd PiLang
   lake build
   ```
2. **Add a `build.sh` script** at the repo root that validates the working directory and invokes the correct Lake commands.
3. **Update all documentation** (`README.md`, `PIRTM-README-Claim-Table.md`, ADR docs) to use the corrected path.

## Consequences

- New contributors can build the project without directory confusion.
- The documented quick-start path matches the physical layout.
- CI documentation and contributor documentation are consistent.
