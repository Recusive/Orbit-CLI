# codex-rs/tui/src/public_widgets/

Publicly exported reusable TUI widgets.

## What this folder does

Exposes widget components from the `codex-tui` crate that are designed for reuse by other crates in the workspace. Currently contains a single public widget: `ComposerInput`.

## What it plugs into

- **../lib.rs**: Re-exports `ComposerInput` and `ComposerAction` as public API of the `codex-tui` crate.
- **codex-cloud-tasks** and other external crates consume `ComposerInput` as a reusable text input field.
- Uses the internal `ChatComposer` from `bottom_pane/chat_composer.rs` under the hood.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module root; declares `composer_input` sub-module. |
| `composer_input.rs` | `ComposerInput` -- a minimal public wrapper around the internal `ChatComposer` that provides multi-line text input with submit semantics (Enter to submit, Shift+Enter for newline). Exposes `ComposerAction` enum (`Submitted(String)` or `None`). Supports `is_empty()`, `clear()`, `handle_key_event()`, `needs_redraw()`, and `render()`. |

## Exports

- `ComposerInput` -- reusable text input widget.
- `ComposerAction` -- action enum returned from key event handling.
