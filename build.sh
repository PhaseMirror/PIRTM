#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ ! -f "lakefile.toml" ]] && [[ ! -f "lakefile.lean" ]]; then
  echo "ERROR: lakefile.lean or lakefile.toml not found in $SCRIPT_DIR"
  echo "Please run this script from the PiLang repository root."
  exit 1
fi

echo "Building all Lean targets..."
lake build --rehash Foundations PIRTM prime_tensors TestDriver

echo "Running Lean tests..."
lake test

echo "Running Rust workspace tests..."
cargo test --workspace --all-targets

echo "Checking Lean proof debt..."
if grep -RIn "sorry" lean; then
  echo "ERROR: executable or documented sorry found in lean/" >&2
  exit 1
fi

echo "Build and verification complete."
