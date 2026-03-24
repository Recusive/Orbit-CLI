# Learning 2: Anthropic Model Metadata Pipeline

> Session date: 2026-03-24
> Duration: ~6 hours of implementation, debugging, and testing

---

## What Was Built

A complete Anthropic model metadata pipeline that mirrors the GPT architecture. Claude models now flow through the same bundled catalog -> remote API -> cache -> ModelsManager -> TUI display path as GPT models, replacing hardcoded slug-matching workarounds.

### Files Created

| File | Purpose |
|------|---------|
| `anthropic/src/models.rs` | Typed client for `GET /v1/models` with pagination and auth |
| `anthropic/src/models_tests.rs` | 7 tests: deserialization, pagination, auth headers, error handling |
| `core/src/models_manager/anthropic_mapping.rs` | Maps API responses to ModelInfo: `merge_anthropic_capabilities`, `model_info_from_anthropic_api` |
| `core/src/models_manager/anthropic_mapping_tests.rs` | 9 tests: merge, fallback, zero values, unknown models |
| `docs/tracked/done/anthropic-model-metadata-pipeline.md` | Implementation plan (moved from todo to done) |

### Files Significantly Modified

| File | What Changed |
|------|-------------|
| `protocol/src/openai_models.rs` | Added `ThinkingStyle` enum (Budgeted/Adaptive), 5 capability fields on `ModelInfo` |
| `protocol/src/protocol.rs` | Added `model_context_window` to `SessionConfiguredEvent` |
| `core/models.json` | Claude entries populated with thinking_style, supports_effort, max_output_tokens |
| `core/src/models_manager/manager.rs` | Dual-provider ModelsManager with provider-owned snapshots, concurrent fetch |
| `core/src/models_manager/cache.rs` | Per-provider cache (models_cache_openai.json, models_cache_anthropic.json) |
| `core/src/models_manager/model_info.rs` | Reverted to direct assign for config overrides (original Codex behavior) |
| `core/src/anthropic_bridge.rs` | `anthropic_model_defaults` reads from ModelInfo, 3 slug-matching functions deleted |
| `core/src/thread_manager.rs` | Wires both OpenAI and Anthropic providers to ModelsManager |
| `core/src/codex.rs` | Populates `SessionConfiguredEvent.model_context_window` from catalog |
| `app-server-protocol/src/protocol/v2.rs` | Added `model_context_window` to `TurnStartedNotification` |
| `tui/src/status/card.rs` | Deleted `anthropic_model_context_window()` workaround |
| `tui/src/chatwidget.rs` | Initializes context window from `SessionConfiguredEvent`, model switch resolution |
| `tui/src/app.rs` | `UpdateModel` resolves context window via `get_catalog_context_window` |

---

## Key Architectural Discoveries

### 1. Provider-Owned Snapshots Pattern

The most important design discovery. The original `apply_remote_models()` rebuilt from the bundled file on every call. A naive merge-into-RwLock approach leaves stale entries when a provider stops advertising a model.

The correct design:
```
bundled_models: Vec<ModelInfo>          // immutable, from models.json
openai_remote: RwLock<Vec<ModelInfo>>   // provider-owned snapshot
anthropic_remote: RwLock<Vec<ModelInfo>> // provider-owned snapshot
remote_models: RwLock<Vec<ModelInfo>>   // DERIVED merged view
```

Each provider refresh replaces its own snapshot as a unit, then calls `rebuild_merged_catalog()`. This solves concurrent refreshes, stale entry cleanup, and bundled-reversion in one design.

### 2. with_config_overrides: Direct Assign, Not Capping

**Critical discovery**: The capping behavior (`min(config_value, catalog_max)`) in `with_config_overrides` was introduced by our Stage 3a commit, NOT the original Codex. The original behavior was **direct assign**:

```rust
// ORIGINAL (correct):
if let Some(context_window) = config.model_context_window {
    model.context_window = Some(context_window);
}

// WHAT WE INCORRECTLY ADDED (capping):
let capped = match model.context_window {
    Some(model_max) => context_window.min(model_max),
    None => context_window,
};
```

The direct assign is critical because `model_context_window = 1000000` in the config is how users unlock 1M context for GPT models. The bundled catalog has 272K for GPT-5.4, and the config override is what gives it 1M. Capping breaks this.

**Verification**: `git show df435eee3^:codex-rs/core/src/models_manager/model_info.rs` shows the original code.

### 3. Context Window Display: 3-Level Priority Chain

The status card and footer use a 3-level chain:

1. `token_info.model_context_window` -- from `TurnStartedEvent` (after turn, engine's resolved value)
2. `SessionConfiguredEvent.model_context_window` -- catalog value for Claude models (before turn)
3. `config.model_context_window` -- raw config fallback for GPT models (before turn)

For Claude Sonnet: #2 = 200K (from catalog) -> shows 200K before turn
For GPT-5.4: #2 = None -> falls to #3 = 1M (config override)
For Claude Opus: #2 = 1M (from catalog) -> shows 1M before turn

The `SessionConfiguredEvent.model_context_window` is populated ONLY for Claude models (`is_known_anthropic_model`). GPT models get `None` so the TUI falls through to config. This preserves original Codex behavior.

### 4. Anthropic /v1/models OAuth 401 Is Expected

The Anthropic `/v1/models` endpoint returns 401 for OAuth/Bearer tokens. Only API keys work. This is a known limitation. The error handling logs it at `warn!` level and falls back to the bundled catalog gracefully. The bundled catalog has correct values for all Claude models.

### 5. GPT-5.4 Bundled Catalog Has context_window: 272000

Despite being marketed as a 1M model, GPT-5.4 in the bundled `models.json` has `context_window: 272000`. The OpenAI remote API also returns 272K. The 1M capability comes from the user's config override. This is why the config override must use direct assign (not capping).

### 6. Model Switching Does Not Create a New Session

When a user selects a different model via `/model`, the session ID stays the same. The model change is applied in-place. No new `SessionConfiguredEvent` is emitted. The `AppEvent::UpdateModel` handler in `app.rs` must explicitly resolve the new model's context window.

### 7. tui_app_server Has #[cfg(test)] Methods That Mirror tui/ Non-Test Methods

`apply_turn_started_context_window` exists in `tui/src/chatwidget.rs` as a regular method but in `tui_app_server/src/chatwidget.rs` as `#[cfg(test)]` only. When we needed it in non-test code for `on_session_configured`, we had to remove the `#[cfg(test)]` gate. Be aware of this asymmetry when mirroring changes.

### 8. SessionConfiguredEvent Is Emitted Inside Session::new, Not the Outer Codex Spawn

The `SessionConfiguredEvent` is emitted at ~line 1868 in `Session::new()` (an inner function), NOT in the outer Codex spawning function that calls `Session::new()`. Variables defined in the outer function (like `model_info` at line 522) are NOT in scope. Data must be computed inside `Session::new` or passed as parameters.

---

## Anthropic API Reference

`GET https://api.anthropic.com/v1/models` returns:
```json
{
  "data": [{
    "id": "claude-opus-4-6",
    "display_name": "Claude Opus 4.6",
    "max_input_tokens": 1000000,
    "max_tokens": 128000,
    "capabilities": {
      "thinking": { "supported": true, "types": { "enabled": { "supported": true }, "adaptive": { "supported": true } } },
      "effort": { "supported": true, "low": {...}, "medium": {...}, "high": {...}, "max": { "supported": true } }
    }
  }],
  "has_more": false
}
```

Headers: `anthropic-version: 2023-06-01`, `x-api-key` or `Authorization: Bearer`
Pagination: `after_id` / `limit` (default 20, max 1000)
Auth caveat: OAuth returns 401. API key works.

---

## Claude Model Catalog Values

| Model | context_window | thinking_style | supports_effort | supports_effort_max | requires_extended_context_beta | max_output_tokens |
|-------|---------------|---------------|----------------|--------------------|-----------------------------|------------------|
| claude-sonnet-4-5-20250929 | 200K | budgeted | true | false | false | 64000 |
| claude-sonnet-4-6 | 200K | adaptive | true | false | false | 64000 |
| claude-opus-4-6 | 1M | adaptive | true | true | true | 128000 |
| claude-haiku-4-5-20251001 | 200K | budgeted | false | false | false | 64000 |

Key facts:
- Only Opus 4.6 gets the 1M context beta header (`context-1m-2025-08-07`)
- Only Opus 4.6 supports `max` effort level
- Sonnet 4.6 is adaptive thinking but 200K context (NOT 1M)
- max_output_tokens changed from hardcoded 32K to catalog values (128K Opus, 64K others)

---

## Mistakes Made and Corrected

### 1. Introduced Capping That Didn't Exist Upstream
The plan claimed `with_config_overrides` capping was pre-existing. It wasn't -- we introduced it in Stage 3a. Verified via `git show df435eee3^:codex-rs/core/src/models_manager/model_info.rs`. Reverted to direct assign.

### 2. cap_context_window_for_model Workaround
First attempt at fixing the status card display used a `cap_context_window_for_model` function that loaded the bundled catalog and did slug-matching (`starts_with("claude-")`). This was the same pattern we were trying to eliminate. Replaced with the proper `SessionConfiguredEvent` approach.

### 3. SessionConfiguredEvent Initially Used get_model_info (With Config Overrides)
First implementation populated `SessionConfiguredEvent.model_context_window` from `get_model_info()` which applies config overrides. This made GPT show 272K (capped) and Claude show 1M (config override). Fixed to use `get_catalog_context_window()` for Claude and `None` for GPT.

### 4. Assumed GPT-5.4 Had No context_window in Bundled Catalog
Checked the wrong lines in models.json (the entry spans ~100 lines with the huge base_instructions string). GPT-5.4 DOES have `context_window: 272000` in the bundled catalog at line 90.

---

## Testing Patterns Discovered

### TestTransport for HTTP Mocking
The `orbit-code-client` crate has `HttpTransport` trait with `execute` (non-streaming) and `stream` (SSE) methods. Tests create local `TestTransport` structs that implement this trait, queue responses, and record requests. No `MockTransport` exists in the library -- each test file defines its own.

### Snapshot Tests for Status Card
Any change to the status card triggers ~10 insta snapshot updates. Use `cargo insta accept` after reviewing. The snapshots are in `tui/src/status/snapshots/` and `tui/src/chatwidget/snapshots/`.

### SessionConfiguredEvent Test Constructions
~60+ test files construct `SessionConfiguredEvent` explicitly. Adding a field requires updating all of them. The subagent approach works well for this mechanical task.

---

## Performance Characteristics

- Dual-provider fetch uses `tokio::join!` -- OpenAI and Anthropic refresh run concurrently
- Anthropic 401 (OAuth) returns in ~50ms -- not a meaningful delay
- Bundled catalog parsing from `include_str!` is near-instant
- Per-provider cache files are sub-KB, reads are sub-millisecond
- `rebuild_merged_catalog` acquires a write lock on `remote_models`, serializing the two rebuilds

---

## Configuration Reference

```toml
# ~/.orbit/config.toml

# Global model context window override. Direct assign (no capping).
# This is what gives GPT models 1M context -- without it, GPT uses
# the bundled catalog value (272K).
# Claude models use their catalog values regardless of this setting
# (200K for Sonnet/Haiku, 1M for Opus).
model_context_window = 1000000
model_auto_compact_token_limit = 900000
```

The config override is the mechanism that unlocks 1M for GPT. Without it, GPT falls back to the bundled/remote catalog value. Claude models are unaffected -- their context windows come from the bundled catalog via the `SessionConfiguredEvent` pipeline.
