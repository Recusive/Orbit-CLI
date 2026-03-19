# codex-rs/codex-experimental-api-macros/src/

This file applies to `codex-rs/codex-experimental-api-macros/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-experimental-api-macros` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-experimental-api-macros`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-experimental-api-macros` proc-macro crate.

### What this folder does

Contains the single-file implementation of the `ExperimentalApi` derive macro.

### Key files

| File | Role |
|------|------|
| `lib.rs` | `derive_experimental_api` proc macro: parses `#[experimental("reason")]` and `#[experimental(nested)]` attributes on struct fields and enum variants; generates `ExperimentalApi` trait impl with `experimental_reason()` method; generates `EXPERIMENTAL_FIELDS` constant; handles `Option<T>`, `Vec<T>`, `HashMap<K,V>`, and `bool` presence detection; snake_to_camel field name conversion for serialized names |
