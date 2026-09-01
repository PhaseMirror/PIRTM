/-!
# ADR Foundations Examples

Concrete ADR instances for the PIRTM project, now including prime‑recursive foundation ADR.
-/
import .Core .Proofs .PrimeRecursive
open PIRTM.ADR

/-- Example ADR: Foundry Component Integration (ID 31) -/
def foundryIntegration : ADR := {
  id := 31,
  title := "Foundry Component Integration",
  status := ADRStatus.Accepted,
  context := "PIRTM requires deterministic generation of legal artifacts. Foundry offers a proven component for template rendering.",
  decision := "Integrate Foundry as the canonical rendering backend for all ADR‑generated documents.",
  consequences := [
    "All document pipelines must call `Foundry.render`.",
    "Deprecate legacy renderer in `legacy/`.",
    "Version‑lock Foundry to v2.3.1."
  ],
  supersedes := none,
  links := [
    {uri := "https://github.com/pirtm/foundry", label := "Foundry Repo"},
    {uri := "git::abcd1234", label := "Commit introducing integration"}
  ]
}

/-- Placeholder ADR 1 (ID 32) -/
def placeholderADR1 : ADR := {
  id := 32,
  title := "Placeholder ADR 1",
  status := ADRStatus.Proposed,
  context := "Placeholder context",
  decision := "Placeholder decision",
  consequences := [],
  supersedes := none,
  links := []
}

/-- Placeholder ADR 2 (ID 33) -/
def placeholderADR2 : ADR := {
  id := 33,
  title := "Placeholder ADR 2",
  status := ADRStatus.Proposed,
  context := "Placeholder context",
  decision := "Placeholder decision",
  consequences := [],
  supersedes := none,
  links := []
}

/-- ADR for Prime Recursive Foundations of Existence (ID 34) -/
def primeRecursiveFoundations : ADR := {
  id := 34,
  title := "Prime Recursive Foundations of Existence",
  status := ADRStatus.Proposed,
  context := "Introduce prime‑recursive witness constructions to provide constructive existence proofs.",
  decision := "Adopt the PrimeRecursive module as the canonical approach for encoding existential witnesses.",
  consequences := [
    "All future existence proofs must be expressed via `existsPrimeRecursive`.",
    "Provide library lemmas for extracting witnesses from `PrimeWitness`.",
    "Document the pattern in ADR‑032."
  ],
  supersedes := none,
  links := [{uri := "../docs/adr/ADR-032-Prime-Recursive-Foundations-of-Existence.md", label := "ADR‑032 Document"}]
}
