#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="${HOME}/.local/bin"

echo "=== PIRTM Local PC Installation & Environment Initialization ==="
echo "Target Binary Directory: ${BIN_DIR}"
mkdir -p "${BIN_DIR}"

# 1. Verify Rust environment
echo "[1/5] Verifying Rust environment..."
if ! command -v rustc &> /dev/null; then
    echo "ERROR: rustc not found in PATH. Please install Rust 1.81+ via rustup."
    exit 1
fi
echo "  Found rustc: $(rustc --version)"

# 2. Verify Lean environment
echo "[2/5] Verifying Lean environment..."
if ! command -v lean &> /dev/null; then
    echo "ERROR: lean not found in PATH. Please install elan/lean4."
    exit 1
fi
echo "  Found lean: $(lean --version)"

# 3. Build Rust Workspace
echo "[3/5] Compiling PIRTM Rust workspace binaries..."
cd "${SCRIPT_DIR}/rust"
cargo build --release --workspace

# 4. Install binaries into local PATH
echo "[4/5] Installing executables into ${BIN_DIR}..."
EXECUTABLES=(pirtm pirtm-tools pirtm-mcp pirtm-monitor pirtm-web-sdk adr-verifier lsp)
for exe in "${EXECUTABLES[@]}"; do
    if [[ -f "target/release/${exe}" ]]; then
        cp "target/release/${exe}" "${BIN_DIR}/${exe}"
        chmod +x "${BIN_DIR}/${exe}"
        echo "  Installed: ${BIN_DIR}/${exe}"
    fi
done

# Standardize PIRTM compiler binary name
if [[ -f "${BIN_DIR}/pirtm" ]]; then
    cp "${BIN_DIR}/pirtm" "${BIN_DIR}/pirtmc"
    echo "  Aliased: ${BIN_DIR}/pirtmc -> ${BIN_DIR}/pirtm"
fi

# Standardize PIRTM LSP binary name
if [[ -f "${BIN_DIR}/lsp" ]]; then
    cp "${BIN_DIR}/lsp" "${BIN_DIR}/pirtm-lsp"
    echo "  Aliased: ${BIN_DIR}/pirtm-lsp -> ${BIN_DIR}/lsp"
fi

# 5. Build and Verify Lean kernel
echo "[5/5] Building and verifying Lean kernel formal proofs..."
cd "${SCRIPT_DIR}"
./build.sh

echo ""
echo "=== PIRTM Installation Successful! ==="
echo "PIRTM platform binaries are installed at ${BIN_DIR}:"
echo "  - pirtm / pirtmc   : PIRTM Compiler & Execution CLI"
echo "  - pirtm-tools      : Evaluation & Verification Tools"
echo "  - pirtm-mcp        : Model Context Protocol Server"
echo "  - pirtm-lsp / lsp  : Language Server Protocol Server"
echo "  - adr-verifier     : ADR Compliance & Verification Gate"
echo ""
echo "You can now run 'pirtm --help' or begin programming with PIRTM!"
