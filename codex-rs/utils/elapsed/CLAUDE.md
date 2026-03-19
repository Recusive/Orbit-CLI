# codex-rs/utils/elapsed/

Crate `codex-utils-elapsed` -- human-readable elapsed time formatting.

## What this folder does

Provides compact, human-readable formatting of durations: milliseconds for sub-second, two-decimal seconds for under a minute, and `Xm YYs` for longer durations.

## Key types and functions

- `format_elapsed(start_time: Instant) -> String` -- format elapsed time since an `Instant`
- `format_duration(duration: Duration) -> String` -- format a `Duration` directly
- Formatting rules: `<1s` = `"250ms"`, `1s-60s` = `"1.50s"`, `>=60s` = `"1m 15s"`

## Imports from

No external dependencies (std only).

## Exports to

Used by `codex-tui` and other crates for displaying timing information in the UI.

## Key files

- `Cargo.toml` -- crate metadata (no dependencies)
- `src/lib.rs` -- `format_elapsed`, `format_duration`, and `format_elapsed_millis` with tests
