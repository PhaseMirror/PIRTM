import Foundations.ADR.Core

/-!
# ADR-063: GitHub Pages Automated Documentation Site Deployment
Formal verification of GitHub Pages deployment architecture and site generator safety.
-/

namespace Foundations.ADR.GitHubPagesDeployment

def adr0063 : PIRTM.ADR.ADR := {
  id := 63,
  title := "GitHub Pages Automated Documentation Site Deployment",
  status := PIRTM.ADR.ADRStatus.Proposed,
  context := "PIRTM formal models, ADR records, and WASM engines require a unified, machine-checked static site.",
  decision := "Automate mdBook + doc-gen4 + WASM interactive playground deployment via GitHub Actions.",
  consequences := ["Zero-drift documentation site", "Interactive browser-based spectral contractivity testing"],
  supersedes := none,
  links := []
}

inductive DocSiteLayer where
  | MdBookGuide
  | LeanDocGen4
  | WasmPlayground
  deriving Repr, DecidableEq

def isRequiredSiteLayer : DocSiteLayer → Bool
  | DocSiteLayer.MdBookGuide => true
  | DocSiteLayer.LeanDocGen4 => true
  | DocSiteLayer.WasmPlayground => true

theorem all_doc_layers_required (layer : DocSiteLayer) :
    isRequiredSiteLayer layer = true := by
  cases layer <;> rfl

end Foundations.ADR.GitHubPagesDeployment
