# codex-rs/utils/oss/src/

This file applies to `codex-rs/utils/oss/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-oss` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-oss`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-oss` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `get_default_model_for_oss_provider(provider_id: &str) -> Option<&'static str>` -- matches against LM Studio and Ollama provider IDs to return their default models
  - `ensure_oss_provider_ready(provider_id: &str, config: &Config) -> Result<(), io::Error>` -- async function that:
    - For LM Studio: calls `codex_lmstudio::ensure_oss_ready`
    - For Ollama: first verifies responses API support, then calls `codex_ollama::ensure_oss_ready`
    - For unknown providers: no-op
  - Tests for each provider and unknown provider case
