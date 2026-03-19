# sdk/python/examples/14_turn_controls/

This file applies to `sdk/python/examples/14_turn_controls/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Examples should stay minimal, runnable, and aligned with the public SDK surface. Keep sync and async examples consistent when both exist.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Demonstrates turn steering and interruption.

### Purpose

Shows how to use `TurnHandle.steer()` to redirect an in-progress turn and `TurnHandle.interrupt()` to cancel one. Both are best-effort operations.

### Key Files

- `sync.py` -- Synchronous version demonstrating both steer and interrupt
- `async.py` -- Async version

### Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex`, `codex_app_server.TextInput`
