# sdk/python/examples/01_quickstart_constructor/

Quickstart example demonstrating the simplest possible SDK usage.

## Purpose

Shows how to create a `Codex` instance, start a thread, run a single turn, and print the result. This is the recommended first example to run.

## Key Files

- `sync.py` -- Synchronous version using `Codex` context manager
- `async.py` -- Async version using `AsyncCodex`

## Imports From

- `_bootstrap` (parent `examples/` directory) for `ensure_local_sdk_src`, `runtime_config`, `server_label`
- `codex_app_server.Codex` / `codex_app_server.AsyncCodex`

## Running

```bash
cd sdk/python
python examples/01_quickstart_constructor/sync.py
python examples/01_quickstart_constructor/async.py
```
