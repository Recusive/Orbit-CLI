# codex-rs/tui_app_server/src/bin/

This file applies to `codex-rs/tui_app_server/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Auxiliary binary targets for the `codex-tui-app-server` crate.

### What this folder does

Contains additional binary entry points declared in `Cargo.toml` alongside the main `codex-tui-app-server` binary. Currently holds a single diagnostic utility.

### What it plugs into

- **Cargo.toml**: Declares `md-events-app-server` as a `[[bin]]` target with path `src/bin/md-events.rs`.

### Key files

| File | Role |
|------|------|
| `md-events.rs` | Diagnostic tool that reads Markdown from stdin, parses it with `pulldown-cmark`, and prints each parser event to stdout. Useful for debugging markdown rendering behavior in the TUI. |

### Imports from

- `pulldown_cmark` (workspace dependency) for markdown parsing.
- Standard library only (`std::io`).

### Exports to

- Standalone binary; not imported by other crates.
