<p align="center">
  <strong>Orbit CLI</strong>
</p>

<h1 align="center">Orbit CLI</h1>

<p align="center">
  <strong>The terminal agent for Orbit.</strong><br/>
  AI-powered coding agent that runs locally in your terminal. Built on the Ratatui TUI framework with a pluggable backend architecture.
</p>

<p align="center">
  <a href="https://github.com/Recusive/Orbit-CLI"><img src="https://img.shields.io/badge/Orbit_CLI-Terminal_Agent-24C8D8" alt="Orbit CLI" /></a>
  <img src="https://img.shields.io/badge/Rust-1.85-DEA584?logo=rust&logoColor=black" alt="Rust" />
  <img src="https://img.shields.io/badge/TypeScript-5.7-3178C6?logo=typescript&logoColor=white" alt="TypeScript" />
  <img src="https://img.shields.io/badge/macOS-Apple%20Silicon-000000?logo=apple&logoColor=white" alt="macOS" />
  <img src="https://img.shields.io/badge/Linux-x86__64%20%7C%20arm64-FCC624?logo=linux&logoColor=black" alt="Linux" />
</p>

---

## What is Orbit CLI

Orbit CLI is the terminal-based coding agent for the [Orbit](https://github.com/Recusive/Orbit) ecosystem. It provides a rich TUI (terminal user interface) for interacting with AI agents directly from your terminal — no IDE required.

Orbit CLI is the command-line counterpart to [Orbit Desktop](https://github.com/Recusive/Orbit), the AI-native development environment. While Orbit Desktop provides a full GUI with editor, browser, and terminal surfaces, Orbit CLI brings the same agent capabilities to developers who prefer working in the terminal.

---

## Features

- **Rich Terminal UI** — Built with Ratatui for a polished, responsive terminal experience
- **AI Agent Integration** — Conversational coding agent with tool execution
- **Sandboxed Execution** — Commands run in platform-specific sandboxes (Seatbelt on macOS, Landlock/seccomp on Linux)
- **MCP Support** — Model Context Protocol server for IDE integrations
- **App Server** — JSON-RPC WebSocket server for programmatic access
- **Skills System** — Extensible skill framework for custom agent behaviors
- **Session Management** — Persistent sessions with conversation history
- **File Operations** — Read, write, search, and patch files
- **Git Integration** — Built-in git operations and diff handling
- **Hooks System** — Lifecycle hooks for customizing agent behavior
- **Multi-Agent** — Spawn and manage multiple agent instances

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Orbit CLI                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  codex-rs/tui        Terminal UI (Ratatui)          │
│  codex-rs/core       Agent engine & tool execution  │
│  codex-rs/protocol   Message types & prompts        │
│  codex-rs/cli        Binary entry point & dispatch  │
│                                                     │
│  codex-rs/app-server      JSON-RPC WebSocket API    │
│  codex-rs/mcp-server      MCP protocol server       │
│  codex-rs/exec-server     Headless execution server │
│                                                     │
│  codex-rs/exec            Sandboxed execution       │
│  codex-rs/linux-sandbox   Linux sandbox (Landlock)  │
│  codex-rs/windows-sandbox Windows sandbox            │
│                                                     │
│  sdk/python          Python SDK                     │
│  sdk/typescript      TypeScript SDK                 │
│  shell-tool-mcp/     Shell tool MCP server          │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Development

### Prerequisites

- Rust 1.85+
- Node.js 22+
- pnpm 10+
- `just` command runner
- `cargo-nextest` (recommended for faster tests)

### Quick Start

```bash
# Clone
git clone https://github.com/Recusive/Orbit-CLI.git
cd Orbit-CLI

# Install Rust dependencies
cd codex-rs && cargo fetch

# Run from source
just codex

# Run tests
just test

# Format code
just fmt

# Lint
just fix
```

### Key Commands

| Command | Description |
|---------|-------------|
| `just codex` | Run Orbit CLI from source |
| `just test` | Run all Rust tests |
| `just fmt` | Format Rust code |
| `just fix` | Run clippy fixes |
| `just codex exec` | Run in headless/exec mode |
| `just mcp-server-run` | Run the MCP server |
| `just write-config-schema` | Regenerate config JSON schema |

---

## Repository Structure

```
Orbit-CLI/
├── codex-rs/              # Primary Rust codebase
│   ├── cli/               # Main binary entry point
│   ├── core/              # Agent engine, tools, config
│   ├── tui/               # Terminal UI (Ratatui)
│   ├── tui_app_server/    # TUI with app-server backend
│   ├── protocol/          # Message types and prompts
│   ├── app-server/        # JSON-RPC WebSocket API
│   ├── mcp-server/        # MCP protocol server
│   ├── exec/              # Headless execution
│   ├── exec-server/       # Execution server
│   ├── hooks/             # Lifecycle hook system
│   ├── skills/            # Skills framework
│   ├── state/             # SQLite session persistence
│   ├── config/            # TOML config system
│   ├── login/             # OAuth authentication
│   ├── utils/             # 19 utility crates
│   └── ...                # 50+ total crates
├── sdk/                   # Client SDKs
│   ├── python/            # Python SDK
│   └── typescript/        # TypeScript SDK
├── shell-tool-mcp/        # Shell tool MCP server
├── codex-cli/             # npm package wrapper
├── docs/                  # Documentation
├── scripts/               # Build & install scripts
└── tools/                 # Developer tooling
```

---

## Ecosystem

| Project | Description |
|---------|-------------|
| [**Orbit**](https://github.com/Recusive/Orbit) | AI-native desktop IDE (Tauri + React) |
| **Orbit CLI** (this repo) | Terminal-based coding agent |

Orbit CLI shares the agent engine with Orbit Desktop — the same AI capabilities, different interface.

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just fmt` and `just fix` in `codex-rs/`
5. Run `just test` to verify
6. Submit a pull request

---

## License

This project is based on [OpenAI Codex CLI](https://github.com/openai/codex), licensed under the [Apache License 2.0](LICENSE).

Modifications and additions by [Recursive Labs](https://orbit.build) are also licensed under Apache 2.0.

---

Built by [Recursive Labs](https://orbit.build)
