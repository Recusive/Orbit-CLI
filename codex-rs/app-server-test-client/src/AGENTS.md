# codex-rs/app-server-test-client/src/

This file applies to `codex-rs/app-server-test-client/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-app-server-test-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-app-server-test-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Source code for the `codex-app-server-test-client` binary. Implements a CLI tool for exercising the app-server JSON-RPC API.

### Key Files

| File | Role |
|------|------|
| `main.rs` | Binary entry point. Creates a single-threaded Tokio runtime and calls `codex_app_server_test_client::run()`. Minimal -- all logic is in `lib.rs`. |
| `lib.rs` | Core client implementation. Defines the CLI structure using `clap` with subcommands for all major app-server operations. Key components: |

#### `lib.rs` Details

- **CLI structure:** `Args` struct with `--url` (WebSocket endpoint), `--auto-approve`, `--log-level`, and a `Subcommand` enum covering: `ThreadStart`, `ThreadResume`, `ThreadRead`, `ThreadList`, `ThreadFork`, `ThreadRollback`, `ThreadArchive`, `ThreadUnarchive`, `TurnStart`, `TurnInterrupt`, `TurnSteer`, `ModelList`, `Login`, `GetAccount`, `GetAccountRateLimits`, `ConfigRead`, `ConfigValueWrite`, `Review`, `CommandExec`, `PluginInstall`, `PluginList`, `PluginRead`, `PluginUninstall`, `ThreadIncrementElicitation`, `ThreadDecrementElicitation`, and more.
- **Transport:** Connects via synchronous WebSocket (`tungstenite`) or spawns app-server as a child process with stdio transport.
- **Message loop:** `run_message_loop()` reads JSON-RPC messages, routes responses to pending requests, handles server requests (approval prompts) with optional auto-approval, and displays server notifications.
- **Auto-approval:** When `--auto-approve` is set, automatically accepts command execution and file change approval requests.
- **Interactive input:** For `TurnStart`, reads user prompts from stdin and streams agent responses to stdout.

### What It Plugs Into

- Connects to `codex-app-server` via WebSocket or stdio.
- Uses `codex-app-server-protocol` for all message types.

### Imports From

- `codex-app-server-protocol` -- All JSON-RPC types, `ClientRequest`, `ServerRequest`, `ServerNotification`, individual request/response param structs.
- `codex-core` -- Config loading for stdio mode.
- `codex-protocol` -- Shared protocol types.
- `tungstenite` -- Synchronous WebSocket client.
- `clap` -- CLI argument parsing.
- `serde_json` -- JSON serialization.

### Exports To

- `lib.rs` exports `pub async fn run()` consumed by `main.rs`.
