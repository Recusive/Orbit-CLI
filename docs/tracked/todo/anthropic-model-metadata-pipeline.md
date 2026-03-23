# Anthropic Model Metadata Pipeline — Proper Architecture

## Context

This repo is a fork of OpenAI's Codex CLI with Claude support added. GPT models have a complete metadata pipeline (bundled catalog → remote API refresh → cache → ModelsManager). Claude models were bolted on with hardcoded workarounds: `uses_adaptive_thinking()`, `requires_1m_context()`, `supports_effort_parameter()` in `anthropic_bridge.rs`, plus `anthropic_model_context_window()` in the TUI status card. This creates maintenance burden and incorrect behavior (context window display bug).

The fix: build a proper Anthropic model metadata pipeline mirroring the GPT architecture. Fetch capabilities from Anthropic's `/v1/models` API, merge with bundled behavior config, cache per-provider, and eliminate all hardcoded workarounds. This repo will become an SDK for a Tauri GUI app, so proper structure matters.

**Supersedes:** `fix-status-context-window-display.md` — the context window display bug is a symptom of the missing pipeline. This plan fixes the root cause.

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
      "effort": { "supported": true, "low": { "supported": true }, "medium": { "supported": true }, "high": { "supported": true }, "max": { "supported": true } },
      "image_input": { "supported": true },
      "pdf_input": { "supported": true },
      "structured_outputs": { "supported": true },
      "batch": { "supported": true },
      "citations": { "supported": true },
      "code_execution": { "supported": false },
      "context_management": { "supported": true, "compact_20260112": { "supported": true } }
    }
  }],
  "has_more": false
}
```
Headers: `anthropic-version: 2023-06-01`, `x-api-key` or `Authorization: Bearer`

**Note:** `max_input_tokens` reflects the API-key context limit. Via Claude OAuth (Pro/Max subscription), only Opus 4.6 gets 1M context — Sonnet 4.6 is rate-limited on long context for subscriptions. The `requires_extended_context_beta` flag applies to Opus only.

**Auth caveat:** OAuth/Bearer support for `/v1/models` must be validated during implementation — the documented contract only shows `x-api-key`. If OAuth fails, the bundled catalog is the fallback.

**Pagination:** API uses `after_id` / `before_id` / `limit` (default 20, max 1000), returns `first_id` / `last_id` / `has_more`.

---

## Behavioral Contracts to Preserve

### Custom Catalog (`model_catalog_json`)
When `config.model_catalog` / `model_catalog_json` is set, `CatalogMode::Custom` disables ALL remote refresh (`manager.rs:382`). Both the OpenAI and Anthropic fetchers must respect this — never mutate a user-supplied catalog via remote overlay unless an explicit opt-in is added.

### Config Override Semantics (`with_config_overrides`)
Current behavior: `config.model_context_window` is capped to `min(config_value, model_max)`. This lets users LOWER context windows for known models (e.g., to force earlier compaction). This plan does NOT change `with_config_overrides()` — the context window display bug is fixed in the status/token-info path instead.

### Provider-Specific Auth for Fetching
The repo supports provider-specific auth via `auth_manager.auth_cached_for_provider(ProviderName::Anthropic)` and `auth_cached_for_provider(ProviderName::OpenAI)`. The current refresh logic keys off `AuthMode::Chatgpt` for OpenAI only. The new Anthropic fetcher must use `ProviderName::Anthropic` auth discovery independently — not a single active-mode switch.

**Fetch eligibility rules:**
- OpenAI fetch: requires `auth_mode() == Some(AuthMode::Chatgpt)` — **preserves existing behavior** (`manager.rs:386`). API-key-only OpenAI sessions do NOT trigger remote refresh (they use bundled/cached catalog). This is intentional: the `/models` endpoint is a ChatGPT backend service, not the public OpenAI API.
- Anthropic fetch: requires `auth_cached_for_provider(ProviderName::Anthropic)` — works for both API-key and OAuth auth
- Mixed auth: both fetchers run independently using their respective eligibility rules
- No auth for either: bundled catalog only, no remote fetch
- `CatalogMode::Custom`: neither fetcher runs regardless of auth

---

## Implementation Phases

### Phase 1: Anthropic Models Client

**New file:** `codex-rs/anthropic/src/models.rs`

Create a typed client for `GET /v1/models` using existing infrastructure:
- Reuse `HttpTransport` trait from `orbit-code-client` (same as `AnthropicClient`)
- Reuse `AnthropicAuth` for API key / OAuth Bearer auth
- Set required `anthropic-version: 2023-06-01` header
- Handle pagination (`has_more` + `after_id` cursor, `limit` param)

Types:
```rust
pub struct AnthropicModelInfo {
    pub id: String,
    pub display_name: String,
    pub created_at: String,
    pub max_input_tokens: i64,
    pub max_tokens: i64,
    pub capabilities: AnthropicCapabilities,
}

pub struct SupportedFlag {
    pub supported: bool,
}

pub struct ThinkingCapability {
    pub supported: bool,
    pub types: Option<ThinkingTypes>,
}

pub struct ThinkingTypes {
    pub enabled: Option<SupportedFlag>,
    pub adaptive: Option<SupportedFlag>,
}

pub struct EffortCapability {
    pub supported: bool,
    pub low: Option<SupportedFlag>,
    pub medium: Option<SupportedFlag>,
    pub high: Option<SupportedFlag>,
    pub max: Option<SupportedFlag>,
}

pub struct AnthropicCapabilities {
    pub thinking: Option<ThinkingCapability>,
    pub effort: Option<EffortCapability>,
    pub image_input: Option<SupportedFlag>,
    pub pdf_input: Option<SupportedFlag>,
    pub structured_outputs: Option<SupportedFlag>,
    pub batch: Option<SupportedFlag>,
    pub citations: Option<SupportedFlag>,
    pub code_execution: Option<SupportedFlag>,
    pub context_management: Option<ContextManagementCapability>,
}

pub struct AnthropicModelsResponse {
    pub data: Vec<AnthropicModelInfo>,
    pub has_more: bool,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
}

pub struct AnthropicModelsClient { ... }
impl AnthropicModelsClient {
    pub async fn list_models(&self, auth: &AnthropicAuth) -> Result<AnthropicModelsResponse>;
}
```

**Modify:** `codex-rs/anthropic/src/lib.rs` — add `pub mod models;`, re-export client and types.

### Phase 2: Extend ModelInfo and ModelPreset with Capability Fields

**File:** `codex-rs/protocol/src/openai_models.rs`

#### 2a: Add `context_window` to `ModelPreset`

`ModelPreset` (line 119) currently lacks `context_window`. The `tui_app_server` only has `ModelPreset` (via `model_catalog.rs`), not `ModelInfo`. Without `context_window` on `ModelPreset`, the app-server TUI has no metadata source for the `/status` display fix.

Add to `ModelPreset`:
```rust
/// Context window size for display purposes.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub context_window: Option<i64>,
```

Update the `From<ModelInfo> for ModelPreset` impl to pass through `context_window`.

This gives `tui_app_server` a proper model metadata source: when the user selects a model, `ModelPreset.context_window` is the authoritative display value.

#### 2a (continued): End-to-end transport for `tui_app_server`

The full chain is: `ModelInfo` → `ModelPreset` (core) → v2 `Model` (wire via app-server `model/list`) → `ModelPreset` (in `tui_app_server`). To get `context_window` through:

**File:** `codex-rs/app-server-protocol/src/protocol/v2.rs` (~line 1816)
Add `context_window` to the public v2 `Model` struct. Per convention 20, `#[ts(optional = nullable)]` is for `*Params` only — response types use plain `Option<T>`:
```rust
pub struct Model {
    // ... existing fields ...
    #[ts(type = "number | null")]
    pub model_context_window: Option<i64>,
}
```

**File:** `codex-rs/app-server/src/models.rs` (~line 24)
Update `model_from_preset()` to pass through:
```rust
fn model_from_preset(preset: ModelPreset) -> Model {
    Model {
        // ... existing fields ...
        model_context_window: preset.context_window,
    }
}
```

**File:** `codex-rs/tui_app_server/src/app_server_session.rs`
Update the `Model` → `ModelPreset` conversion to include `context_window` from the v2 `Model.model_context_window` field.

This is an additive, non-breaking change to the public API — `model_context_window` is optional with `serde(default)`.

**Schema/docs updates required:**
- `just write-app-server-schema` (v2 `Model` shape changed)
- `codex-rs/app-server/README.md` — document new `modelContextWindow` field on `model/list` response
- `codex-rs/app-server/tests/suite/v2/model_list.rs` — update test assertions

#### 2b: Add capability fields to `ModelInfo`

Add fields to `ModelInfo` so capabilities are queryable from the catalog instead of hardcoded:

```rust
#[serde(default)]
pub supports_effort: bool,

/// Whether effort: "max" is supported (Opus 4.6 only).
#[serde(default)]
pub supports_effort_max: bool,

#[serde(default)]
pub thinking_style: ThinkingStyle,  // new enum: Budgeted (default) | Adaptive

/// Whether this model needs the 1M context beta header.
/// Sourced from bundled config, NOT inferred from max_input_tokens threshold.
#[serde(default)]
pub requires_extended_context_beta: bool,

/// Max output tokens for this model (e.g. 128000 for Opus 4.6, 64000 for Sonnet 4.6).
/// Replaces the hardcoded DEFAULT_ANTHROPIC_MAX_TOKENS in anthropic_bridge.rs.
#[serde(default, skip_serializing_if = "Option::is_none")]
pub max_output_tokens: Option<i64>,
```

New enum:
```rust
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default, TS, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingStyle {
    #[default]
    Budgeted,
    Adaptive,
}
```

All fields use `#[serde(default)]` for backward compatibility with existing serialized data.

### Phase 3: Mapping Layer — Anthropic API → ModelInfo

**New file:** `codex-rs/core/src/models_manager/anthropic_mapping.rs`

Two functions:

```rust
/// Merge API capabilities into an existing bundled ModelInfo.
/// Bundled entry provides behavior config (instructions, personality, truncation)
/// AND provider-specific runtime policy (requires_extended_context_beta).
/// API provides runtime capabilities (context_window, thinking_style, effort).
pub(crate) fn merge_anthropic_capabilities(
    bundled: &ModelInfo,
    api_model: &AnthropicModelInfo,
) -> ModelInfo;

/// Create ModelInfo for an API model with no bundled entry (newly released model).
/// Starts from model_info::model_info_from_slug(api_model.id) to get conservative
/// defaults for all behavior fields, then overlays API-derived capabilities.
pub(crate) fn model_info_from_anthropic_api(
    api_model: &AnthropicModelInfo,
) -> ModelInfo;
```

Merge logic:
- `context_window` ← `api_model.max_input_tokens` (only if > 0, else keep bundled)
- `max_output_tokens` ← `api_model.max_tokens` (only if > 0, else keep bundled)
- `thinking_style` ← Adaptive if `capabilities.thinking.types.adaptive.supported`, else Budgeted
- `supports_effort` ← `capabilities.effort.supported`
- `supports_effort_max` ← `capabilities.effort.max.supported` (Opus 4.6 only)
- `requires_extended_context_beta` ← **kept from bundled entry** (this is provider/runtime policy, not API capability)
- All behavior fields (instructions, personality, truncation, shell_type) come from bundled entry unchanged

For unknown models (`model_info_from_anthropic_api`): start from `model_info::model_info_from_slug(api_model.id)` to get conservative defaults for prompts, truncation, shell/tool behavior, input modalities, search-tool exposure, personality, and fallback markers. Then overlay the API-derived capability fields listed above.

### Phase 4: Per-Provider Caching

**File:** `codex-rs/core/src/models_manager/cache.rs`

- Cache file per provider: `models_cache_openai.json`, `models_cache_anthropic.json`
- `ModelsCacheManager` gains a `provider_id: String` field
- Backward compat: on first run, if `models_cache_openai.json` does not exist but legacy `models_cache.json` does, read the legacy file as the OpenAI cache; do not delete legacy file (safe migration)
- ETag-based TTL renewal remains OpenAI-only unless Anthropic `/v1/models` proves an equivalent ETag response contract

**Additional files to update:**
- `codex-rs/core/tests/suite/models_cache_ttl.rs` — update cache filename assumptions
- `codex-rs/app-server/tests/common/models_cache.rs` — update cache fixture helpers

### Phase 5: ModelsManager Becomes Provider-Aware

**File:** `codex-rs/core/src/models_manager/manager.rs`

Introduce a provider-scoped internal source that owns: provider id, provider config, cache manager, optional ETag, remote overlay list, and fetch-eligibility logic. Then `ModelsManager` merges source overlays onto the bundled catalog.

```rust
struct ProviderSource {
    provider_id: String,
    provider: ModelProviderInfo,
    cache_manager: ModelsCacheManager,
    etag: RwLock<Option<String>>,
    remote_models: RwLock<Vec<ModelInfo>>,
}
```

Major changes:
1. **Provider-scoped sources** instead of bare dual `Vec<ModelInfo>`:
   ```rust
   openai_source: ProviderSource,
   anthropic_source: Option<ProviderSource>,
   ```
2. **`get_remote_models()`** merges both sources' model lists
3. **New `fetch_anthropic_models()`** — checks `auth_manager.auth_cached_for_provider(ProviderName::Anthropic)`, uses `AnthropicModelsClient`, maps through `anthropic_mapping`, persists to Anthropic-specific cache
4. **`refresh_available_models()` becomes dispatcher** — respects `CatalogMode::Custom` (returns early, no fetch). Otherwise calls OpenAI and/or Anthropic fetch based on provider-specific auth eligibility
5. **Bundled `models.json` continues to serve both** — API fetch enriches, doesn't replace

### Phase 6: ThreadManager Passes Both Providers

**File:** `codex-rs/core/src/thread_manager.rs` (~line 173)

Currently hardcodes OpenAI provider. Change to pass both:
```rust
let openai_provider = config.model_providers
    .get(OPENAI_PROVIDER_ID).cloned()
    .unwrap_or_else(|| ModelProviderInfo::create_openai_provider(None));
let anthropic_provider = config.model_providers
    .get(ANTHROPIC_PROVIDER_ID).cloned()
    .unwrap_or_else(|| ModelProviderInfo::create_anthropic_provider());

models_manager: Arc::new(ModelsManager::new_with_providers(
    orbit_code_home, auth_manager.clone(),
    config.model_catalog.clone(),
    openai_provider, Some(anthropic_provider),
)),
```

### Phase 7: Eliminate Hardcoded Workarounds

**File:** `codex-rs/core/src/anthropic_bridge.rs`

Change `anthropic_model_defaults` to accept `&ModelInfo` instead of `&str`:
```rust
pub(crate) fn anthropic_model_defaults(
    model_info: &ModelInfo,
    effort: Option<ReasoningEffortConfig>,
) -> Result<AnthropicModelDefaults> {
    let max_tokens = model_info.max_output_tokens
        .unwrap_or(DEFAULT_ANTHROPIC_MAX_TOKENS as i64) as u64;

    let thinking = if model_info.thinking_style == ThinkingStyle::Adaptive {
        Some(ThinkingConfig::Adaptive {})
    } else {
        budgeted_thinking_config(max_tokens, normalized_effort)
    };

    let additional_beta_headers = if model_info.requires_extended_context_beta {
        vec![CONTEXT_1M_BETA_HEADER_VALUE]
    } else { Vec::new() };

    // Clamp XHigh → High when model supports effort but not "max".
    let effort = if model_info.supports_effort {
        let clamped = if !model_info.supports_effort_max
            && normalized_effort == ReasoningEffortConfig::XHigh
        {
            ReasoningEffortConfig::High
        } else {
            normalized_effort
        };
        Some(map_reasoning_effort_to_anthropic(clamped))
    } else { None };

    Ok(AnthropicModelDefaults {
        max_tokens,
        thinking,
        additional_beta_headers,
        effort,
    })
}
```

**Delete:** `uses_adaptive_thinking()`, `requires_1m_context()`, `supports_effort_parameter()`.

**File:** `codex-rs/core/src/client.rs` (~line 1459) — update call site to pass `&model_info` instead of `&slug`.

#### Status Display Fix (Critical Issue #1 from audit)

**Standalone TUI** (`codex-rs/tui/`):

**File:** `codex-rs/tui/src/status/card.rs`

Replace `anthropic_model_context_window()` with a proper model metadata path. The status card currently reads `token_info.model_context_window` or falls back to `config.model_context_window`. Instead:
- Thread the current model's `ModelInfo.context_window` (from `ModelsManager`) into the status card constructor
- Use `model_info.context_window` as the authoritative display value
- Fall back to `token_info.model_context_window` only when no model metadata is available (unknown model)
- Delete the `anthropic_model_context_window()` workaround entirely

Both `chatwidget.rs` status-line methods (`status_line_context_window_size`, `status_line_context_remaining_percent`) need the same fix — use model metadata, not `config.model_context_window`, as the display source. The `config.model_context_window` field continues to affect internal compaction via `with_config_overrides()`.

**App-server TUI** (`codex-rs/tui_app_server/`):

The app-server TUI does NOT have access to `ModelsManager` — it uses `ModelCatalog` which stores `Vec<ModelPreset>` (`tui_app_server/src/model_catalog.rs`). The fix relies on Phase 2a adding `context_window` to `ModelPreset`.

**File:** `codex-rs/tui_app_server/src/status/card.rs` — same status card fix, but source `context_window` from the current model's `ModelPreset` (stored in `ModelCatalog`) rather than `ModelsManager`.

**File:** `codex-rs/tui_app_server/src/chatwidget.rs` — status line methods source `context_window` from the selected `ModelPreset.context_window`, not from `config.model_context_window`.

**File:** `codex-rs/tui_app_server/src/model_catalog.rs` — add a method to look up `context_window` by model slug:
```rust
pub(crate) fn context_window_for_model(&self, slug: &str) -> Option<i64> {
    self.models.iter()
        .find(|p| p.model == slug)
        .and_then(|p| p.context_window)
}
```

**File:** `codex-rs/core/src/models_manager/model_info.rs` — `with_config_overrides()` is **unchanged**. The existing `min(config, model_max)` behavior is preserved. Users can still lower context windows for known models.

### Phase 8: Update Bundled models.json

**File:** `codex-rs/core/models.json`

Add capability fields to Claude entries so they work without a live API fetch:
```json
{
    "slug": "claude-opus-4-6",
    "thinking_style": "adaptive",
    "supports_effort": true,
    "supports_effort_max": true,
    "requires_extended_context_beta": true,
    "context_window": 1000000,
    "max_output_tokens": 128000,
    ...
},
{
    "slug": "claude-sonnet-4-6",
    "thinking_style": "adaptive",
    "supports_effort": true,
    "supports_effort_max": false,
    "requires_extended_context_beta": false,
    "context_window": 200000,
    "max_output_tokens": 64000,
    ...
},
{
    "slug": "claude-haiku-4-5-20251001",
    "thinking_style": "budgeted",
    "supports_effort": false,
    "supports_effort_max": false,
    "requires_extended_context_beta": false,
    "context_window": 200000,
    "max_output_tokens": 64000,
    ...
}
```

---

## Dependency Order

```
Phase 1 (anthropic client)  ─┐
Phase 2 (protocol types)    ─┤─→ Phase 3 (mapping) ─→ Phase 5 (manager) ─→ Phase 6 (thread_manager)
Phase 4 (cache)             ─┘                                            ─→ Phase 7 (workarounds + status fix)
Phase 8 (bundled JSON)      ─── (depends on Phase 2 only)
```

Phases 1, 2, 4, 8 can proceed in parallel. Phase 3 gates on 1+2. Phase 5 gates on 1+2+3+4. Phases 6+7 gate on 5.

## Verification

### Automated
```bash
cargo test -p orbit-code-anthropic          # Phase 1: models client deserialization
cargo test -p orbit-code-protocol           # Phase 2: ModelInfo backward compat
cargo test -p orbit-code-core -- models_manager  # Phases 3-5: mapping, manager, cache
cargo test -p orbit-code-core -- remote_models   # Provider-aware fetch
cargo test -p orbit-code-core -- model_catalog_json  # Custom catalog preserved
cargo test -p orbit-code-tui                # Phase 7: status card fix
cargo test -p orbit-code-tui-app-server     # Convention 54 mirror
cargo test -p orbit-code-app-server -- model_list  # App-server model list
cargo insta pending-snapshots -p orbit-code-tui
cargo insta pending-snapshots -p orbit-code-tui-app-server
just write-config-schema                    # Required (ModelInfo changes affect model_catalog_json)
just write-app-server-schema                # Required (v2 Model.model_context_window added)
just write-app-server-schema --experimental # If any experimental fields touched
just bazel-lock-update && just bazel-lock-check  # Only if dependencies change
just fmt
```

### Manual
1. Run `just codex`, select `claude-opus-4-6`, run `/status` — should show `1M` consistently (before AND after sending a message)
2. Switch models (opus → haiku → gpt-5.4 → opus), verify `/status` shows correct per-model context window immediately
3. Disconnect network, verify bundled catalog still provides correct capabilities
4. Check TUI model picker shows both GPT and Claude models with correct metadata
5. Set `model_context_window = 500000` in config, verify opus shows 500K (user override preserved, capped at 1M)

### New Tests to Add
- `anthropic/src/models.rs`: deserialize exact Anthropic API response, auth header injection, error handling (429, 529), pagination
- `core/src/models_manager/anthropic_mapping.rs`: merge API + bundled, unknown model fallback via `model_info_from_slug`, API `max_input_tokens=0` falls back to bundled, `requires_extended_context_beta` preserved from bundled (not derived from threshold)
- `core/src/models_manager/manager_tests.rs`: dual-provider fetch, per-provider cache isolation, auth-gated Anthropic fetch via `ProviderName::Anthropic`, `CatalogMode::Custom` disables both fetchers, legacy cache migration
- `core/src/anthropic_bridge.rs`: `anthropic_model_defaults(&ModelInfo)` with adaptive/budgeted thinking, effort flag, XHigh clamped to High when `supports_effort_max=false`, `max_output_tokens` used instead of hardcoded constant
- `tui/src/status/tests.rs`: `/status` before first turn shows correct context window from model metadata (not from config fallback), `/status` after model switch updates immediately
- `core/tests/suite/`: end-to-end with mocked Anthropic models endpoint, mixed-auth sessions

## Edge Cases

| # | Scenario | Expected Behavior | Where Handled |
|---|----------|-------------------|---------------|
| 1 | `/status` before any turn, Claude model, global `config.model_context_window` set | Standalone TUI: show `ModelInfo.context_window` from `ModelsManager` (e.g. 1M for opus). App-server TUI: show `ModelPreset.context_window` from `ModelCatalog`. Neither falls back to `config.model_context_window` for display — that field only affects internal compaction via `with_config_overrides()`. | Phase 7 status display fix. Requires test: `/status` renders catalog context window, not config value, when `token_info` is `None`. |
| 2 | `model_catalog_json` configured + both OpenAI and Anthropic creds present | Custom catalog is authoritative; `CatalogMode::Custom` early-returns in `refresh_available_models()`. Neither OpenAI nor Anthropic remote fetcher runs. The user-supplied catalog is the sole model source. | Phase 5, `refresh_available_models()` guard at `manager.rs:382`. Requires test: with `CatalogMode::Custom`, verify no HTTP calls even when both provider auths are available. |
| 3 | OpenAI API-key-only session (no ChatGPT auth) | OpenAI remote refresh does NOT run — existing `auth_mode() == Some(AuthMode::Chatgpt)` gate preserved. Bundled/cached catalog used. No behavioral change from today. | Phase 5 fetch eligibility rules. Requires test: API-key-only auth does not trigger OpenAI `/models` fetch. |
| 4 | Anthropic `/v1/models` returns `max_input_tokens=0` or `max_tokens=0` | `merge_anthropic_capabilities()` keeps bundled catalog values when API returns 0. Guard: `if api_model.max_input_tokens > 0 { model.context_window = Some(api_model.max_input_tokens); }`, same for `max_tokens`. | Phase 3 mapping layer. Requires test: merge with zero values preserves bundled defaults. |
| 5 | `ReasoningEffort::XHigh` on Sonnet 4.6 (`supports_effort=true`, `supports_effort_max=false`) | Bridge clamps `XHigh` → `High` before calling `map_reasoning_effort_to_anthropic()`. Wire sends `"high"`, not `"max"`. No error returned — silent downgrade. | Phase 7 bridge fix. Requires test: XHigh + `supports_effort_max=false` → serialized effort is `"high"`. |
| 6 | User sets `config.model_context_window=500000` for opus (intentional lower override) | `with_config_overrides()` produces `min(500K, 1M)` = 500K on `ModelInfo.context_window`. Status display shows 500K because it reads `ModelInfo.context_window` AFTER config overrides. User override respected, compaction triggers at 500K. | Phase 7 — `with_config_overrides()` unchanged. Status reads post-override `ModelInfo`. Requires test: lowered config override reflected in both status display and compaction threshold. |
| 7 | Legacy `models_cache.json` exists, no `models_cache_openai.json` | On first run: `ModelsCacheManager` for OpenAI checks for `models_cache_openai.json` first. If missing, falls back to reading legacy `models_cache.json`. Does NOT delete the legacy file (safe migration — avoids data loss if user downgrades). Writes new `models_cache_openai.json` on next cache persist. | Phase 4 cache migration. Requires test: legacy file read, new file written, legacy file still exists after migration. |
| 8 | `model/list` returns a model visible in app-server but has no bundled `context_window` and no remote overlay | `model_from_preset()` passes `ModelPreset.context_window` which is `None` for unknown models. v2 `Model.model_context_window` is `None` on the wire. `tui_app_server` status falls back to `token_info.model_context_window` (effective value from core), same as today for unknown models. | Phase 2a (`ModelPreset.context_window` defaults to `None`), Phase 7 status fallback chain. Requires test: unknown model with no catalog entry shows runtime token info, not blank. |
| 9 | Anthropic `/v1/models` returns a model not in bundled `models.json` (newly released) | `model_info_from_anthropic_api()` starts from `model_info_from_slug(api_model.id)` for conservative behavior defaults (272K context, budgeted thinking, no effort, generic prompts). Then overlays API capabilities (real context window, thinking style, effort support). Model appears in picker with real capabilities but generic behavior config. | Phase 3 unknown model path. Requires test: API-only model gets fallback behavior + real capabilities. |

## Files Changed Summary

| File | Action |
|------|--------|
| `anthropic/src/models.rs` | **New** — Anthropic models API client |
| `anthropic/src/lib.rs` | Modify — add `pub mod models`, re-exports |
| `protocol/src/openai_models.rs` | Modify — add `ThinkingStyle`, `supports_effort`, `supports_effort_max`, `requires_extended_context_beta`, `max_output_tokens` to `ModelInfo`; add `context_window` to `ModelPreset`; update `From<ModelInfo> for ModelPreset` |
| `core/src/models_manager/anthropic_mapping.rs` | **New** — API→ModelInfo mapping/merge |
| `core/src/models_manager/mod.rs` | Modify — add `pub mod anthropic_mapping` |
| `core/src/models_manager/cache.rs` | Modify — per-provider cache files, legacy migration |
| `core/src/models_manager/manager.rs` | Modify — `ProviderSource` struct, dual provider stores, Anthropic fetch |
| `core/src/thread_manager.rs` | Modify — pass both providers |
| `core/src/anthropic_bridge.rs` | Modify — accept `&ModelInfo`, XHigh clamping, delete hardcoded fns |
| `core/src/client.rs` | Modify — update call site |
| `core/models.json` | Modify — add capability fields to Claude entries |
| `tui/src/status/card.rs` | Modify — thread model metadata, delete `anthropic_model_context_window()` |
| `tui/src/chatwidget.rs` | Modify — status line uses model metadata for display |
| `tui_app_server/src/status/card.rs` | Modify — mirror status card changes, source from `ModelPreset.context_window` |
| `tui_app_server/src/chatwidget.rs` | Modify — mirror chatwidget changes, source from `ModelCatalog` |
| `tui_app_server/src/model_catalog.rs` | Modify — add `context_window_for_model()` lookup |
| `tui_app_server/src/app_server_session.rs` | Modify — pass `model_context_window` in `Model` → `ModelPreset` conversion |
| `app-server-protocol/src/protocol/v2.rs` | Modify — add `model_context_window: Option<i64>` to `Model` struct |
| `app-server/src/models.rs` | Modify — pass through `context_window` in `model_from_preset()` |
| `app-server/README.md` | Modify — document `modelContextWindow` field |
| `app-server/tests/suite/v2/model_list.rs` | Modify — update test assertions |
| `core/tests/suite/models_cache_ttl.rs` | Modify — update cache filename assumptions |
| `app-server/tests/common/models_cache.rs` | Modify — update cache fixture helpers |
| `tui/src/status/snapshots/` | Modify — accept updated snapshots |
| `tui_app_server/src/status/snapshots/` | Modify — accept updated snapshots |
