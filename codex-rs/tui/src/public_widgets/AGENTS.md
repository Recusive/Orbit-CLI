# codex-rs/tui/src/public_widgets/

This file applies to `codex-rs/tui/src/public_widgets/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Publicly exported reusable TUI widgets.

### What this folder does

Exposes widget components from the `codex-tui` crate that are designed for reuse by other crates in the workspace. Currently contains a single public widget: `ComposerInput`.

### What it plugs into

- **../lib.rs**: Re-exports `ComposerInput` and `ComposerAction` as public API of the `codex-tui` crate.
- **codex-cloud-tasks** and other external crates consume `ComposerInput` as a reusable text input field.
- Uses the internal `ChatComposer` from `bottom_pane/chat_composer.rs` under the hood.

### Key files

| File | Role |
|------|------|
| `mod.rs` | Module root; declares `composer_input` sub-module. |
| `composer_input.rs` | `ComposerInput` -- a minimal public wrapper around the internal `ChatComposer` that provides multi-line text input with submit semantics (Enter to submit, Shift+Enter for newline). Exposes `ComposerAction` enum (`Submitted(String)` or `None`). Supports `is_empty()`, `clear()`, `handle_key_event()`, `needs_redraw()`, and `render()`. |

### Exports

- `ComposerInput` -- reusable text input widget.
- `ComposerAction` -- action enum returned from key event handling.
