# codex-rs/shell-escalation/src/

This file applies to `codex-rs/shell-escalation/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-shell-escalation` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-shell-escalation`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the shell-escalation crate.

### What this folder does

Contains the library entry point and the Unix-specific implementation module.

### Key files

- `lib.rs` -- conditional compilation gate: all types are only available on `cfg(unix)`. Re-exports the public API from the `unix` submodule.
- `unix/` -- the full Unix implementation of the escalation protocol.
- `bin/main_execve_wrapper.rs` -- binary entrypoint that delegates to `main_execve_wrapper()`.

### Exports to

- The parent crate (`codex-shell-escalation`) re-exports everything defined here.
