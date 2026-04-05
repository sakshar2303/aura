# Contributing to Aura

Thanks for your interest in contributing to Aura! This guide will help you get started.

## Quick Setup

```bash
git clone https://github.com/360Labs-dev/aura.git
cd aura
cargo build
cargo test    # 147 tests should pass
```

## Project Structure

```
crates/
├── aura-core/          # Compiler core (lexer, parser, types, HIR, LIR)
│   ├── src/lexer/      # Tokenization + indentation handling
│   ├── src/parser/     # Recursive descent parser → AST
│   ├── src/semantic/   # Type checking, security types, error poisoning
│   ├── src/hir/        # High-level IR + builder
│   ├── src/lir/        # Low-level IR
│   ├── src/types/      # Type system (structural, union, generics)
│   ├── src/design/     # Design token resolution
│   ├── src/errors/     # Error types + confidence-scored fixes
│   ├── src/cache.rs    # Incremental compilation
│   ├── src/project.rs  # Multi-file project model
│   ├── src/sourcemap.rs # Source map v3 generation
│   ├── src/treeshake.rs # Dead code elimination
│   └── src/declarations.rs # .d.aura generation
├── aura-cli/           # CLI binary (build, run, fmt, etc.)
├── aura-backend-web/   # HTML/CSS/JS codegen
├── aura-backend-swift/ # SwiftUI codegen
├── aura-backend-compose/ # Jetpack Compose codegen
├── aura-agent/         # AI Agent API (JSON-RPC)
├── aura-backend-win/   # WinUI codegen (planned)
├── aura-backend-tui/   # Terminal UI codegen (planned)
├── aura-lsp/           # LSP server (planned)
├── aura-pkg/           # Package manager (planned)
└── aura-benchmark/     # Benchmark runner
```

## How to Contribute

### 1. Find something to work on

- Check [open issues](https://github.com/360Labs-dev/aura/issues)
- Look for `good first issue` labels
- Check the "Areas Needing Help" section below

### 2. Make your changes

- Create a branch: `git checkout -b my-feature`
- Write code following the patterns in existing modules
- Add tests for new functionality

### 3. Test your changes

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test -p aura-core --lib parser::tests::test_parse_minimal

# Run conformance tests (all backends)
cargo test -p aura-core --test conformance

# Run snapshot tests
cargo test -p aura-core --test snapshots

# Run benchmarks
cargo run --release --bin aura-bench

# If you changed codegen, update snapshots:
cargo insta test --accept -p aura-core --test snapshots
```

### 4. Submit a PR

- Push your branch and create a Pull Request
- CI will run automatically (tests, lint, build, benchmarks)
- Describe what you changed and why

## Coding Standards

### Rust
- `cargo fmt` before committing
- `cargo clippy` should pass with no warnings
- All public APIs have doc comments
- Error types are specific (use `thiserror`, not `anyhow`)
- Tests follow `test_{component}_{scenario}_{expected}` naming

### Commit Messages
```
area: short description

Areas: core, parser, types, hir, lir, backend-web, backend-swift,
       backend-compose, cli, lsp, agent, pkg, spec, docs, tests
```

### Adding a New Backend

1. Create `crates/aura-backend-{name}/`
2. Implement codegen that consumes HIR nodes
3. Add conformance tests (same .aura files, verify output)
4. Add snapshot baselines via `cargo insta test --accept`
5. Wire into CLI (`aura build --target {name}`)

### Adding a New Language Feature

1. Update `spec/language.md` with the feature spec
2. Add tokens to `lexer/tokens.rs` if needed
3. Add AST nodes to `ast/mod.rs`
4. Add parser rules to `parser/mod.rs`
5. Add type checking to `semantic/scope.rs`
6. Add HIR nodes to `hir/nodes.rs` and builder
7. Update ALL backends (web, swift, compose)
8. Add conformance tests
9. Update snapshot baselines

## Areas Needing Help

### High Impact
- **WinUI backend** — Windows native codegen (crates/aura-backend-win/)
- **Terminal UI backend** — CLI app codegen (crates/aura-backend-tui/)
- **LSP server** — Full Language Server Protocol (crates/aura-lsp/)
- **Package manager** — Dependency resolution, registry (crates/aura-pkg/)

### Medium Impact
- **More conformance tests** — We have 10, TypeScript has 9000+
- **Error messages** — Improve clarity and suggestions
- **Documentation** — Tutorials, guides, API docs
- **IDE plugins** — JetBrains, Neovim, Sublime

### Good First Issues
- Add more icon-to-emoji mappings in web backend
- Add more sketch templates (e-commerce, social media, dashboard)
- Improve CLI error messages with color output
- Add `aura check` command (type-check without compiling)
- Add `--verbose` flag to show compilation phases

## Running the Full CI Locally

```bash
# Everything CI does:
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo build --release
cargo run --release --bin aura-bench
for f in examples/*.aura; do
  ./target/release/aura build "$f" --target web -o "/tmp/$(basename $f .aura)"
done
```

## Questions?

Open an issue or start a discussion on GitHub.
