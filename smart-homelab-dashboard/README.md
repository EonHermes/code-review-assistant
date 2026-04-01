# Automated Document Synthesizer (EON-003)

A powerful tool that automatically generates comprehensive documentation from code repositories by analyzing structure, dependencies, comments, and Git history.

## Features

- **Multi-language support**: Rust (Cargo), TypeScript/JavaScript (npm), Python (poetry/setuptools)
- **AST-based analysis**: Deep parsing of source code to extract modules, types, functions, and documentation comments
- **Dependency graphs**: Generate interactive Graphviz architecture diagrams
- **API documentation**: Auto-generate OpenAPI specs from route annotations/comments
- **Changelog generation**: Create formatted changelogs from Git commit history with conventional commits support
- **Cross-reference detection**: Identify relationships between modules and components
- **Export formats**: Markdown, HTML, JSON
- **Fast**: Rust WASM core for parallel analysis

## Tech Stack

**Backend**: Rust (clap, syn, walkdir, petgraph, graphviz, git2, serde, anyhow)
**Frontend**: React + TypeScript + Vite + Tailwind + ReactFlow (for graph visualization)
**Testing**: Rust integration tests, Jest for frontend

## Quick Start

```bash
# Clone and build
git clone https://github.com/EonHermes/automated-document-synthesizer.git
cd automated-document-synthesizer
cargo build --release

# Analyze a project
cargo run -- --path ../my-project --output docs/

# Start the frontend dev server
cd frontend
npm install
npm run dev
```

## Configuration

Create a `docs.yaml` in your project root:

```yaml
exclude:
  - target
  - node_modules
  - .git
include:
  - src
  - lib
api_detection:
  auto: true
  frameworks:
    - actix-web
    - rocket
    - axum
    - express
    - fastapi
```

## Examples

The tool generates:
- `architecture.png` - dependency graph
- `modules.md` - module structure with functions and types
- `api.json` - OpenAPI 3.0 specification
- `changelog.md` - conventional commits formatted changelog
- `index.html` - interactive documentation viewer

## License

MIT
