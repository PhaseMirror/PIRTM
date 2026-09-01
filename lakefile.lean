import Lake
open Lake DSL

package PIRTM where
  name := "PIRTM"
  version := "0.1.0"
  testDriver := "TestDriver"

lean_lib ADR where
  srcDir := "lean/Foundations/ADR"

lean_lib MOC where
  srcDir := "lean/MOC"

lean_exe TestDriver where
  root := `TestDriver/Main

require std from git "https://github.com/leanprover/std4" @ "main"
require #fmt from git "https://github.com/leanprover/lean4fmt" @ "master"

@[test]
library_test "ADR.Test" where
  srcDir := "lean/Foundations/ADR"
  dependencies := #[«PIRTM.ADR»]

script generateDocs where
  script := do
    let outDir := "docs/adr/generated"
    IO.FS.createDirAll outDir
    let src ← IO.FS.readFile "lean/Foundations/ADR/Export.lean"
    IO.FS.writeFile (outDir ++ "/Export.lean") src
