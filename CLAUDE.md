# Orbit Code — Monorepo Root

## What This Is
Orbit Code is the terminal-based coding agent for the Orbit ecosystem. It provides a rich TUI for interacting with AI agents directly from the terminal. This is a fork of OpenAI's Codex CLI, rebuilt to connect to the Orbit backend.

**Parent project:** [Orbit](https://github.com/Recusive/Orbit) — AI-native desktop IDE
**This repo:** [Orbit Code](https://github.com/Recusive/Orbit-Code) — Terminal agent

## Repository Structure

| Directory | Purpose |
|-----------|---------|
| `codex-rs/` | **Primary codebase** — Rust implementation of the CLI, TUI, core engine, and all subsystems (50+ crates) |
| `codex-cli/` | npm package wrapper — thin JS launcher that resolves platform-specific Rust binaries |
| `sdk/` | Client SDKs (Python + TypeScript) for programmatic access |
| `shell-tool-mcp/` | MCP server that exposes shell tool capabilities |
| `scripts/` | Repo-wide utility scripts (release, install) |
| `docs/` | Documentation (contributing, install, config) |
| `tools/` | Developer tooling (argument-comment linting) |
| `patches/` | pnpm patch overrides for dependencies |
| `third_party/` | Vendored third-party code (Meriyah, WezTerm) |

## Key Files at Root

| File | Purpose |
|------|---------|
| `package.json` | pnpm workspace root — formatting scripts, engine requirements (Node ≥22) |
| `pnpm-workspace.yaml` | Workspace packages: `codex-cli`, `sdk/typescript`, `shell-tool-mcp` |
| `justfile` | Task runner for Rust development — build, test, format, lint, schema generation |
| `MODULE.bazel` | Bazel build configuration for remote builds |
| `flake.nix` | Nix development environment |
| `AGENTS.md` | Coding conventions for Rust code, TUI styling, test patterns |
| `CONTRIBUTING.md` | Contribution guidelines |

## Build Systems
- **Rust**: Cargo (primary local dev) + Bazel (CI/release builds)
- **TypeScript**: pnpm + esbuild
- **Task runner**: `just` (justfile in root, working dir set to `codex-rs/`)

## Common Commands
```bash
just codex          # Run Orbit Code from source
just test           # Run Rust tests (nextest)
just fmt            # Format Rust code
just fix            # Run clippy fixes
just write-config-schema    # Regenerate config JSON schema
just write-app-server-schema # Regenerate app-server protocol schemas
```

## Architecture Overview
Orbit Code is primarily a **Rust application** (`codex-rs/`) that:
1. Provides a terminal UI (TUI) built with `ratatui`
2. Connects to a backend API for AI model access
3. Executes tools (shell commands, file operations) in sandboxed environments
4. Manages sessions, conversations, and agent state
5. Exposes an app-server (JSON-RPC WebSocket) for IDE integrations
6. Provides an MCP server for Model Context Protocol integrations

## Orbit Ecosystem Context
- **Orbit** (Desktop) = Full GUI IDE with editor, browser, terminal, vault
- **Orbit Code** (Terminal) = Terminal-only agent with the same core engine
- Both share the AI agent architecture; this repo provides the TUI + backend integration layer
