# codex-rs/utils/approval-presets/src/

This file applies to `codex-rs/utils/approval-presets/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-approval-presets` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-approval-presets`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-approval-presets` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `ApprovalPreset` struct with fields: `id`, `label`, `description`, `approval` (`AskForApproval`), `sandbox` (`SandboxPolicy`)
  - `builtin_approval_presets()` -- returns a `Vec<ApprovalPreset>` with three presets:
    - `read-only` -- read files only, approval needed for edits and network
    - `auto` -- read/edit in workspace, approval for network and external edits
    - `full-access` -- no approval required, no sandbox
