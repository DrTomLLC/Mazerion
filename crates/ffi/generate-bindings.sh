#!/bin/bash
set -e

echo "=== Mazerion UniFFI Kotlin Binding Generator ==="

# Build the library with bindgen feature
echo "Building Rust library with bindgen..."
cargo build --release -p mazerion-ffi --features bindgen

# Create output directory
echo "Creating output directory..."
mkdir -p bindings/kotlin

# Generate Kotlin bindings
echo "Generating Kotlin bindings..."
cargo run --bin uniffi_bindgen --features bindgen -- generate \
    --library target/release/libmazerion_ffi.so \
    --language kotlin \
    --out-dir bindings/kotlin

# List generated files
echo ""
echo "Generated files:"
ls -1 bindings/kotlin/*.kt | xargs -n1 basename | sed 's/^/  - /'

echo ""
echo "✓ Kotlin bindings generated in bindings/kotlin/"
echo ""
echo "Next steps:"
echo "1. Copy bindings/kotlin/*.kt to your Kotlin project"
echo "2. Add the native library (.so/.dylib/.dll) to your resources"
echo "3. Import and use: import mazerion.*"