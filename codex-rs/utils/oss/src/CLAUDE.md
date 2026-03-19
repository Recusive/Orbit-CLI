# codex-rs/utils/oss/src/

Source directory for the `codex-utils-oss` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `get_default_model_for_oss_provider(provider_id: &str) -> Option<&'static str>` -- matches against LM Studio and Ollama provider IDs to return their default models
  - `ensure_oss_provider_ready(provider_id: &str, config: &Config) -> Result<(), io::Error>` -- async function that:
    - For LM Studio: calls `codex_lmstudio::ensure_oss_ready`
    - For Ollama: first verifies responses API support, then calls `codex_ollama::ensure_oss_ready`
    - For unknown providers: no-op
  - Tests for each provider and unknown provider case
