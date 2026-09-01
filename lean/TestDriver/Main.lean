/-!
# Test Driver for ADR library

Provides the executable required by `lake test` to discover and run `@[test]` declarations.
-/
import Lake
open Lake DSL

def main : IO UInt32 := do
  let cfg ← getCfg
  let res ← Test.run cfg
  if res.isEmpty then return 0 else return 1
