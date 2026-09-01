# ADR-029: Replace Dummy Justification with Real Entailment Checker

- **Status**: Resolved
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01
- **Resolved**: 2026-09-01

## Resolution

1. **`fromADR` reimplemented** in `lean/ADR/Proofs.lean`:
   - Changed return type from `Justification` to `List Justification`.
   - Each justification now pairs the shared premise set (non-empty lines from `context` and `decision`) with the actual consequence string from `a.consequences`.
   - Eliminated the hardcoded `"dummy"` conclusion.
2. **Entailment checker is now meaningful** — `entails` checks whether each actual consequence text appears in the premise set, rather than always failing on a placeholder.
3. **Existing proofs preserved** — `accepted_adr_consequences_nonempty` and `accepted_adr_explicitly_justified` were already proven without `sorry`; no changes needed.
4. **`Test.lean` explicit justifications** — `adr1001Justifications` already uses real consequence strings, demonstrating the intended usage pattern.

## Validation

```bash
$ lake build ADR.Proofs
Build completed successfully (3 jobs).
```

## Context

`lean/ADR/Proofs.lean` defines:

```lean
def fromADR (a : ADR) : Justification :=
  let ctxLines := ...
  let decLines := ...
  ⟨ctxLines ++ decLines, "dummy"⟩
```

The conclusion is hardcoded to `"dummy"`, which means `entails` always returns `false` for any real consequence. This makes `ConsequencesEntailed` and `JustifiedWith` vacuously false for accepted ADRs.

## Hidden Assumption

That a textual substring check is sufficient for consequence entailment. In reality, the placeholder `"dummy"` prevents any ADR from passing the entailment gate.

## Decision

1. **Replace `"dummy"` with a real consequence** from the ADR, or
2. **Redesign `Justification`** to pair each premise set with its corresponding conclusion index into `a.consequences`.
3. **Prove `accepted_adr_consequences_nonempty`** and `accepted_adr_explicitly_justified` without `sorry`.

## Consequences

- Accepted ADRs must have consequences that are textually derivable from their context and decision.
- The entailment checker catches malformed ADRs at construction time.
- Proofs of consequence entailment are machine-checked.
