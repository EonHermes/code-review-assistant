# EON-017: Automated Documentation Generator

An intelligent documentation system that parses Rust and TypeScript codebases to generate comprehensive, up-to-date documentation with examples, cross-references, and architecture diagrams.

## Features

- **Rust code parsing** using `syn` and `quote` crates
- **TypeScript/JavaScript parsing** with Tree-sitter
- **Automatic example extraction** from test files
- **Cross-reference linking** between modules and types
- **Mermaid.js diagram generation** for architecture and data flows
- **CI/CD integration** ready (GitHub Actions)
- **CLI and library** usage modes
- **Markdown output** with beautiful formatting

## Tech Stack

- **Backend**: Rust (async, clap, syn, quote, tree-sitter)
- **Frontend**: TypeScript + React (dashboard for viewing docs)
- **Database**: None (static file generation)
- **CI**: GitHub Actions
- **Diagrams**: Mermaid.js

## Project Structure

```
eon-doc-generator/
├── crates/
│   ├── doc-core/          # Core documentation engine (Rust)
│   ├── rust-parser/       # Rust-specific parser
│   ├── ts-parser/         # TypeScript parser
│   └── mermaid-gen/       #Diagram generation
├── dashboard/
│   ├── src/               # React TypeScript frontend
│   └── package.json
├── examples/
│   ├── sample-rust/       # Example Rust project
│   └── sample-ts/         # Example TypeScript project
├── .github/
│   └── workflows/
│       └── docs.yml       # CI workflow
├── README.md
└── Cargo.toml (workspace)
```

## Quick Start

```bash
# Install
cargo install eon-doc-generator

# Generate docs for a Rust project
eon-docgen --lang rust --path ./my-crate --output ./docs

# Generate docs for TypeScript
eon-docgen --lang typescript --path ./my-ts --output ./docs

# Watch mode (regenerate on changes)
eon-docgen --watch --path ./src --output ./docs
```

## Architecture

The system uses a modular pipeline:
1. **Parse** source files into AST
2. **Analyze** structure (modules, functions, types, docs)
3. **Extract** examples from tests
4. **Generate** cross-references and diagrams
5. **Render** to Markdown with Mermaid.js
6. **Serve** via static file or dashboard

## Development

```bash
# Build all workspace crates
cargo build --workspace

# Run tests
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## License

MIT © 2026 EonHermes