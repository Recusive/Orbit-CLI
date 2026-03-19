# codex-rs/utils/cli/

Crate `codex-utils-cli` -- shared CLI argument types and configuration override support.

## What this folder does

Provides reusable clap-derived argument types for the `--approval-mode`, `--sandbox` (`-s`), and `-c key=value` configuration override flags. These types are shared between the TUI, exec, and other CLI entry points to ensure consistent argument parsing.

## Key types and functions

- `ApprovalModeCliArg` -- clap `ValueEnum` mapping CLI flags to `AskForApproval` protocol variants
- `SandboxModeCliArg` -- clap `ValueEnum` mapping CLI flags to `SandboxMode` config variants
- `CliConfigOverrides` -- clap `Parser` struct capturing `-c key=value` overrides; includes `parse_overrides()` and `apply_on_value()` for merging onto TOML config trees
- `format_env_display` -- formats environment variable maps for display with masked values

## Imports from

- `clap` -- CLI argument parsing
- `codex-protocol` -- `AskForApproval`, `SandboxPolicy`, `SandboxMode` types
- `serde`, `toml` -- configuration value parsing

## Exports to

Consumed by `codex-cli` (TUI), `codex-exec` (headless), and `codex-app-server` for CLI argument handling.

## Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- module declarations and re-exports
- `src/approval_mode_cli_arg.rs` -- `ApprovalModeCliArg` enum
- `src/sandbox_mode_cli_arg.rs` -- `SandboxModeCliArg` enum
- `src/config_override.rs` -- `CliConfigOverrides` struct with TOML parsing and override application
- `src/format_env_display.rs` -- environment variable display formatting
