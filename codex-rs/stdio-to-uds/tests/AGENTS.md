# codex-rs/stdio-to-uds/tests/

This file applies to `codex-rs/stdio-to-uds/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-stdio-to-uds` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-stdio-to-uds`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the stdio-to-UDS adapter.

### What this folder does

Tests the `codex-stdio-to-uds` binary end-to-end by creating a temporary Unix socket, spawning the binary, and verifying bidirectional data transfer.

### Key files

- `stdio_to_uds.rs` -- `pipes_stdin_and_stdout_through_socket`: creates a UDS listener in a temp directory, spawns the `codex-stdio-to-uds` binary with the socket path argument, sends "request" via stdin, verifies the server receives it, sends "response" from the server, and verifies it appears on stdout. Includes timeout handling and diagnostic event collection for flaky test debugging.

### Imports from

- `codex-utils-cargo-bin` -- locates the compiled binary.
- `tempfile` -- creates temporary directories for socket files.
- `anyhow`, `pretty_assertions`.
