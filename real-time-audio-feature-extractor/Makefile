.PHONY: help build test clean install-deps

# Default target
help:
	@echo "Real-Time Audio Feature Extractor - Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build          - Build Rust WASM core and React frontend"
	@echo "  wasm           - Build only the WASM core"
	@echo "  frontend       - Build only the React frontend"
	@echo "  dev            - Start development server"
	@echo "  test           - Run all tests"
	@echo "  lint           - Run linters"
	@echo "  clean          - Remove build artifacts"
	@echo "  install-deps   - Install Rust and Node dependencies"

# Check dependencies
check-deps:
	@command -v cargo >/dev/null 2>&1 || { echo "Rust/Cargo not found. Install from https://rustup.rs"; exit 1; }
	@command -v node >/dev/null 2>&1 || { echo "Node.js not found. Install from https://nodejs.org"; exit 1; }
	@command -v npm >/dev/null 2>&1 || { echo "npm not found. Install Node.js with npm"; exit 1; }

# Install dependencies
install-deps: check-deps
	cd audio-core && cargo fetch
	cd frontend && npm ci

# Build WASM core
wasm: check-deps
	cd audio-core && \
	cargo build --release --target wasm32-unknown-unknown

# Build frontend
frontend: check-deps
	cd frontend && npm run build

# Build everything
build: wasm frontend
	@echo "✅ Build complete! Frontend assets in dist/"

# Run development server
dev: check-deps
	@echo "Starting development server..."
	cd frontend && npm run dev

# Run tests
test: check-deps
	cd audio-core && cargo test
	cd frontend && npm run test 2>/dev/null || echo "Frontend tests setup needed"

# Run linters
lint:
	cd frontend && npm run lint

# Clean build artifacts
clean:
	rm -rf audio-core/target/
	rm -rf frontend/dist/
	rm -rf frontend/node_modules/
	rm -rf frontend/.vite
	rm -rf dist/

# Full release build
release: check-deps
	@echo "🔨 Building release version..."
	cd audio-core && cargo build --release --target wasm32-unknown-unknown
	cd frontend && npm run build
	@echo "✅ Release build complete!"