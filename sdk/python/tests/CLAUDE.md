# sdk/python/tests/

Pytest test suite for the Python SDK (`codex-app-server-sdk`).

## Purpose

Comprehensive tests covering the SDK's public API signatures, runtime behavior, JSON-RPC client methods, async client behavior, contract generation, artifact/binary workflows, and real app-server integration.

## Key Files

| File | Role |
|------|------|
| `conftest.py` | Test configuration: adds `sdk/python/src` to `sys.path` and clears cached `codex_app_server` module imports to ensure fresh loading |
| `test_public_api_signatures.py` | Validates that all public symbols exported from `codex_app_server` match expected names, types, and signatures |
| `test_public_api_runtime_behavior.py` | Tests runtime behavior of the public API surface (Codex, Thread, TurnHandle, etc.) with mocked app-server |
| `test_client_rpc_methods.py` | Tests `AppServerClient` JSON-RPC method calls and response parsing |
| `test_async_client_behavior.py` | Tests `AsyncAppServerClient` behavior with asyncio |
| `test_contract_generation.py` | Validates that generated Pydantic models in `generated/` are consistent and importable |
| `test_artifact_workflow_and_binaries.py` | Tests the release artifact staging and binary resolution workflows |
| `test_real_app_server_integration.py` | Integration tests that spawn a real `codex app-server` process (requires CLI binary) |

## Imports From

- `codex_app_server` (the SDK package, loaded from `src/` via conftest)
- `pytest` framework

## Running

```bash
cd sdk/python
pytest
```
