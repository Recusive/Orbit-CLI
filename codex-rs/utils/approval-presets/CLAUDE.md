# codex-rs/utils/approval-presets/

Crate `codex-utils-approval-presets` -- built-in approval and sandbox policy presets.

## What this folder does

Defines a UI-agnostic list of approval presets that pair an `AskForApproval` policy with a `SandboxPolicy`. These presets are reused by both the TUI and MCP server to offer users a simple selection of security postures.

## Key types and functions

- `ApprovalPreset` -- struct holding an id, label, description, approval policy, and sandbox policy
- `builtin_approval_presets()` -- returns the three built-in presets: "Read Only", "Default" (auto), and "Full Access"

## Imports from

- `codex-protocol` -- `AskForApproval` and `SandboxPolicy` types

## Exports to

Consumed by `codex-tui` and `codex-app-server` for presenting approval mode choices to users.

## Key files

- `Cargo.toml` -- crate metadata; depends on `codex-protocol`
- `src/lib.rs` -- `ApprovalPreset` struct and `builtin_approval_presets()` function
