# Installation Guide

This guide provides detailed, platform-specific instructions for setting up the PIRTM/MOC development environment.

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Platform-Specific Setup](#platform-specific-setup)
   - [Ubuntu / Debian](#ubuntu--debian)
   - [macOS](#macos)
   - [Windows (WSL2)](#windows-wsl2)
3. [Verifying Installation](#verifying-installation)
4. [Building the Project](#building-the-project)
5. [Troubleshooting](#troubleshooting)

---

## System Requirements

| Component | Minimum Version | Recommended |
|-----------|----------------|-------------|
| **OS** | Ubuntu 22.04, macOS 13, or WSL2 | Ubuntu 24.04 LTS |
| **Rust** | 1.81.0 | latest stable |
| **Lean 4** | v4.33.0-rc2 | v4.33.0-rc2 (pinned) |
| **LLVM** | 17 (`mlir-translate`, `llc`, `clang`) | 18+ |
| **Git** | 2.40 | latest |
| **RAM** | 8 GB | 16 GB |
| **Disk** | 10 GB free | 20 GB free |

---

## Platform-Specific Setup

### Ubuntu / Debian

```bash
# 1. Install system dependencies
sudo apt update
sudo apt install -y build-essential git curl pkg-config libssl-dev llvm-17 llvm-17-dev clang-17 lld-17

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# 3. Install elan (Lean version manager)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
source $HOME/.elan/env

# 4. Install pinned Lean toolchain
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version

# 5. Clone the repository
git clone https://github.com/PhaseMirror/PiLang.git
cd PiLang
```

### macOS

```bash
# 1. Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Install dependencies
brew install rustup llvm git

# 3. Install Rust
rustup-init -y
source $HOME/.cargo/env

# 4. Install elan
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y --default-toolchain none
source $HOME/.elan/env

# 5. Install pinned Lean toolchain
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version

# 6. Clone the repository
git clone https://github.com/PhaseMirror/PiLang.git
cd PiLang
```

### Windows (WSL2)

PIRTM/MOC requires a Unix-like environment. Use WSL2:

```bash
# 1. Install WSL2 (from PowerShell as Administrator)
wsl --install -d Ubuntu-24.04

# 2. Inside WSL2, follow Ubuntu instructions above
```

---

## Verifying Installation

### Check Rust

```bash
rustc --version
# Expected: rustc 1.81.0 (or higher)

cargo --version
# Expected: cargo 1.81.0 (or higher)
```

### Check Lean

```bash
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version
# Expected: Lean (v4.33.0-rc2)
```

### Check LLVM

```bash
mlir-translate --version
llc --version
clang --version
```

### Check Git

```bash
git --version
# Expected: git version 2.40.0 (or higher)
```

---

## Building the Project

### 1. Build Rust Workspace

```bash
cd rust
cargo build --workspace
```

### 2. Build Lean Kernel

```bash
cd ..
./build.sh
```

Expected output:
```
Building PiLang Lean kernel...
...
Build and test complete.
```

### 3. Run Tests

```bash
# Rust tests
cd rust
cargo test --workspace

# Lean build verification
cd ..
lake build --rehash
```

### 4. Run Clippy (Rust Linting)

```bash
cd rust
cargo clippy --workspace -- -D warnings
```

---

## Troubleshooting

### `mlir-translate` not found

**Ubuntu/Debian:**
```bash
sudo apt install llvm-17 llvm-17-dev
export PATH="/usr/lib/llvm-17/bin:$PATH"
```

**macOS:**
```bash
brew install llvm@17
export PATH="/opt/homebrew/opt/llvm@17/bin:$PATH"
```

### Lean toolchain mismatch

```bash
# Ensure lean-toolchain and lake-manifest.json agree
cat lean-toolchain
cat lake-manifest.json | grep toolchain

# Reinstall toolchain if needed
elan run leanprover/lean4:v4.33.0-rc2 -- lean --version
```

### `lake build` fails with "file not found"

Ensure you are in the `PiLang/` root directory (not `lean/`):
```bash
cd /path/to/PiLang
ls lakefile.toml  # Should exist
./build.sh
```

### `cargo build` fails with linking errors

Ensure you have the C linker installed:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

### Out of memory during Lean build

Increase memory allocation or build individual targets:
```bash
lake build ADR.Proofs
lake build ADR.ZenoController
```

---

## Next Steps

- Read [USER_GUIDE.md](USER_GUIDE.md) for compilation and runtime usage
- Review [CONTRIBUTING.md](CONTRIBUTING.md) for development workflow
- Check [docs/](docs/) for Architecture Decision Records
- See [README.md](README.md) for project overview and quick start

---

*Last Updated: 2026-09-01*
