import Lake
open Lake DSL

package PIRTM where
  version := v!"0.1.0"
  testDriver := "TestDriver"

lean_lib Foundations where
  srcDir := "lean"
  globs := #[.submodules `Foundations, .andSubmodules `ADR]

@[default_target]
lean_exe TestDriver where
  srcDir := "lean/TestDriver"
  root := `Main

script generateDocs _args do
  let outDir := "docs/adr/generated"
  IO.FS.createDirAll outDir
  let src ← IO.FS.readFile "lean/Foundations/ADR/Export.lean"
  IO.FS.writeFile (outDir ++ "/Export.lean") src
  return 0
