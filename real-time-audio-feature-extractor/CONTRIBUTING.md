# Contributing to Real-Time Audio Feature Extractor

Thank you for your interest in contributing! This document outlines the development workflow and standards.

## 🚀 Quick Start

1. **Fork & clone** the repository
2. **Build dependencies**: Rust (stable), Node.js 18+, wasm-pack
3. **Run development server**: `make dev`
4. **Make changes** and **ensure tests pass**: `make test`
5. **Submit a PR** with clear description

## 🏗 Development Workflow

### Rust/WASM Core (`audio-core/`)

```bash
cd audio-core

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test --release

# Build WASM
wasm-pack build --target web --out-dir pkg
```

**Key files:**
- `src/lib.rs` - Main `AudioAnalyzer` struct and WASM bindings
- `src/tempo.rs` - BPM detection algorithm
- `src/chroma.rs` - Musical key detection
- `src/spectral.rs` - Spectral feature extraction

### Frontend (`frontend/`)

```bash
cd frontend

# Install dependencies
npm ci

# Development server (hot reload)
npm run dev

# Build for production
npm run build

# Type checking
npx tsc --noEmit

# Linting
npm run lint
```

**Key files:**
- `src/App.tsx` - Main React component with D3 visualizations
- `src/audioProcessor.ts` - WASM interface layer
- `src/types.ts` - TypeScript definitions

## 🧪 Testing Strategy

- **Unit tests**: Located in `audio-core/tests/` alongside modules
- **Integration tests**: Test whole-analyzer behavior
- **Frontend**: Manual testing via dev server; add Jest tests as needed

Run full test suite: `make test`

## 📏 Code Standards

### Rust
- Use `rustfmt` formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Prefer `Result` over panics in public APIs
- Document public functions with `///` doc comments
- Keep WASM exports minimal and ergonomic

### TypeScript/React
- Use functional components with hooks
- Strict TypeScript mode enabled
- No `any` types (except in very specific WASM interop)
- D3 visualizations should be responsive
- CSS in `index.css` with BEM-like class names where helpful

### Commits
- Write clear, concise commit messages
- Follow Conventional Commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`
- One logical change per commit

## 🎯 Feature Development

When adding new features:

1. **Rust side**: Add method to `AudioAnalyzer`, WASM-bind it, and write tests
2. **Frontend**: Add to `AudioFeatures` TS interface, process in `App.tsx`, create visualization
3. **Documentation**: Update README with new feature details
4. **Demo**: Ensure feature works in both mic and file upload modes

### Adding a New Feature

Example: Adding "spectral rolloff" (already done)
1. Implement algorithm in `src/spectral.rs`, update `SpectralFeatures`
2. Bind in `lib.rs` (already called in `analyze_chunk`)
3. Add to TS `SpectralFeatures` interface
4. Display in UI (bar chart already includes it)
5. Document in README

## 🔍 Code Review Checklist

- [ ] Rust code compiles without warnings
- [ ] Tests pass (unit + integration)
- [ ] WASM build succeeds (or at least doesn't regress)
- [ ] TypeScript compiles without errors
- [ ] Linting passes
- [ ] README updated (if new feature)
- [ ] No debug `console.log` left in (unless useful)
- [ ] Responsive design considered
- [ ] Performance evaluated (real-time < 10ms latency goal)

## 🐛 Reporting Issues

When filing issues, include:
- OS/browser (for frontend)
- Rust version (`rustc --version`)
- Node/npm version
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if UI-related
- Console errors/warnings

## 🤝 Community

- Be kind and constructive
- Focus on the code, not the person
- Celebrate good ideas and improvements!
- This is a learning project - quality > speed

---

Questions? Open an issue or reach out to [@EonHermes](https://github.com/EonHermes).

Happy coding! 🎵