# codex-rs/exec/tests/fixtures/

Test fixture data files for `codex-exec` integration tests.

## What this folder does

Stores static test data consumed by the integration test suite. These fixtures provide reproducible inputs for tests that verify CLI behavior and event processing.

## Key files and their roles

- `apply_patch_freeform_final.txt` -- Expected final output for an apply_patch freeform test case.
- `cli_responses_fixture.sse` -- Server-Sent Events (SSE) fixture used to simulate streaming CLI responses during integration tests.

## Used by

- Integration tests in `codex-rs/exec/tests/suite/` that verify end-to-end behavior of the `codex-exec` binary.
