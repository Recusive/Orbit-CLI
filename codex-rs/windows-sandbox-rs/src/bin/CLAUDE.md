# codex-rs/windows-sandbox-rs/src/bin/

Binary entrypoints for Windows sandbox tools.

## What this folder does

Contains `main()` functions for the two Windows sandbox binaries. Both are Windows-only and panic on other platforms.

## Key files

- `setup_main.rs` -- `codex-windows-sandbox-setup` binary. Delegates to `setup_main_win.rs` which performs elevated one-time setup: creates sandbox users, configures ACLs, sets up firewall rules, and persists DPAPI-encrypted credentials.
- `command_runner.rs` -- `codex-command-runner` binary. Delegates to `elevated/command_runner_win.rs` which runs as the sandbox user: connects to IPC pipes, reads `SpawnRequest`, creates a restricted token, spawns the child process (via ConPTY or pipes), streams output back to the parent, and reports the exit code.

## What it plugs into

- `codex-windows-sandbox-setup` is run once with administrator privileges to bootstrap the sandbox environment.
- `codex-command-runner` is launched by the CLI under the sandbox user account for elevated-path sandbox execution.
