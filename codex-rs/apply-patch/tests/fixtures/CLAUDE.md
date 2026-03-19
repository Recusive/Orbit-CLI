# codex-rs/apply-patch/tests/fixtures/

Test fixture data for the apply-patch integration tests.

## What this folder does

Contains the `scenarios/` directory, which holds all end-to-end test cases for the apply-patch specification. Each scenario is a self-contained directory with input state, a patch file, and expected output state.

## What it plugs into

- Read by `tests/suite/scenarios.rs` which iterates over every scenario directory, copies input files to a temp directory, runs the `apply_patch` binary with the patch, and compares the resulting filesystem state against the expected directory.

## Key files

| File | Role |
|------|------|
| `scenarios/` | Directory of numbered test scenarios (001 through 022). |
| `scenarios/README.md` | Documents the scenario specification format. |
| `scenarios/.gitattributes` | Git attributes for fixture files. |
