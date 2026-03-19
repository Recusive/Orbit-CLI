# codex-rs/utils/elapsed/src/

This file applies to `codex-rs/utils/elapsed/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-elapsed` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-elapsed`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-elapsed` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `format_elapsed(start_time: Instant) -> String` -- convenience wrapper
  - `format_duration(duration: Duration) -> String` -- public API
  - `format_elapsed_millis(millis: i64) -> String` -- internal formatter with rules: `<1000ms` -> `"Xms"`, `<60000ms` -> `"X.XXs"`, `>=60000ms` -> `"Xm YYs"`
  - Tests covering sub-second, second-range, minute-range, and boundary cases
