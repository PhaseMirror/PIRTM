import Foundations.ADR.Core
import Foundations.ADR.Examples

/-!
# ADR Foundations Export

Utility to generate markdown documentation from ADR records.
-/
open System (FilePath)
open PIRTM.ADR

def exportADR (a : ADR) (outPath : FilePath) : IO Unit := do
  let consequencesStr := String.intercalate "\n" (a.consequences.map (fun c => "- " ++ c))
  let linksStr := String.intercalate "\n" (a.links.map (fun l => "- [" ++ l.label ++ "](" ++ l.uri ++ ")"))
  let supersedesStr := match a.supersedes with
    | some id => toString id
    | none => "none"
  let statusStr := match a.status with
    | ADRStatus.Proposed => "Proposed"
    | ADRStatus.Accepted => "Accepted"
    | ADRStatus.Deprecated => "Deprecated"
    | ADRStatus.Superseded => "Superseded"

  let content := s!"# {a.title}\n\n- **ID**: {a.id}\n- **Status**: {statusStr}\n- **Context**: {a.context}\n- **Decision**: {a.decision}\n- **Consequences**:\n{consequencesStr}\n- **Supersedes**: {supersedesStr}\n- **Links**:\n{linksStr}\n"
  IO.FS.writeFile outPath content

def exportAll : IO Unit := do
  let outDir := "docs/adr/generated"
  IO.FS.createDirAll outDir
  exportADR foundryIntegration (outDir ++ "/ADR-031-Foundry-Component-Integration.md")
  exportADR primeRecursiveFoundations (outDir ++ "/ADR-032-Prime-Recursive-Foundations-of-Existence.md")
  exportADR qmhesIntegration (outDir ++ "/ADR-033-QMHES-Integration.md")
  exportADR primeIndexedDialecticalSemantics (outDir ++ "/ADR-034-Prime-Indexed-Dialectical-Semantics.md")
  exportADR primeEncodedQuantumStates (outDir ++ "/ADR-035-Prime-Encoded-Quantum-States.md")
  exportADR primeStructuredTensorNetworkAutoencoder (outDir ++ "/ADR-036-Prime-Structured-Tensor-Network-Autoencoder.md")
  exportADR primeIndexedPhaseDissonance (outDir ++ "/ADR-037-Prime-Indexed-Phase-Dissonance.md")
  exportADR governanceManifold (outDir ++ "/ADR-038-Governance-Manifold-Fail-Closed-Control.md")
  exportADR cognitiveEconomy (outDir ++ "/ADR-039-Cognitive-Economy-Ethical-Projection.md")
  exportADR echoBraidFeedback (outDir ++ "/ADR-040-EchoBraid-Quantum-Feedback.md")
  exportADR multiplicityFloerOperator (outDir ++ "/ADR-041-Multiplicity-Floer-Differential-Operator.md")
  exportADR primeConstitutionalOrder (outDir ++ "/ADR-042-Prime-Constitutional-Order-CSL.md")
  exportADR lawfulRecursionLicense (outDir ++ "/ADR-043-Lawful-Recursion-License.md")
  exportADR registryReconciliation (outDir ++ "/ADR-044-Comprehensive-Registry-Reconciliation.md")
  exportADR uiUxIntegration (outDir ++ "/ADR-045-UI-UX-Integration-PIRTM.md")
  exportADR goldilocksFieldIntegration (outDir ++ "/ADR-046-The-Goldilocks-prime-field.md")
