# crates.io Publishing & Release Automation for PIRTM v3.0.0

This checklist guides the automated publication of all 21 workspace crates to `crates.io`.

---

## 📦 Dependency Order for Publishing

To satisfy `crates.io` dependency resolution, crates must be published in topological order:

1. `pirtm-kernel-lexer`
2. `pirtm-app-lexer`
3. `pirtm-parser`
4. `pirtm-goldilocks`
5. `pirtm-invariants`
6. `pirtm-stdlib`
7. `telemetry-recorder`
8. `pirtm-telemetry`
9. `pirtm-tensor`
10. `pirtm-registry`
11. `pirtm-monitor`
12. `pirtm-mlir`
13. `pirtm-dist`
14. `pirtm-engine`
15. `pirtm-orchestration`
16. `pirtm-mcp`
17. `pirtm-compiler`
18. `pirtm-tools`
19. `pirtm-web-sdk`
20. `adr_rust`
21. `adr-verifier`

---

## 🛠️ Publication Command Sequence

```bash
#!/usr/bin/env bash
set -euo pipefail

CRATES=(
  "pirtm-kernel-lexer"
  "pirtm-app-lexer"
  "pirtm-parser"
  "pirtm-goldilocks"
  "pirtm-invariants"
  "pirtm-stdlib"
  "telemetry-recorder"
  "pirtm-telemetry"
  "pirtm-tensor"
  "pirtm-registry"
  "pirtm-monitor"
  "pirtm-mlir"
  "pirtm-dist"
  "pirtm-engine"
  "pirtm-orchestration"
  "pirtm-mcp"
  "pirtm-compiler"
  "pirtm-tools"
  "pirtm-web-sdk"
  "adr_rust"
  "adr-verifier"
)

for crate in "${CRATES[@]}"; do
  echo "Publishing $crate to crates.io..."
  cargo publish --package "$crate" --allow-dirty
  sleep 10
done
```
