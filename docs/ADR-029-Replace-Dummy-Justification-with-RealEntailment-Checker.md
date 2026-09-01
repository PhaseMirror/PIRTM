# ADR-029: Replace Dummy Justification with Real Entailment Checker

- **Status**: Proposed
- **Deciders**: Phase Mirror Governance, Formal Methods Engineering
- **Date**: 2026-09-01

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
