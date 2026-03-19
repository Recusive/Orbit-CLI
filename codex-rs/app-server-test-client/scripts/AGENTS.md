# codex-rs/app-server-test-client/scripts/

This file applies to `codex-rs/app-server-test-client/scripts/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-app-server-test-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-app-server-test-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Contains shell scripts used for manual or automated testing of app-server features via the test client.

### Key Files

| File | Role |
|------|------|
| `live_elicitation_hold.sh` | Tests MCP server elicitation hold behavior. Requires `APP_SERVER_URL` and `APP_SERVER_TEST_CLIENT_BIN` environment variables plus `CODEX_THREAD_ID`. The script: (1) increments the elicitation counter on the specified thread via the test client, (2) sleeps for a configurable duration (`ELICITATION_HOLD_SECONDS`, default 15s), (3) decrements the elicitation counter. Includes a cleanup trap to ensure the counter is decremented on script exit/interruption. |

### What It Plugs Into

- Invokes the `codex-app-server-test-client` binary with `thread-increment-elicitation` and `thread-decrement-elicitation` subcommands.
- Connects to a running app-server instance via the `APP_SERVER_URL` WebSocket endpoint.

### Required Environment Variables

- `APP_SERVER_URL` -- WebSocket URL of the running app-server.
- `APP_SERVER_TEST_CLIENT_BIN` -- Path to the test client binary.
- `CODEX_THREAD_ID` (or `THREAD_ID`) -- Thread ID to target for elicitation operations.
- `ELICITATION_HOLD_SECONDS` (optional) -- How long to hold the elicitation lock (default: 15 seconds).

### Exports To

- No exports; standalone test script.
