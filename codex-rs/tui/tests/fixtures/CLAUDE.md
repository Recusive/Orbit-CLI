# codex-rs/tui/tests/fixtures/

Test fixture data files for integration tests.

## What this folder does

Contains static data files used by the integration test suite. Currently holds a single JSONL file used for testing OSS story rendering.

## Key files

| File | Role |
|------|------|
| `oss-story.jsonl` | A JSONL fixture file containing a recorded session/story used by integration tests to verify rendering of OSS (open-source) provider sessions. |

## What it plugs into

- **../suite/**: Test modules in the suite directory load fixture files from here during test execution.
