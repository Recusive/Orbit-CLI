# codex-rs/utils/cli/src/

Source directory for the `codex-utils-cli` crate.

## Key files

- `lib.rs` -- module declarations; re-exports `ApprovalModeCliArg`, `CliConfigOverrides`, `SandboxModeCliArg`
- `approval_mode_cli_arg.rs` -- `ApprovalModeCliArg` enum (Untrusted, OnFailure, OnRequest, Never) with `From` impl converting to `AskForApproval`
- `sandbox_mode_cli_arg.rs` -- `SandboxModeCliArg` enum (ReadOnly, WorkspaceWrite, DangerFullAccess) with `From` impl converting to `SandboxMode`
- `config_override.rs` -- `CliConfigOverrides` struct:
  - Captures `-c key=value` flags via `clap::ArgAction::Append`
  - `parse_overrides()` splits on first `=`, parses RHS as TOML (falling back to raw string)
  - `apply_on_value()` applies dotted-path overrides onto a `toml::Value` tree
  - Canonicalizes legacy key aliases (e.g., `use_legacy_landlock` -> `features.use_legacy_landlock`)
- `format_env_display.rs` -- `format_env_display()` function that formats env vars with masked values for display
