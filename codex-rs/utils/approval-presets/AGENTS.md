# codex-rs/utils/approval-presets/

This file applies to `codex-rs/utils/approval-presets/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-approval-presets` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-approval-presets`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-approval-presets` -- built-in approval and sandbox policy presets.

### What this folder does

Defines a UI-agnostic list of approval presets that pair an `AskForApproval` policy with a `SandboxPolicy`. These presets are reused by both the TUI and MCP server to offer users a simple selection of security postures.

### Key types and functions

- `ApprovalPreset` -- struct holding an id, label, description, approval policy, and sandbox policy
- `builtin_approval_presets()` -- returns the three built-in presets: "Read Only", "Default" (auto), and "Full Access"

### Imports from

- `codex-protocol` -- `AskForApproval` and `SandboxPolicy` types

### Exports to

Consumed by `codex-tui` and `codex-app-server` for presenting approval mode choices to users.

### Key files

- `Cargo.toml` -- crate metadata; depends on `codex-protocol`
- `src/lib.rs` -- `ApprovalPreset` struct and `builtin_approval_presets()` function
