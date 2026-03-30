# Contributing to Automated Document Synthesizer

Thank you for your interest in improving Automated Document Synthesizer! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

- Check existing issues before creating a new one
- Include detailed steps to reproduce
- Provide system information (OS, Rust version, etc.)
- Attach relevant logs or screenshots

### Suggesting Features

- Open an issue to discuss the feature before implementing
- Explain the use case and why the feature would be valuable
- Consider if it fits the project scope (documentation generation, not a full IDE)

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with clear, descriptive commits
4. Run tests (`cargo test` and `npm test` for frontend)
5. Ensure code compiles without warnings
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request with a clear description

### Development Setup

```bash
# Clone and setup
git clone https://github.com/EonHermes/automated-document-synthesizer.git
cd automated-document-synthesizer

# Install Rust toolchain (stable)
rustup update stable

# Build backend
cargo build

# Setup frontend
cd frontend
npm install
npm run dev
```

### Code Style

- Rust: Follow standard Rust conventions, run `cargo fmt` and `cargo clippy`
- TypeScript/React: Follow the provided ESLint config
- Write documentation for public APIs and components
- Keep functions small and focused (single responsibility)
- Prefer clear names over comments, but document complex algorithms

### Testing

- Write unit tests for core logic
- Add integration tests for parsers using sample files in `tests/fixtures/`
- Frontend: Jest + React Testing Library for components
- Ensure test coverage doesn't decrease

### Architecture Guidelines

- Keep parsers modular and language-agnostic through the `LanguageParser` trait
- The analyzer should be language-agnostic, delegating to specific parsers
- Output generators should be pluggable (Markdown, HTML, JSON)
- Configuration should be YAML-based with sensible defaults

## Project Structure

```
├── src/
│   ├── analyzer.rs      # Main scanning and coordination
│   ├── config.rs        # Configuration loading/validation
│   ├── errors.rs        # Error types and handling
│   ├── graph.rs         # Dependency graph construction
│   ├── output.rs        # Documentation generation
│   ├── parser_trait.rs  # Language parser abstraction
│   ├── rust_parser.rs   # Rust AST parser
│   ├── ts_parser.rs     # TypeScript parser (stub)
│   ├── python_parser.rs # Python parser (stub)
│   ├── git_analyzer.rs  # Git changelog generation
│   └── types.rs         # Shared data structures
├── frontend/
│   ├── src/
│   │   ├── App.tsx      # Main React application
│   │   ├── App.css      # Tailwind CSS styles
│   │   └── main.tsx     # Entry point
├── tests/
│   ├── fixtures/        # Sample projects for testing
│   └── integration/     # Integration tests
├── docs/                # Generated documentation examples
├── example-project/     # Sample Rust project for demo
└── Cargo.toml          # Rust dependencies
```

## Roadmap

- [ ] Complete Rust parser (module, function, struct, enum, trait extraction)
- [ ] Add TypeScript parser using `ts-morph` or `swc`
- [ ] Add Python parser using `rustpython-parser` or `python3-sys`
- [ ] Implement Graphviz DOT generation with `petgraph`
- [ ] Add ReactFlow integration for frontend graph visualization
- [ ] Implement git2 changelog with conventional commits
- [ ] Add configurable templates
- [ ] WASM support for browser-based analysis

## Questions?

Open an issue or reach out to @EonHermes.
