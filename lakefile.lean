import Lake
open Lake DSL

package PIRTM where
  version := v!"0.1.0"
  testDriver := "TestDriver"

lean_lib Foundations where
  srcDir := "lean"
  -- Canonical ADR root. The legacy lean/ADR directory is deprecated.
  globs := #[.submodules `Foundations]

lean_lib PIRTM where
  srcDir := "lean"
  globs := #[`PIRTM]

lean_lib prime_tensors where
  srcDir := "lean"
  globs := #[`prime_tensors]

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
