# app-server/tests/suite

## Purpose

Top-level integration test suite directory for the app-server. Organizes tests into feature-area modules and contains both v1-era tests and the `v2/` subdirectory for the v2 protocol.

## Structure

Declared in `mod.rs`:

- **`auth.rs`** -- Authentication flow tests.
- **`conversation_summary.rs`** -- Conversation summary request tests.
- **`fuzzy_file_search.rs`** -- Fuzzy file search integration tests.
- **`v2/`** -- Subdirectory containing the bulk of v2 protocol integration tests.
- **`bash/`, `zsh/`** -- Shell-specific test fixtures or scripts.

## What It Plugs Into

- Included by `tests/all.rs` via `mod suite;`.
- Each test module uses the shared `common/` (app_test_support) library for mock servers, auth fixtures, and process management.

## Test Pattern

Tests typically:
1. Start a mock model server with `create_mock_responses_server_*`.
2. Write a test config via `write_mock_responses_config_toml`.
3. Spawn the app-server as a child process via `McpProcess` or use in-process transport.
4. Send JSON-RPC requests and assert on responses/notifications.

## Imports From

- `app_test_support` (from `tests/common/`) -- All shared test infrastructure.
- `codex-app-server-protocol` -- JSON-RPC types for request construction and response parsing.

## Exports To

- No exports; test-only modules.
