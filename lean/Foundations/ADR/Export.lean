/-!
# ADR Foundations Export

Utility to generate markdown documentation from ADR records.
-/
import .Core .Examples
open System (FilePath IO)
open PIRTM.ADR

def exportADR (a : ADR) (outPath : FilePath) : IO Unit := do
  let content := s!"""
# {a.title}

- **ID**: {a.id}
- **Status**: {a.status}
- **Context**: {a.context}
- **Decision**: {a.decision}
- **Consequences**:
  {a.consequences.map (fun c => "- " ++ c) |>.joinToString "\n"}
- **Supersedes**: {a.supersedes.map (fun id => toString id) |>.getD "none"}
- **Links**:
  {a.links.map (fun l => "- [" ++ l.label ++ "](" ++ l.uri ++ ")") |>.joinToString "\n"}
"""
  IO.FS.writeFile outPath content

def exportAll : IO Unit := do
  let outDir := "docs/adr/generated"
  IO.FS.createDirAll outDir
  exportADR foundryIntegration (outDir ++ "/ADR-031-Foundry-Component-Integration.md")
  exportADR placeholderADR1 (outDir ++ "/ADR-032-Placeholder-1.md")
  exportADR placeholderADR2 (outDir ++ "/ADR-033-Placeholder-2.md")
