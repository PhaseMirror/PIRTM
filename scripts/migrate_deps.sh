#!/bin/bash
# migrate_deps.sh – Copy essential crates from Foundry to PiLang workspace

set -e

PI_LANG_RUST="/home/citizen/Multiplicity/PiLang/rust"
FOUNDRY_RUST="/home/citizen/Multiplicity/Foundry/packages/rust"

# We map source dir to target dir
copy_crate() {
    local src_dir=$1
    local target_dir=$2
    local SRC="$FOUNDRY_RUST/$src_dir"
    local DST="$PI_LANG_RUST/$target_dir"
    
    if [ -d "$SRC" ]; then
        echo "Copying $src_dir to $target_dir..."
        cp -r "$SRC" "$DST"
    else
        echo "Warning: $src_dir not found in Foundry."
    fi
}

copy_crate "pirtm-lexer" "pirtm-lexer"
copy_crate "mlir" "pirtm-mlir"

# Update pirtm-parser/Cargo.toml
if [ -f "$PI_LANG_RUST/pirtm-parser/Cargo.toml" ]; then
    sed -i 's|path = "../lexer"|path = "../pirtm-lexer"|g' "$PI_LANG_RUST/pirtm-parser/Cargo.toml"
fi

# Update pirtm-compiler/Cargo.toml 
if [ -f "$PI_LANG_RUST/pirtm-compiler/Cargo.toml" ]; then
    sed -i 's|path = "../mlir"|path = "../pirtm-mlir"|g' "$PI_LANG_RUST/pirtm-compiler/Cargo.toml"
fi

echo "✅ Migration complete. Run 'cargo test' in the PiLang workspace to verify."
