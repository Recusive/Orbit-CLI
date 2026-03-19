# sdk/python/examples/12_turn_params_kitchen_sink/

This file applies to `sdk/python/examples/12_turn_params_kitchen_sink/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Examples should stay minimal, runnable, and aligned with the public SDK surface. Keep sync and async examples consistent when both exist.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Demonstrates advanced turn parameters including structured output.

### Purpose

Shows how to use the full set of `TurnStartParams` options: structured output schemas, reasoning effort, sandbox policies, and other advanced configuration.

### Key Files

- `sync.py` -- Synchronous version
- `async.py` -- Async version

### Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex`, various param types from `codex_app_server`
