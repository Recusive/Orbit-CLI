# codex-rs/tui/tests/fixtures/

This file applies to `codex-rs/tui/tests/fixtures/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test fixture data files for integration tests.

### What this folder does

Contains static data files used by the integration test suite. Currently holds a single JSONL file used for testing OSS story rendering.

### Key files

| File | Role |
|------|------|
| `oss-story.jsonl` | A JSONL fixture file containing a recorded session/story used by integration tests to verify rendering of OSS (open-source) provider sessions. |

### What it plugs into

- **../suite/**: Test modules in the suite directory load fixture files from here during test execution.
