#!/bin/bash
set -e

echo "🦀 Checking Rust Backend..."
cd src-tauri
echo "  - Formatting..."
cargo fmt --check
echo "  - Clippy..."
cargo clippy -- -D warnings
echo "  - Testing..."
cargo test
cd ..

echo "⚛️ Checking React Frontend..."
echo "  - Linting..."
npm run lint
echo "  - Building..."
npm run build
echo "  - Testing..."
npx vitest run

echo "✅ All checks passed!"
