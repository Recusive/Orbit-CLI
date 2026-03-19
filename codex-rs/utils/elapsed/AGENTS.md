# codex-rs/utils/elapsed/

This file applies to `codex-rs/utils/elapsed/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-elapsed` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-elapsed`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-elapsed` -- human-readable elapsed time formatting.

### What this folder does

Provides compact, human-readable formatting of durations: milliseconds for sub-second, two-decimal seconds for under a minute, and `Xm YYs` for longer durations.

### Key types and functions

- `format_elapsed(start_time: Instant) -> String` -- format elapsed time since an `Instant`
- `format_duration(duration: Duration) -> String` -- format a `Duration` directly
- Formatting rules: `<1s` = `"250ms"`, `1s-60s` = `"1.50s"`, `>=60s` = `"1m 15s"`

### Imports from

No external dependencies (std only).

### Exports to

Used by `codex-tui` and other crates for displaying timing information in the UI.

### Key files

- `Cargo.toml` -- crate metadata (no dependencies)
- `src/lib.rs` -- `format_elapsed`, `format_duration`, and `format_elapsed_millis` with tests
