# codex-rs/ollama/

This file applies to `codex-rs/ollama/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-ollama` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-ollama`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Client library for integrating Codex with a local Ollama server.

### What this folder does

The `codex-ollama` crate manages communication with a local Ollama instance for running open-source models. It handles server connectivity checks, model listing, model pulling with progress reporting, version detection, and Responses API compatibility checking.

### Where it plugs in

- **Consumed by**: `codex-cli` and `codex-tui` when the `--oss` flag is used and the configured provider is Ollama.
- **Depends on**: `codex-core` (for `Config`, `ModelProviderInfo`, `OLLAMA_OSS_PROVIDER_ID`), `reqwest` (HTTP client), `semver` (version parsing), `futures`/`async-stream` (streaming pull), `serde_json`, `tokio`, `tracing`.

### Main functions

- `ensure_oss_ready(config)` -- Top-level entry: verifies the Ollama server is reachable, checks model availability, pulls if missing with CLI progress display.
- `ensure_responses_supported(provider)` -- Version check: ensures Ollama >= 0.13.4 for Responses API support.
- `OllamaClient::try_from_oss_provider(config)` / `try_from_provider(provider)` -- Constructs a client and probes server health.
- `OllamaClient::fetch_models()` -- Lists local models via `/api/tags`.
- `OllamaClient::fetch_version()` -- Queries server version via `/api/version`.
- `OllamaClient::pull_model_stream(model)` -- Streams pull progress events.
- `OllamaClient::pull_with_reporter(model, reporter)` -- High-level pull with progress reporting.

### Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; depends on codex-core, reqwest, semver, async-stream, futures |
| `src/lib.rs` | Public API: `ensure_oss_ready()`, `ensure_responses_supported()`, `DEFAULT_OSS_MODEL` (`"gpt-oss:20b"`), version support logic |
| `src/client.rs` | `OllamaClient` implementation: server probing (native `/api/tags` or OpenAI-compat `/v1/models`), model listing, version fetching, streaming pull, progress-reported pull |
| `src/parser.rs` | `pull_events_from_value()`: parses JSON pull stream objects into `PullEvent` variants |
| `src/pull.rs` | `PullEvent` enum (Status, ChunkProgress, Success, Error), `PullProgressReporter` trait, `CliProgressReporter` (stderr inline progress with speed), `TuiProgressReporter` (delegates to CLI for now) |
| `src/url.rs` | URL utilities: `is_openai_compatible_base_url()` (detects `/v1` suffix), `base_url_to_host_root()` (strips `/v1` to get native Ollama root) |
