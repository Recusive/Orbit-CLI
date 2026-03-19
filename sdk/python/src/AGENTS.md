# sdk/python/src/

This file applies to `sdk/python/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source root for the `codex-app-server-sdk` Python package. Contains the single package `codex_app_server/`.

### Purpose

This directory is the `packages` root in `pyproject.toml` for the Hatch build system. Everything under `codex_app_server/` is what gets distributed in the wheel.

### Contents

- `codex_app_server/` -- the installable Python package (see its own CLAUDE.md for details)

### Plugs Into

- Referenced by `pyproject.toml` at `sdk/python/pyproject.toml` as `[tool.hatch.build.targets.wheel] packages = ["src/codex_app_server"]`
- Tests in `sdk/python/tests/` add this directory to `sys.path` via `conftest.py`
- Examples in `sdk/python/examples/` add this directory to `sys.path` via `_bootstrap.py`
