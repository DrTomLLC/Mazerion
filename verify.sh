#!/usr/bin/env bash
set -euo pipefail

echo "🔍 Mazerion Build Verification"
echo "================================"

echo ""
echo "1️⃣ Checking line limits..."
cargo run --bin line-guard

echo ""
echo "2️⃣ Running tests..."
cargo test --all-features

echo ""
echo "3️⃣ Checking formatting..."
cargo fmt --all -- --check

echo ""
echo "4️⃣ Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo ""
echo "5️⃣ Building all targets..."
cargo build --all-targets

echo ""
echo "6️⃣ Building with DB feature..."
cargo build --features db

echo ""
echo "✅ All checks passed!"
