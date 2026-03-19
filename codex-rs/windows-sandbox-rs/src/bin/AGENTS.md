# codex-rs/windows-sandbox-rs/src/bin/

This file applies to `codex-rs/windows-sandbox-rs/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-windows-sandbox` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-windows-sandbox`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Binary entrypoints for Windows sandbox tools.

### What this folder does

Contains `main()` functions for the two Windows sandbox binaries. Both are Windows-only and panic on other platforms.

### Key files

- `setup_main.rs` -- `codex-windows-sandbox-setup` binary. Delegates to `setup_main_win.rs` which performs elevated one-time setup: creates sandbox users, configures ACLs, sets up firewall rules, and persists DPAPI-encrypted credentials.
- `command_runner.rs` -- `codex-command-runner` binary. Delegates to `elevated/command_runner_win.rs` which runs as the sandbox user: connects to IPC pipes, reads `SpawnRequest`, creates a restricted token, spawns the child process (via ConPTY or pipes), streams output back to the parent, and reports the exit code.

### What it plugs into

- `codex-windows-sandbox-setup` is run once with administrator privileges to bootstrap the sandbox environment.
- `codex-command-runner` is launched by the CLI under the sandbox user account for elevated-path sandbox execution.
