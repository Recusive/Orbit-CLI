# sdk/python/examples/11_cli_mini_app/

This file applies to `sdk/python/examples/11_cli_mini_app/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Examples should stay minimal, runnable, and aligned with the public SDK surface. Keep sync and async examples consistent when both exist.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Interactive CLI chat loop built with the SDK.

### Purpose

Demonstrates building a simple interactive chat application using the SDK. Reads user input in a loop and streams agent responses.

### Key Files

- `sync.py` -- Synchronous interactive chat
- `async.py` -- Async interactive chat

### Imports From

- `_bootstrap` for setup helpers
- `codex_app_server.Codex` / `codex_app_server.AsyncCodex`
