# codex-rs/utils/sandbox-summary/src/

Source directory for the `codex-utils-sandbox-summary` crate.

## Key files

- `lib.rs` -- module declarations; re-exports `create_config_summary_entries` and `summarize_sandbox_policy`
- `sandbox_summary.rs` -- `summarize_sandbox_policy(policy: &SandboxPolicy) -> String`:
  - `DangerFullAccess` -> `"danger-full-access"`
  - `ReadOnly` -> `"read-only"` with optional network access suffix
  - `ExternalSandbox` -> `"external-sandbox"` with optional network access suffix
  - `WorkspaceWrite` -> `"workspace-write [workdir, /tmp, $TMPDIR, <roots>]"` with optional network access suffix; includes/excludes `/tmp` and `$TMPDIR` based on config flags
  - Tests covering external sandbox, read-only, and workspace-write variants
- `config_summary.rs` -- `create_config_summary_entries(config: &Config, model: &str) -> Vec<(&str, String)>`:
  - Always includes: workdir, model, provider, approval policy, sandbox summary
  - For `WireApi::Responses`: adds reasoning effort and reasoning summaries
