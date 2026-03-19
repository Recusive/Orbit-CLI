# codex-rs/cli/src/

Source directory for the `codex-cli` crate (the `codex` binary).

## What this folder does

Contains the main binary entry point and library code for the Codex CLI multitool. The `main.rs` defines all subcommands via clap and dispatches to the appropriate crate. The `lib.rs` exports shared types used by `main.rs` plus the sandbox and login modules.

## Where it plugs in

- `main.rs` is the binary entry point producing the `codex` executable
- `lib.rs` provides the library target `codex_cli` used by `main.rs`
- Submodules handle sandbox debugging, desktop app management, login, MCP, and exit status

## Key files

| File | Role |
|------|------|
| `main.rs` | `MultitoolCli` clap struct with ~18 subcommands; `cli_main` dispatches to TUI, exec, MCP server, app server, apply, cloud, sandbox, login, features, etc. |
| `lib.rs` | Exports `SeatbeltCommand`, `LandlockCommand`, `WindowsCommand`; declares `debug_sandbox`, `exit_status`, and `login` modules |
| `debug_sandbox.rs` | `run_command_under_seatbelt`, `run_command_under_landlock`, `run_command_under_windows` -- spawns commands inside platform-specific sandboxes with config-driven policies |
| `login.rs` | Login/logout flow implementations |
| `mcp_cmd.rs` | `McpCli` subcommand for managing external MCP servers |
| `exit_status.rs` | `handle_exit_status` helper for mapping process exit codes |
| `wsl_paths.rs` | `normalize_for_wsl` -- adjusts paths for Windows Subsystem for Linux |
| `app_cmd.rs` | macOS-only `AppCommand` for launching/installing the Codex desktop app |
| `debug_sandbox/` | macOS-specific sandbox denial logging and PID tracking |
| `desktop_app/` | macOS desktop app open/install logic |
