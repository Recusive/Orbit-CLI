# sdk/python/scripts/

This file applies to `sdk/python/scripts/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-app-server-sdk` Python package. Keep import surfaces and packaging metadata consistent when you move or rename modules.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/sdk/python && pytest`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Maintainer tooling for code generation and release staging.

### Purpose

Contains the `update_sdk_artifacts.py` script that handles:
1. **Type generation** (`generate-types`): Reads the JSON schema bundle from `codex-rs/app-server-protocol/schema/json/` and generates `src/codex_app_server/generated/v2_all.py` and `notification_registry.py` using `datamodel-code-generator`
2. **SDK staging** (`stage-sdk`): Prepares a release directory for `codex-app-server-sdk` with a pinned `codex-cli-bin` dependency version
3. **Runtime staging** (`stage-runtime`): Prepares a release directory for `codex-cli-bin` with a platform-specific Codex binary

### Key Files

- `update_sdk_artifacts.py` -- Multi-command CLI script for codegen and release artifact staging (~33KB)

### Imports From

- `codex-rs/app-server-protocol/schema/json/` for the source JSON schemas
- `sdk/python-runtime/` as the template for runtime package staging

### Exports To

- `src/codex_app_server/generated/` -- generated Pydantic models
- Release staging directories (temporary, used during CI)

### Usage

```bash
cd sdk/python
python scripts/update_sdk_artifacts.py generate-types
python scripts/update_sdk_artifacts.py stage-sdk /tmp/release-dir --runtime-version 1.2.3
python scripts/update_sdk_artifacts.py stage-runtime /tmp/runtime-dir /path/to/codex --runtime-version 1.2.3
```
