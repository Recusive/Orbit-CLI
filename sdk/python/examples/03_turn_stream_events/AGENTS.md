# sdk/python/examples/03_turn_stream_events/

This file applies to `sdk/python/examples/03_turn_stream_events/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Examples should stay minimal, runnable, and aligned with the public SDK surface. Keep sync and async examples consistent when both exist.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Demonstrates streaming turn events using `TurnHandle.stream()`.

### Purpose

Shows how to use `thread.turn(...)` to get a `TurnHandle`, then iterate over the notification stream to observe `turn/started`, `item/agentMessage/delta`, and `turn/completed` events in real time.

### Key Files

- `sync.py` -- Synchronous streaming with `for event in turn.stream()`
- `async.py` -- Async streaming with `async for event in turn.stream()`

### Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex`, `codex_app_server.TextInput`
