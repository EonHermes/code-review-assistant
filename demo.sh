#!/usr/bin/env bash
# Automated Document Synthesizer - Demo Script
# Usage: ./demo.sh [project-path]

set -e

PROJECT_PATH="${1:-.}"
OUTPUT_DIR="generated-docs"

echo "🗃️  Automated Document Synthesizer - Demo"
echo "=========================================="
echo "Project: $PROJECT_PATH"
echo "Output: $OUTPUT_DIR"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Please run from the project root."
    exit 1
fi

# Build the tool
echo "🔨 Building..."
cargo build --release

# Run analysis on the example project
echo "📊 Analyzing project..."
target/release/automated-document-synthesizer \
    --path "$PROJECT_PATH" \
    --output "$OUTPUT_DIR" \
    --format markdown \
    --changelog

# Show generated files
echo ""
echo "✅ Documentation generated!"
echo "📁 Output files:"
ls -la "$OUTPUT_DIR"

echo ""
echo "To view the documentation:"
echo "  - Open $OUTPUT_DIR/index.md in a markdown viewer"
echo "  - Or use 'make frontend-build' and open frontend/dist/index.html"
echo ""
echo "To analyze a different project:"
echo "  cargo run -- --path /path/to/project --output docs/"
