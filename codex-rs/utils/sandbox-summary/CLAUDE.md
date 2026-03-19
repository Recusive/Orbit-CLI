# codex-rs/utils/sandbox-summary/

Crate `codex-utils-sandbox-summary` -- human-readable summaries of sandbox and configuration policies.

## What this folder does

Generates human-readable summary strings for sandbox policies and configuration settings, used in the TUI status bar and other display contexts.

## Key types and functions

- `summarize_sandbox_policy(policy: &SandboxPolicy) -> String` -- produces a compact summary like `"workspace-write [workdir, /tmp, $TMPDIR] (network access enabled)"` for any sandbox policy variant
- `create_config_summary_entries(config, model) -> Vec<(&str, String)>` -- builds key/value pairs summarizing the effective config (workdir, model, provider, approval, sandbox, reasoning effort/summaries)

## Imports from

- `codex-core` -- `Config`, `WireApi`
- `codex-protocol` -- `SandboxPolicy`, `NetworkAccess`

## Exports to

Used by `codex-tui` for displaying configuration status and by other UI surfaces.

## Key files

- `Cargo.toml` -- crate metadata; depends on `codex-core`, `codex-protocol`
- `src/lib.rs` -- module declarations and re-exports
- `src/sandbox_summary.rs` -- `summarize_sandbox_policy` with variant-specific formatting
- `src/config_summary.rs` -- `create_config_summary_entries` for building config display data
