# sdk/python/examples/03_turn_stream_events/

Demonstrates streaming turn events using `TurnHandle.stream()`.

## Purpose

Shows how to use `thread.turn(...)` to get a `TurnHandle`, then iterate over the notification stream to observe `turn/started`, `item/agentMessage/delta`, and `turn/completed` events in real time.

## Key Files

- `sync.py` -- Synchronous streaming with `for event in turn.stream()`
- `async.py` -- Async streaming with `async for event in turn.stream()`

## Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex`, `codex_app_server.TextInput`
