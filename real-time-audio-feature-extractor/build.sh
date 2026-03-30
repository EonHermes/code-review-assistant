#!/usr/bin/env bash

set -e

echo "🔨 Building Real-Time Audio Feature Extractor"
echo "=============================================="

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Rust/Cargo not found - skipping WASM build"
    echo "To build WASM: install Rust from https://rustup.rs"
else
    echo "📦 Building Rust WASM core..."
    cd audio-core
    cargo build --release --target wasm32-unknown-unknown
    cd ..
    echo "✅ WASM core built (or would be if wasm target installed)"
fi

# Check for npm
if ! command -v npm &> /dev/null; then
    echo "⚠️  npm not found - skipping frontend build"
else
    echo "📦 Installing frontend dependencies..."
    cd frontend
    npm ci --silent || npm install
    echo "✅ Dependencies installed"
    echo "🎯 To run: npm run dev"
    cd ..
fi

echo ""
echo "✅ Project structure complete!"
echo ""
echo "Next steps:"
echo "  1. wasm-pack build --target web -p audio-core (in audio-core/)"
echo "  2. cp audio-core/pkg/* frontend/public/"
echo "  3. npm run build (in frontend/)"
echo ""