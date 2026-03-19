# codex-rs/tui/src/bin/

Additional binary targets for the `codex-tui` crate.

## What this folder does

Contains auxiliary binary entry points beyond the main `codex-tui` binary (which lives at `../main.rs`). Currently holds a single debugging utility.

## Key files

| File | Role |
|------|------|
| `md-events.rs` | `md-events` binary -- a simple debugging tool that reads Markdown from stdin and prints the `pulldown-cmark` parse events to stdout. Useful for diagnosing markdown rendering issues in the TUI. |

## What it plugs into

- Uses `pulldown-cmark` directly for markdown parsing (same library used by the TUI's markdown renderer).
- Not part of the main application; strictly a developer debugging utility.
