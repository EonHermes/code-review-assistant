.PHONY: all build test clean install frontend run help

# Build the Rust backend
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -rf frontend/dist

# Install dependencies (frontend)
frontend:
	cd frontend && npm install

# Build frontend
frontend-build:
	cd frontend && npm run build

# Run backend with default test project
run:
	target/release/automated-document-synthesizer --path example-project --output docs/

# Run with custom path
run-custom:
	target/release/automated-document-synthesizer --path $(path) --output $(output)

# Install the tool globally
install:
	cargo install --path .

# Build everything
all: build frontend-build

# Show help
help:
	@echo "Available commands:"
	@echo "  make build       - Build Rust backend"
	@echo "  make test        - Run tests"
	@echo "  make frontend    - Install frontend dependencies"
	@echo "  make frontend-build - Build React frontend"
	@echo "  make run         - Run on example project"
	@echo "  make all         - Build both backend and frontend"
	@echo "  make install     - Install globally"
	@echo "  make clean       - Clean build artifacts"
