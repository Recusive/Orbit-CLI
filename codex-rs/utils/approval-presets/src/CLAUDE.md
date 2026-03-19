# codex-rs/utils/approval-presets/src/

Source directory for the `codex-utils-approval-presets` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `ApprovalPreset` struct with fields: `id`, `label`, `description`, `approval` (`AskForApproval`), `sandbox` (`SandboxPolicy`)
  - `builtin_approval_presets()` -- returns a `Vec<ApprovalPreset>` with three presets:
    - `read-only` -- read files only, approval needed for edits and network
    - `auto` -- read/edit in workspace, approval for network and external edits
    - `full-access` -- no approval required, no sandbox
