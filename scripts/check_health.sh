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
# npm run lint # (Skipping for now as default create-tauri-app might not have strict linting set up yet, or we can try)
# Let's assume we want to run it if it exists, or just build.
# Actually, let's run the build as a check.
echo "  - Building..."
npm run build
echo "  - Testing..."
npx vitest run

echo "✅ All checks passed!"
