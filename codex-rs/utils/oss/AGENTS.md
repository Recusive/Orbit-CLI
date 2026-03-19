# codex-rs/utils/oss/

This file applies to `codex-rs/utils/oss/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-oss` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-oss`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-oss` -- OSS (open-source) model provider utilities.

### What this folder does

Shared utilities for working with local/OSS model providers (LM Studio and Ollama). Provides functions to get default models and ensure providers are ready, used by both the TUI and exec entry points.

### Key types and functions

- `get_default_model_for_oss_provider(provider_id) -> Option<&str>` -- returns the default model name for LM Studio or Ollama
- `ensure_oss_provider_ready(provider_id, config) -> Result<(), io::Error>` -- async function that ensures the specified provider is reachable and has required models downloaded

### Imports from

- `codex-core` -- `LMSTUDIO_OSS_PROVIDER_ID`, `OLLAMA_OSS_PROVIDER_ID`, `Config`
- `codex-lmstudio` -- `DEFAULT_OSS_MODEL`, `ensure_oss_ready`
- `codex-ollama` -- `DEFAULT_OSS_MODEL`, `ensure_responses_supported`, `ensure_oss_ready`

### Exports to

Consumed by `codex-tui` and `codex-exec` for OSS provider setup and model selection.

### Key files

- `Cargo.toml` -- crate metadata; depends on `codex-core`, `codex-lmstudio`, `codex-ollama`
- `src/lib.rs` -- `get_default_model_for_oss_provider`, `ensure_oss_provider_ready`, and tests
