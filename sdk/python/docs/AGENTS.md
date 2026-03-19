# sdk/python/docs/

This file applies to `sdk/python/docs/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.
- Documentation changes should track the current code and command surface. Update examples when behavior or CLI flags change.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`
- `cd /Users/no9labs/Developer/Recursive/codex && pnpm format`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Documentation for the Python SDK (`codex-app-server-sdk`).

### Purpose

Provides human-readable guides covering installation, API usage, and common pitfalls for SDK consumers.

### Key Files

| File | Role |
|------|------|
| `getting-started.md` | Quickstart tutorial: install, first sync turn, multi-turn threads, async parity, resuming threads, generated models |
| `api-reference.md` | Complete API reference with method signatures and behavioral notes for `Codex`, `Thread`, `TurnHandle`, `AppServerClient`, and all public types |
| `faq.md` | Common decisions, pitfalls, and troubleshooting guidance |

### Plugs Into

- Referenced from `sdk/python/README.md` as the "Docs map"
- Referenced from `getting-started.md` as cross-links to other docs
