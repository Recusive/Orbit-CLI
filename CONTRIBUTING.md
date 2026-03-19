# Contributing to Orbit CLI

Thank you for your interest in contributing to Orbit CLI!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/Orbit-CLI.git`
3. Install prerequisites:
   - Rust 1.85+
   - Node.js 22+
   - pnpm 10+
   - `just` command runner (`cargo install just`)
   - `cargo-nextest` (`cargo install cargo-nextest`)
4. Build and test: `cd codex-rs && cargo fetch && just test`

## Development

### Running from Source

```bash
just codex
```

### Code Style

**Rust:**
- Run `just fmt` after making changes
- Run `just fix -p <crate>` to lint specific crates
- Keep modules under 500 LoC (800 max)
- Inline `format!` args when possible
- Collapse `if` statements per clippy rules
- Use method references over closures when possible
- Prefer exhaustive `match` statements over wildcards

**TypeScript:**
- Run `pnpm format` for formatting
- Strict TypeScript — no `any`

### Testing

```bash
# Run tests for a specific crate
cargo test -p codex-tui

# Run all tests
just test

# Snapshot tests (TUI)
cargo insta accept -p codex-tui
```

### Commit Guidelines

- Write clear, descriptive commit messages
- One logical change per commit
- Reference issues when applicable

## Pull Requests

1. Create a feature branch from `main`
2. Make focused, incremental changes
3. Ensure all tests pass
4. Run `just fmt` and `just fix`
5. Write a clear PR description explaining the "why"

## Reporting Issues

Open an issue at [github.com/Recusive/Orbit-CLI/issues](https://github.com/Recusive/Orbit-CLI/issues).

Include:
- Steps to reproduce
- Expected vs actual behavior
- OS and Rust version
- Relevant logs or screenshots

## Code of Conduct

Be respectful and constructive. We're building something together.

---

Built by [Recursive Labs](https://orbit.build)
