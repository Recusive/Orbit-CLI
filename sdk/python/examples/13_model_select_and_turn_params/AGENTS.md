# sdk/python/examples/13_model_select_and_turn_params/

This file applies to `sdk/python/examples/13_model_select_and_turn_params/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Examples should stay minimal, runnable, and aligned with the public SDK surface. Keep sync and async examples consistent when both exist.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Demonstrates model discovery and turn parameter configuration.

### Purpose

Shows how to list available models, select the highest-capability model, configure reasoning effort, run turns, and print both the message and token usage.

### Key Files

- `sync.py` -- Synchronous version
- `async.py` -- Async version

### Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex` / `codex_app_server.AsyncCodex`
