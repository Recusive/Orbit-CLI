# codex-rs/utils/sandbox-summary/src/

This file applies to `codex-rs/utils/sandbox-summary/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-sandbox-summary` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-sandbox-summary`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-sandbox-summary` crate.

### Key files

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
