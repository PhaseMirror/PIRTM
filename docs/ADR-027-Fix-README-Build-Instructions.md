# ADR-027: Fix README Build Instructions

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Documentation
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`build.sh` script added** at repo root (`PiLang/build.sh`):
   - Validates that `lakefile.toml` exists in the working directory.
   - Invokes `lake build` and `lake test`.
   - Provides clear error messages if run from the wrong directory.
2. **README.md updated** — Quick Start section now references `./build.sh` instead of raw `lake build` commands.
3. **Documentation corrected**:
   - `docs/PIRTM-README-Claim-Table.md` — Updated Lean Axiom-Clean Core, MLIR Lowering Soundness, Admissibility Validator, and Toolchain Lock entries to reflect resolved status (ADR-018, ADR-021, ADR-024).
   - `artifacts/PIRTM-README-Claim-Table.md` — Fixed incorrect `lean/lakefile.toml` reference to `lakefile.toml`.

## Validation

```bash
$ ./build.sh
Building PiLang Lean kernel...
...
Build and test complete.
```

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
