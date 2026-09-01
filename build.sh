#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ ! -f "lakefile.toml" ]]; then
  echo "ERROR: lakefile.toml not found in $SCRIPT_DIR"
  echo "Please run this script from the PiLang repository root."
  exit 1
fi

echo "Building PiLang Lean kernel..."
lake build

echo "Running Lean tests..."
lake test

echo "Build and test complete."
