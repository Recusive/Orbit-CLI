# Anthropic Model Metadata Pipeline — Proper Architecture

## Context

This repo is a fork of OpenAI's Codex CLI with Claude support added. GPT models have a complete metadata pipeline (bundled catalog → remote API refresh → cache → ModelsManager → with_config_overrides → TurnContext → TUI display). Claude models were bolted on with hardcoded workarounds: `uses_adaptive_thinking()`, `requires_1m_context()`, `supports_effort_parameter()` in `anthropic_bridge.rs`, plus `anthropic_model_context_window()` in the TUI status card. This creates maintenance burden and incorrect behavior.

The fix: build a proper Anthropic model metadata pipeline that mirrors the GPT architecture exactly. Same layers, same data flow, same config patterns. No workarounds, no special cases in the display layer.

**Reference:** Original OpenAI pipeline traced from `/reference/openai-codex/codex-rs/`.

**Last verified against codebase:** 2026-03-24 (see "Codebase Study" and "Review Board" and "Audit Resolutions" sections at end)

---

## System Design — Complete Data Flow

### GPT Pipeline (existing, reference architecture)

```
~/.orbit/config.toml
│  model = "gpt-5.4"
│  model_context_window = 1000000
│  model_provider = "openai"
│
▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 1: Bundled Catalog                                          │
│                                                                   │
│ core/models.json (compiled into binary via include_str!)          │
│ GPT-5.4: { context_window: 272000, ... }                         │
│                                                                   │
│ Loaded by: ModelsManager::load_remote_models_from_file()          │
│ Stored in: remote_models: RwLock<Vec<ModelInfo>>                  │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 2: Remote API Refresh                                       │
│                                                                   │
│ Trigger: auth_mode == AuthMode::Chatgpt                           │
│ Endpoint: OpenAI /models (via ModelsClient)                       │
│ Returns: GPT-5.4 with context_window: 1000000                    │
│                                                                   │
│ Merge: apply_remote_models() replaces bundled entries by slug     │
│ Result: remote_models RwLock now has GPT-5.4 at 1M               │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 3: Cache                                                    │
│                                                                   │
│ File: models_cache.json (TTL: 300s, versioned)                    │
│ ETag: conditional refresh via x-models-etag header                │
│ On startup: load cache if fresh, skip remote fetch                │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 4: Model Resolution                                         │
│                                                                   │
│ ModelsManager::get_model_info(slug, config)                       │
│   1. Longest-prefix match in remote_models                        │
│   2. Fallback: model_info_from_slug() → 272K default              │
│   3. Apply: with_config_overrides(model_info, config)             │
│      → CAPPED ASSIGN: config value capped to model's catalog max │
│      → min(config.model_context_window, model.context_window)    │
│      → If model has no catalog value: direct assign               │
│      → This prevents global 1M inflating 200K models              │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 5: Session / Turn                                           │
│                                                                   │
│ TurnContext { model_info: ModelInfo }                              │
│   .model_context_window()                                         │
│   = context_window × effective_context_window_percent / 100       │
│   = 1M × 95% = 950K                                              │
│                                                                   │
│ Emits: TurnStartedEvent { model_context_window: Some(950K) }     │
│ Source: core/src/tasks/regular.rs, core/src/compact.rs            │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 6: TUI Display                                              │
│                                                                   │
│ Status card priority (original, 2 levels):                        │
│   1. token_info.model_context_window  (from TurnStartedEvent)     │
│   2. config.model_context_window      (before first turn)         │
│                                                                   │
│ Status line (footer):                                             │
│   Same 2-level chain                                              │
│                                                                   │
│ Before turn: shows config value (1M)                              │
│ After turn: shows 950K (runtime effective value)                  │
└───────────────────────────────────────────────────────────────────┘
```

### Claude Pipeline (target — mirrors GPT exactly)

```
~/.orbit/config.toml
│  model = "claude-opus-4-6"
│  model_context_window = 1000000          ← global, capped to catalog max
│  # No per-provider config needed — capping handles it:
│  # Opus: min(1M, 1M catalog) = 1M
│  # Haiku: min(1M, 200K catalog) = 200K
│
▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 1: Bundled Catalog                                          │
│                                                                   │
│ core/models.json — Claude entries WITH capability fields:         │
│                                                                   │
│ claude-opus-4-6:                                                  │
│   context_window: 1000000                                         │
│   thinking_style: "adaptive"                                      │
│   supports_effort: true                                           │
│   supports_effort_max: true                                       │
│   requires_extended_context_beta: true                            │
│   max_output_tokens: 128000                                       │
│                                                                   │
│ claude-sonnet-4-6:                                                │
│   context_window: 200000                                          │
│   thinking_style: "adaptive"                                      │
│   supports_effort: true                                           │
│   supports_effort_max: false                                      │
│   requires_extended_context_beta: false                           │
│   max_output_tokens: 64000                                        │
│                                                                   │
│ claude-haiku-4-5-20251001:                                        │
│   context_window: 200000                                          │
│   thinking_style: "budgeted"                                      │
│   supports_effort: false                                          │
│   supports_effort_max: false                                      │
│   requires_extended_context_beta: false                           │
│   max_output_tokens: 64000                                        │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 2: Remote API Refresh                                       │
│                                                                   │
│ Trigger: auth_cached_for_provider(ProviderName::Anthropic)        │
│   - Independent of OpenAI auth — both fetchers run separately    │
│   - Works with API key auth                                       │
│   - OAuth returns 401 (known, non-fatal — falls back to bundled) │
│                                                                   │
│ Client: AnthropicModelsClient (new, in anthropic crate)           │
│   GET https://api.anthropic.com/v1/models?limit=1000              │
│   Headers: anthropic-version: 2023-06-01, x-api-key / Bearer     │
│   Pagination: after_id cursor                                     │
│                                                                   │
│ Response → AnthropicModelInfo:                                    │
│   { id, display_name, max_input_tokens, max_tokens, capabilities }│
│                                                                   │
│ Mapping layer (anthropic_mapping.rs):                             │
│   merge_anthropic_capabilities(bundled, api_model) → ModelInfo    │
│     - context_window ← api.max_input_tokens (if > 0)            │
│     - max_output_tokens ← api.max_tokens (if > 0)               │
│     - thinking_style ← capabilities.thinking.types.adaptive      │
│     - supports_effort ← capabilities.effort.supported            │
│     - supports_effort_max ← capabilities.effort.max.supported    │
│     - requires_extended_context_beta ← KEPT FROM BUNDLED          │
│       (runtime policy, not API-advertised)                        │
│     - All behavior fields (instructions, personality) from bundled│
│                                                                   │
│   model_info_from_anthropic_api(api_model) → ModelInfo            │
│     - For models not in bundled catalog (newly released)          │
│     - Starts from Claude-specific default template (NOT generic   │
│       model_info_from_slug — see Phase 4 for full spec)           │
│     - Overlays API capabilities                                   │
│                                                                   │
│ Provider snapshot: replaces anthropic_remote as a unit            │
│ Then: rebuild_merged_catalog() from bundled + all overlays        │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 3: Per-Provider Cache                                       │
│                                                                   │
│ OpenAI:    models_cache_openai.json                               │
│ Anthropic: models_cache_anthropic.json                            │
│ Legacy:    models_cache.json → migrated to _openai on first run  │
│                                                                   │
│ ModelsCacheManager gains:                                         │
│   for_openai(home, ttl)     — reads _openai.json, legacy fallback│
│   for_anthropic(home, ttl)  — reads _anthropic.json              │
│                                                                   │
│ ETag renewal: OpenAI only (Anthropic /v1/models has no ETag)     │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 4: Model Resolution (same path as GPT)                      │
│                                                                   │
│ ModelsManager::get_model_info("claude-opus-4-6", config)          │
│   1. Match in remote_models (bundled + remote overlay)            │
│   2. Returns: ModelInfo with all capability fields populated      │
│   3. Apply: with_config_overrides(model_info, config)             │
│      → CAPPED ASSIGN (same as GPT path, existing behavior)      │
│      → min(config.model_context_window, model.context_window)    │
│      → Config source: global model_context_window only           │
│                                                                   │
│ For Opus:  catalog=1M, config=1M → min(1M, 1M) = 1M             │
│ For Haiku: catalog=200K, no config override → 200K               │
│ For Haiku with global 1M: min(1M, 200K) = 200K (catalog wins)   │
│ For Haiku with no catalog ctx: 1M direct (fallback has no cap)   │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 5: anthropic_bridge.rs (catalog-driven)                     │
│                                                                   │
│ anthropic_model_defaults(&model_info, effort) → defaults          │
│                                                                   │
│ READS FROM model_info (no hardcoded strings):                     │
│   max_tokens     ← model_info.max_output_tokens.unwrap_or(32K)   │
│   thinking       ← model_info.thinking_style == Adaptive          │
│                    ? ThinkingConfig::Adaptive                      │
│                    : budgeted_thinking_config(max_tokens, effort)  │
│   beta_headers   ← model_info.requires_extended_context_beta      │
│                    ? [CONTEXT_1M_BETA_HEADER_VALUE]                │
│                    : []                                            │
│   effort         ← model_info.supports_effort                     │
│                    ? map_effort(clamp if !supports_effort_max)    │
│                    : None                                         │
│                                                                   │
│ DELETES: uses_adaptive_thinking(), requires_1m_context(),         │
│          supports_effort_parameter()                              │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 5b: Session / Turn (same as GPT — no special cases)         │
│                                                                   │
│ TurnContext.model_context_window()                                │
│   Opus:  1M × 95% = 950K                                         │
│   Haiku: 200K × 95% = 190K                                       │
│                                                                   │
│ TurnStartedEvent { model_context_window: Some(950K) }            │
└──────────────────────────────┬────────────────────────────────────┘
                               │
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ LAYER 6: TUI Display (same as GPT — no special cases)             │
│                                                                   │
│ NO anthropic_model_context_window() workaround                    │
│ NO catalog lookups in status card                                 │
│ Same 2-level priority: token_info > config                        │
│                                                                   │
│ Opus before turn:  config → min(1M, 1M catalog) = 1M             │
│ Opus after turn:   token_info → 950K                              │
│ Haiku before turn: config → min(1M global, 200K catalog) = 200K  │
│ Haiku after turn:  token_info → 190K                              │
│ Note: capping makes the catalog authoritative for the upper bound │
└───────────────────────────────────────────────────────────────────┘
```

### Dual-Provider ModelsManager Architecture

```
┌───────────────────────────────────────────────────────────────────┐
│ ModelsManager                                                     │
│                                                                   │
│ bundled_models: Vec<ModelInfo>         ← immutable, from JSON     │
│ openai_remote: RwLock<Vec<ModelInfo>>  ← provider-owned snapshot  │
│ anthropic_remote: RwLock<Vec<ModelInfo>> ← provider-owned snapshot│
│ remote_models: RwLock<Vec<ModelInfo>>  ← DERIVED merged view      │
│ catalog_mode: CatalogMode              ← Default | Custom         │
│                                                                   │
│ ┌─────────────────────────┐  ┌──────────────────────────────┐    │
│ │ OpenAI Source            │  │ Anthropic Source (optional)   │    │
│ │                          │  │                               │    │
│ │ provider: ModelProvider  │  │ provider: ModelProviderInfo   │    │
│ │ cache: _openai.json      │  │ cache: _anthropic.json        │    │
│ │ etag: RwLock<Option>     │  │ (no etag)                     │    │
│ │                          │  │                               │    │
│ │ Fetch eligibility:       │  │ Fetch eligibility:            │    │
│ │  auth_mode == Chatgpt    │  │  auth_cached_for_provider     │    │
│ │                          │  │  (ProviderName::Anthropic)    │    │
│ └─────────────────────────┘  └──────────────────────────────┘    │
│                                                                   │
│ refresh_available_models(strategy):                               │
│   if CatalogMode::Custom → return (neither fetcher runs)         │
│   tokio::join!(                                                   │
│     fetch_openai  → replaces openai_remote → rebuild_merged      │
│     fetch_anthropic → replaces anthropic_remote → rebuild_merged │
│   )   ← concurrent, each writes OWN snapshot then rebuilds       │
│                                                                   │
│ rebuild_merged_catalog():                                         │
│   merged = bundled + openai_remote overlay + anthropic_remote     │
│   *remote_models.write() = merged                                │
│                                                                   │
│ get_model_info(slug, config):                                     │
│   Same path for all models — longest-prefix match in merged       │
│   catalog, then with_config_overrides                             │
└───────────────────────────────────────────────────────────────────┘
```

### ThreadManager Wiring

```
ThreadManager::new(config, auth_manager, session_source)
│
├── openai_provider = config.model_providers["openai"]
│     .unwrap_or(create_openai_provider())
│
├── anthropic_provider = config.model_providers["anthropic"]
│     .unwrap_or(Some(create_anthropic_provider()))
│
└── ModelsManager::new_with_providers(
        orbit_code_home, auth_manager, model_catalog,
        openai_provider, Some(anthropic_provider)
    )
```

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
      "effort": { "supported": true, "low": { "supported": true }, "medium": { "supported": true }, "high": { "supported": true }, "max": { "supported": true } },
      "image_input": { "supported": true },
      "pdf_input": { "supported": true },
      "context_management": { "supported": true }
    }
  }],
  "has_more": false
}
```
Headers: `anthropic-version: 2023-06-01`, `x-api-key` or `Authorization: Bearer`
Pagination: `after_id` / `limit` (default 20, max 1000)

**Auth caveat:** OAuth/Bearer returns 401 for `/v1/models`. API key works. When OAuth fails, bundled catalog is the fallback.

---

## Behavioral Contracts to Preserve

### Custom Catalog (`model_catalog_json`)
When `config.model_catalog` is set, `CatalogMode::Custom` disables ALL remote refresh. Neither OpenAI nor Anthropic fetcher runs. The user-supplied catalog is authoritative.

### Config Override Semantics (`with_config_overrides`)
**Current behavior (verified 2026-03-23):** `with_config_overrides()` in `model_info.rs:24-66` CAPS the config override to the model's catalog context window:
```rust
let capped = match model.context_window {
    Some(model_max) => context_window.min(model_max),
    None => context_window,
};
```
This means a global `model_context_window = 1000000` will NOT inflate a model whose catalog says 200K. This is **correct and desirable** for the multi-provider world — it prevents a GPT-tuned global config from inflating Claude Haiku/Sonnet. The capping is a safety net that makes the catalog values authoritative for upper bounds.

**Rule:** Preserve this capping behavior. Do not revert to direct assign. The catalog `context_window` for each model is the source of truth for max capability.

### Provider-Specific Auth for Fetching
- OpenAI: requires `auth_mode() == Some(AuthMode::Chatgpt)` — preserves existing behavior
- Anthropic: requires `auth_cached_for_provider(ProviderName::Anthropic)` — works with API key and OAuth
- Mixed auth: both fetchers run independently
- No auth: bundled catalog only
- `CatalogMode::Custom`: neither fetcher runs

### Anthropic Models-Endpoint Credential Precedence
`auth_cached_for_provider(Anthropic)` at `auth/manager.rs:175` prefers cached/stored OAuth before falling back to `ANTHROPIC_API_KEY`. OAuth returns 401 for `/v1/models`, so a user with both stored OAuth and an API key will always hit the 401 path and fall back to bundled catalog — the working API key is never tried for model refresh.

**Decision: Accept this limitation for now.** The bundled catalog is correct at build time, and the 401 fallback is graceful. Adding models-endpoint-specific auth selection (prefer API key when present) is a follow-up improvement, not a requirement for this plan. The user experience is: models work correctly from the catalog; the remote refresh is a bonus for API-key-only users that keeps capability metadata fresh.

---

## Implementation Phases

### Phase 1: Protocol Types (foundation)

**File:** `protocol/src/openai_models.rs`

Add `ThinkingStyle` enum and capability fields to `ModelInfo`:
```rust
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Display, JsonSchema, TS, EnumIter, Hash)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ThinkingStyle {
    #[default]
    Budgeted,
    Adaptive,
}
```
**Convention note:** Derive set and `rename_all = "lowercase"` match sibling `ReasoningEffort` enum at `openai_models.rs:25-39`. `Display` and `EnumIter` come from `strum_macros` (already a dependency of the protocol crate).

Add to `ModelInfo` (all `#[serde(default)]` for backward compat):
- `supports_effort: bool`
- `supports_effort_max: bool`
- `thinking_style: ThinkingStyle`
- `requires_extended_context_beta: bool`
- `max_output_tokens: Option<i64>`

Update `model_info_from_slug()` fallback in `model_info.rs` with new fields defaulted.

Update all test files that construct `ModelInfo` directly (add fields with defaults).

### Phase 2: Bundled Catalog

**File:** `core/models.json`

Add capability fields to all 4 Claude entries currently in the catalog:

| Model | thinking_style | supports_effort | supports_effort_max | requires_extended_context_beta | max_output_tokens |
|-------|---------------|-----------------|--------------------|-----------------------------|------------------|
| claude-sonnet-4-5-20250929 | budgeted | true | false | false | 64000 |
| claude-sonnet-4-6 | adaptive | true | false | false | 64000 |
| claude-opus-4-6 | adaptive | true | true | true | 128000 |
| claude-haiku-4-5-20251001 | budgeted | false | false | false | 64000 |

**Note:** The bridge function `supports_effort_parameter()` also lists `claude-opus-4-5-20251101` but this model has no catalog entry. Once the bridge reads from `ModelInfo`, this model will get effort support only if it appears in the catalog or via the remote API refresh. No action needed — the data-driven approach handles this automatically.

### Phase 3: Anthropic Models Client

**New file:** `anthropic/src/models.rs`

Typed client for `GET /v1/models`:
- Reuse `HttpTransport` from `orbit-code-client`
- Reuse `AnthropicAuth` for API key / OAuth Bearer
- Handle pagination (`has_more` + `after_id`)
- Types: `AnthropicModelInfo`, `AnthropicModelsResponse`, `AnthropicCapabilities`

**Modify:** `anthropic/src/lib.rs` — add `mod models;` (private, Convention 1) with selective re-exports:
```rust
mod models;
pub use models::AnthropicModelsClient;
pub use models::AnthropicModelInfo;
pub use models::AnthropicModelsResponse;
pub use models::AnthropicCapabilities;
```
This matches the existing pattern in `lib.rs` where all 5 modules are `mod` (private) with `pub use` re-exports.

**Tests:** Sibling `models_tests.rs` with `#[cfg(test)] #[path = "models_tests.rs"] mod tests;` at the bottom of `models.rs`. Must use `pretty_assertions::assert_eq!` (Convention 38). Use `wiremock::MockServer` for HTTP mocking (Convention 40).

### Phase 4: Mapping Layer

**New file:** `core/src/models_manager/anthropic_mapping.rs`

Two functions:
- `merge_anthropic_capabilities(bundled: &ModelInfo, api: &AnthropicModelInfo) -> ModelInfo`
  - context_window ← api.max_input_tokens (if > 0, else keep bundled)
  - max_output_tokens ← api.max_tokens (if > 0, else keep bundled)
  - thinking_style ← adaptive if capabilities.thinking.types.adaptive.supported
  - supports_effort ← capabilities.effort.supported
  - supports_effort_max ← capabilities.effort.max.supported
  - requires_extended_context_beta ← **KEPT FROM BUNDLED** (policy, not API data)
  - All behavior fields (instructions, personality, truncation) from bundled unchanged

- `model_info_from_anthropic_api(api: &AnthropicModelInfo) -> ModelInfo`
  - For unknown models (newly released, not in bundled catalog)
  - **Does NOT start from `model_info_from_slug()`** — that generic fallback has `visibility: None`, `shell_type: Default`, non-Claude instructions, and would produce a hidden/misconfigured entry
  - Instead, starts from a **Claude-specific default template** with:
    - `visibility: List` (picker-visible)
    - `shell_type: ShellCommand`
    - `apply_patch_tool_type: Some(Freeform)`
    - `supported_in_api: true`
    - `base_instructions`: Claude instruction template (from nearest known preset or `prompt.md`)
    - `truncation_policy: TruncationPolicyConfig::bytes(10_000)` (matches existing Claude entries)
    - `thinking_style: Budgeted` (conservative default, overridden by API capabilities)
    - `context_window: Some(200_000)` (safe Claude default, overridden by `api.max_input_tokens`)
  - Then overlays API-derived capabilities (context_window, max_output_tokens, thinking_style, effort support)
  - This ensures a newly released Claude model from `/v1/models` is immediately usable in the picker with correct Claude behavior

**Tests:** Sibling `anthropic_mapping_tests.rs` with `pretty_assertions::assert_eq!` (Convention 38). Test merge, fallback, zero-value API fields, and unknown model handling.

### Phase 5: Per-Provider Cache

**File:** `core/src/models_manager/cache.rs`

- `ModelsCacheManager::for_openai(home, ttl)` — path: `models_cache_openai.json`, legacy fallback from `models_cache.json`
- `ModelsCacheManager::for_anthropic(home, ttl)` — path: `models_cache_anthropic.json`
- Legacy migration: if `_openai.json` missing but `models_cache.json` exists, read legacy as OpenAI cache (don't delete legacy)

### Phase 6: Dual-Provider ModelsManager

**File:** `core/src/models_manager/manager.rs`

**Provider-owned snapshots (REQUIRED — audit-driven redesign):**

The existing `apply_remote_models()` at `manager.rs:463-475` rebuilds from the bundled file on every call. A naive fix (merge into RwLock state) avoids last-writer-wins but leaves stale remote-only entries that cannot be removed when a provider stops advertising a model. The correct design keeps provider-owned state:

```rust
pub struct ModelsManager {
    // Immutable for process lifetime — loaded from include_str!("../../models.json")
    bundled_models: Vec<ModelInfo>,

    // Provider-owned remote snapshots — replaced as a unit on each refresh/cache load
    openai_remote: RwLock<Vec<ModelInfo>>,
    anthropic_remote: RwLock<Vec<ModelInfo>>,

    // Derived merged view — rebuilt after any provider snapshot changes
    remote_models: RwLock<Vec<ModelInfo>>,

    // Existing fields, renamed for clarity
    catalog_mode: CatalogMode,
    auth_manager: Arc<AuthManager>,
    openai_etag: RwLock<Option<String>>,
    openai_cache_manager: ModelsCacheManager,
    openai_provider: ModelProviderInfo,
    anthropic_cache_manager: Option<ModelsCacheManager>,
    anthropic_provider: Option<ModelProviderInfo>,
}
```

**Merge function — rebuild from three sources:**
```rust
async fn rebuild_merged_catalog(&self) {
    let openai = self.openai_remote.read().await;
    let anthropic = self.anthropic_remote.read().await;
    let mut merged = self.bundled_models.clone();

    // OpenAI overlays: replace matching slugs, add new ones
    for model in openai.iter() {
        if let Some(idx) = merged.iter().position(|m| m.slug == model.slug) {
            merged[idx] = model.clone();
        } else {
            merged.push(model.clone());
        }
    }
    // Anthropic overlays: same merge, on top of bundled+openai
    for model in anthropic.iter() {
        if let Some(idx) = merged.iter().position(|m| m.slug == model.slug) {
            merged[idx] = model.clone();
        } else {
            merged.push(model.clone());
        }
    }
    *self.remote_models.write().await = merged;
}
```

**Why this works:**
- Each provider refresh **replaces** its own snapshot as a unit, then calls `rebuild_merged_catalog()`
- If a provider stops advertising a model, its snapshot shrinks → the merged view reverts that slug to bundled metadata
- If a provider refresh fails, its snapshot stays as-is (from cache or previous fetch) → merged view is still correct
- `remote_models` is the read-optimized derived view used by `get_model_info()` and `list_models()`

New constructor: `new_with_providers(home, auth, catalog, openai_provider, anthropic_provider)`

**Concurrent fetching (REQUIRED):**
```rust
let (openai_result, anthropic_result) = tokio::join!(
    self.fetch_openai_models(),   // replaces openai_remote, calls rebuild
    self.fetch_anthropic_models(), // replaces anthropic_remote, calls rebuild
);
```
- CatalogMode::Custom → return (neither fetcher runs)
- OpenAI fetch: existing behavior, extracted to `fetch_openai_models()`
- Anthropic fetch: independent, concurrent, non-fatal on failure
- Each fetch writes to its own `RwLock<Vec<ModelInfo>>`, then calls `rebuild_merged_catalog()`
- The rebuild acquires a write lock on `remote_models`, serializing the two rebuilds

**Error handling boundary (REQUIRED):** `fetch_anthropic_models()` must:
- Catch ALL errors at the top level and log them — never propagate to callers
- Log network/timeout failures at `warn!` level (not `error!` — these are expected for OAuth users)
- Log OAuth 401 specifically at `info!` level with message "Anthropic OAuth does not support /v1/models, using bundled catalog"
- Do NOT reuse `map_anthropic_error()` from the streaming bridge — that mapper is for the Messages API, not the Models API
- Suppress retries for 401 responses (set `retry_429: false, retry_5xx: true` but skip 4xx entirely)

**LoC mitigation:** `manager.rs` is currently 575 lines. Adding dual-provider logic will push it past the 800-line threshold (Convention 5). Extract the OpenAI fetch path into a new `openai_fetch.rs` module alongside the Anthropic fetch, or extract the Anthropic fetch logic into the existing `anthropic_mapping.rs` module. Keep `manager.rs` as the coordinator that delegates to provider-specific fetch modules.

### Phase 7: ThreadManager Wiring

**File:** `core/src/thread_manager.rs`

Update `ThreadManager::new()` to resolve both providers and pass to `ModelsManager::new_with_providers()`.

### Phase 8: Eliminate Hardcoded Workarounds

**File:** `core/src/anthropic_bridge.rs`

Change `anthropic_model_defaults(slug: &str, ...)` to `anthropic_model_defaults(model_info: &ModelInfo, ...)`:
- Read thinking_style, supports_effort, supports_effort_max, requires_extended_context_beta, max_output_tokens from ModelInfo
- Delete: `uses_adaptive_thinking()`, `requires_1m_context()`, `supports_effort_parameter()`
- Add XHigh → High clamping when `supports_effort_max == false`

**File:** `core/src/client.rs` — update call site to pass `model_info` instead of `&slug`.

**USER-FACING BEHAVIORAL CHANGE — max_output_tokens:**
Currently `DEFAULT_ANTHROPIC_MAX_TOKENS = 32_000` is used for ALL Claude models. After this change, `max_tokens` is read from `model_info.max_output_tokens.unwrap_or(32_000)`:
- Opus 4.6: 32K → **128K** (4x increase)
- Sonnet 4.6: 32K → **64K** (2x increase)
- Sonnet 4.5: 32K → **64K** (2x increase)
- Haiku 4.5: 32K → **64K** (2x increase)

Effects: models can produce longer non-thinking text responses. Potential cost increase for API key users. Thinking budgets are unaffected — `budgeted_thinking_config` has `.min()` caps (16K for High, 31,999 for XHigh) that remain binding. Opus uses adaptive thinking (no budget), so the higher max_tokens has no effect on its thinking behavior. These values (128K/64K) match Anthropic's documented model maximums — the 32K was an artificial cap.

### Phase 9: Capping Observability (simplified — per-provider config DROPPED)

**DROPPED: Per-provider `model_context_window` on `ModelProviderInfo`.**
The original plan proposed `[model_providers.anthropic] model_context_window = 200000`, but this is **not implementable** in the current config loader:
- `config/mod.rs:152-157` defines `RESERVED_MODEL_PROVIDER_IDS` including `"anthropic"`
- `config/mod.rs:1967-1984` — `validate_reserved_model_provider_ids()` rejects any user config with reserved IDs
- `config/mod.rs:2408-2411` — user providers merge with `or_insert`, so built-in entries cannot be overridden

This would require a config loader redesign (relaxing reserved-ID validation, adding per-provider override surfaces). That is out of scope for this plan.

**Why this is OK:** The `with_config_overrides` capping behavior at `model_info.rs:30-40` already does the right thing. A global `model_context_window = 1000000` is capped to each model's catalog maximum:
- Opus: min(1M config, 1M catalog) = 1M
- Haiku: min(1M config, 200K catalog) = 200K
No per-provider config is needed for correct behavior.

**What Phase 9 now does:** Add `tracing::info!` to `with_config_overrides` when capping fires, for debuggability:

**File:** `core/src/models_manager/model_info.rs`
```rust
if capped < context_window {
    tracing::info!(
        model = %model.slug,
        config_value = context_window,
        catalog_max = model_max,
        capped_to = capped,
        "context window capped to model catalog maximum"
    );
}
```
No `just write-config-schema` needed (no config schema change).

### Phase 10: Status Display Cleanup

**Files:** `tui/src/status/card.rs`, `tui_app_server/src/status/card.rs`

Delete `anthropic_model_context_window()` workaround. The status card uses the original 2-level priority: `token_info > config`. No catalog lookups, no special cases.

**File:** `tui_app_server/src/app/app_server_adapter.rs`

Fix TurnStarted to extract `model_context_window` from notification instead of hardcoding `None`. This requires adding the field to v2 `TurnStartedNotification` in `app-server-protocol/src/protocol/v2.rs` and populating it in `app-server/src/bespoke_event_handling.rs`.

**Exact v2 field declaration (Convention 20 — MUST NOT use `skip_serializing_if` on v2 `Option<T>`):**
```rust
pub struct TurnStartedNotification {
    pub thread_id: String,
    pub turn: Turn,
    #[serde(default)]
    #[ts(type = "number | null")]
    pub model_context_window: Option<i64>,
}
```
This follows the sibling pattern from `ThreadTokenUsage.model_context_window` — `#[serde(default)]` for backward compat with old clients, `#[ts(type = "number | null")]` for correct TypeScript generation. Do NOT use `#[ts(optional = nullable)]` — that is only for `*Params` types per the V2 Quick Reference. The struct already has `#[ts(export_to = "v2/")]` and `#[serde(rename_all = "camelCase")]` (field serializes as `modelContextWindow`).

Run: `just write-app-server-schema`

**BUILD.bazel:** The `anthropic/BUILD.bazel` and `core/BUILD.bazel` use `orbit_code_rust_crate(...)` which auto-discovers `.rs` source files — no manual `srcs` edits needed for new Rust modules. No new `include_str!` calls are introduced, so `compile_data` does not change either. Run `just bazel-lock-check` to verify.

---

## Phase Dependencies

```
Phase 1 (protocol types) ─┐
Phase 2 (bundled JSON)    ─┤
Phase 3 (API client)      ─┤─→ Phase 4 (mapping) ─→ Phase 6 (manager)
Phase 5 (cache)            ─┘                       ─→ Phase 7 (thread_manager)
                                                    ─→ Phase 8 (bridge)
Phase 9 (capping log)      ── independent (trivial)
Phase 10 (status cleanup)  ── after Phase 8
```

Phases 1, 2, 3, 5, 9 can proceed in parallel. Phase 4 gates on 1+3. Phase 6 gates on 1-5. Phases 7+8+10 gate on 6.

---

## Config File Example

After implementation, users can configure:

```toml
# ~/.orbit/config.toml

# Global model context window — capped to each model's catalog maximum.
# Setting this to 1M is safe: Haiku/Sonnet are capped at 200K by catalog,
# Opus uses its full 1M, and GPT models use their catalog values.
model_context_window = 1000000

# The capping behavior means this single global value "does the right thing"
# for all models automatically. No per-provider config needed:
#   Opus:  min(1M config, 1M catalog) = 1M
#   Haiku: min(1M config, 200K catalog) = 200K
#   GPT:   min(1M config, catalog value) = catalog value
```

---

## Verification

### Automated
```bash
cargo test -p orbit-code-anthropic          # Phase 3: models client
cargo test -p orbit-code-protocol           # Phase 1: ModelInfo backward compat
cargo test -p orbit-code-core --lib         # Phases 4-8: mapping, manager, bridge
cargo test -p orbit-code-core -- remote_models   # Integration tests
cargo test -p orbit-code-app-server -- model_list # App-server integration
cargo test -p orbit-code-tui                # Phase 10: status card after workaround deletion
cargo test -p orbit-code-tui-app-server     # Phase 10: mirrored TUI
cargo insta pending-snapshots -p orbit-code-tui          # Check for snapshot changes
cargo insta pending-snapshots -p orbit-code-tui-app-server
just write-app-server-schema                # Phase 10
just bazel-lock-check                       # Verify Bazel lockfile
just fmt
```

**Note:** `just write-config-schema` is NOT needed — Phase 9 was simplified to a logging change only (no config schema change).

### Manual
1. `RUST_LOG=debug cargo run --bin orbit-code`, select `claude-opus-4-6`, `/status` → should show `1M` (min(config, 1M catalog) = 1M)
2. Switch to `claude-haiku-4-5-20251001`, `/status` → should show `200K` (min(1M global config, 200K catalog) = 200K, before turn)
3. Send message on Haiku, `/status` → should show `190K` (200K × 95%, from TurnStartedEvent)
4. Switch to `gpt-5.4`, `/status` → should show config value (from config.model_context_window)
5. Disconnect network → bundled catalog still works
6. **Capping test:** Set global `model_context_window = 2000000` (2M), select Haiku → should show 200K (capped to catalog), select Opus → should show 1M (capped to catalog)
7. **Provider removal test:** If Anthropic API stops listing a model, next refresh should revert that slug to bundled metadata (not retain stale remote overlay)

### Test Files to Add/Update
- `anthropic/src/models_tests.rs` — deserialization, auth, pagination (wiremock, pretty_assertions)
- `core/src/models_manager/anthropic_mapping_tests.rs` — merge, Claude-specific fallback, zero values, unknown model (pretty_assertions)
- All existing test files constructing `ModelInfo` — add new fields with defaults
- `app-server/tests/suite/v2/model_list.rs` — use `TestCodexBuilder` with injected auth/config to prevent real API calls (do NOT mutate process env per repo convention)
- `tui/src/status/tests.rs` — add Claude Opus and Haiku before-turn/after-turn context window tests that hit real bundled catalog (not just `construct_model_info_offline_for_tests`)
- `tui_app_server/src/status/tests.rs` — mirror TUI status tests (Convention 54)
- `core/src/models_manager/manager_tests.rs` — test dual-provider merge: both providers refresh, verify neither overlay is lost

---

## Files Changed Summary

| File | Action | Phase |
|------|--------|-------|
| `protocol/src/openai_models.rs` | Modify — ThinkingStyle enum, capability fields | 1 |
| `core/models.json` | Modify — capability fields on Claude entries | 2 |
| `anthropic/src/models.rs` | **New** — API client | 3 |
| `anthropic/src/models_tests.rs` | **New** — tests | 3 |
| `anthropic/src/lib.rs` | Modify — `mod models;` + selective `pub use` re-exports | 3 |
| `core/src/models_manager/anthropic_mapping.rs` | **New** — mapping layer | 4 |
| `core/src/models_manager/anthropic_mapping_tests.rs` | **New** — tests | 4 |
| `core/src/models_manager/mod.rs` | Modify — add module | 4 |
| `core/src/models_manager/cache.rs` | Modify — per-provider constructors | 5 |
| `core/src/models_manager/manager.rs` | Modify — dual-provider | 6 |
| `core/src/thread_manager.rs` | Modify — pass both providers | 7 |
| `core/src/anthropic_bridge.rs` | Modify — accept &ModelInfo, delete hardcoded fns | 8 |
| `core/src/client.rs` | Modify — update call site | 8 |
| `core/src/models_manager/model_info.rs` | Modify — add fields to fallback, add capping log | 1, 9 |
| `tui/src/status/card.rs` | Modify — delete workaround | 10 |
| `tui_app_server/src/status/card.rs` | Modify — delete workaround | 10 |
| `tui_app_server/src/app/app_server_adapter.rs` | Modify — extract model_context_window | 10 |
| `app-server-protocol/src/protocol/v2.rs` | Modify — TurnStartedNotification field | 10 |
| `app-server/src/bespoke_event_handling.rs` | Modify — populate field | 10 |
| 12+ test files | Modify — add new ModelInfo fields | 1 |

---

## Codebase Study — Verified Findings (2026-03-23)

A full codebase study was performed before implementation to validate every assumption in this plan. Results below.

### Confirmed Accurate

| Claim | Verified At | Status |
|-------|------------|--------|
| `ModelInfo` has no capability fields (thinking_style, supports_effort, etc.) | `protocol/src/openai_models.rs:243-294` | Correct — fields must be added |
| 3 hardcoded slug-matching functions exist in bridge | `core/src/anthropic_bridge.rs:593-610` | Correct — `uses_adaptive_thinking`, `requires_1m_context`, `supports_effort_parameter` |
| `anthropic_model_defaults` takes `slug: &str` not `&ModelInfo` | `core/src/anthropic_bridge.rs:61-99` | Correct — must change signature |
| ModelsManager is single-provider (one etag, one cache, one provider) | `core/src/models_manager/manager.rs:174-182` | Correct — dual-provider needed |
| `refresh_available_models()` only fetches for OpenAI (`auth_mode == Chatgpt`) | `core/src/models_manager/manager.rs:380-416` | Correct — Anthropic path missing |
| models.json has 4 Claude entries, opus has `context_window: 1000000` | `core/models.json` lines 847-1012 | Correct — no capability fields |
| Both TUI status cards have identical `anthropic_model_context_window()` workaround | `tui/src/status/card.rs:598-609`, `tui_app_server/src/status/card.rs:593-602` | Correct |
| `tui_app_server` TurnStarted hardcodes `model_context_window: None` | `tui_app_server/src/app/app_server_adapter.rs:658-661` | Correct |
| `TurnStartedNotification` (v2) has no `model_context_window` field | `app-server-protocol/src/protocol/v2.rs:4739-4742` | Correct |
| Anthropic crate has no `models.rs` module | `anthropic/src/lib.rs:1-39` | Correct — new file needed |
| `ModelProviderInfo` has no `model_context_window` field | `core/src/model_provider_info.rs:79-139` | Correct |
| `ModelsCacheManager` uses single `models_cache.json` | `core/src/models_manager/cache.rs:16-19`, `manager.rs:41` | Correct |
| `ThreadManager` wires `ModelsManager::new_with_provider` with OpenAI only | `core/src/thread_manager.rs:191-196` | Correct |
| `auth_cached_for_provider(ProviderName)` exists | `core/src/auth/manager.rs:175` | Correct |
| `create_anthropic_provider()` exists with correct base_url and headers | `core/src/model_provider_info.rs:290-323` | Correct |
| `model_info_from_slug()` fallback uses `context_window: Some(272_000)` | `core/src/models_manager/model_info.rs:94` | Correct |

### Discrepancies Found & Corrected

**1. `with_config_overrides` — Capping, Not Direct Assign (CRITICAL)**

Original plan claimed: "DIRECT ASSIGN, no capping (original OpenAI behavior)"

Actual code at `model_info.rs:30-40`:
```rust
let capped = match model.context_window {
    Some(model_max) => context_window.min(model_max),
    None => context_window,
};
model.context_window = Some(capped);
```
The code caps the config override to the model's catalog maximum. This is **correct behavior** — it prevents a global 1M config (set for GPT) from inflating Haiku's 200K. Plan updated to reflect and preserve this capping.

**Impact on design:** With capping + catalog `context_window` values on all Claude entries, the status card workaround (`anthropic_model_context_window()`) becomes naturally unnecessary. The catalog itself provides the upper bound, and `with_config_overrides` respects it. This makes Phase 10 cleanup even cleaner than originally described.

**2. `claude-opus-4-5-20251101` — Not in Catalog**

The bridge function `supports_effort_parameter()` lists this slug, but `models.json` has no entry for it. The data-driven approach handles this automatically — if the model isn't in the catalog and is discovered via the Anthropic `/v1/models` API, it gets Claude-specific fallback metadata (Phase 4's `model_info_from_anthropic_api`) with API capabilities overlaid. If it's not in the API either, `model_info_from_slug()` provides a generic conservative fallback.

**3. `client.rs` Call Site — Already Passes `model_info`**

At `client.rs:1455`, the call is `anthropic_model_defaults(&model_info.slug, effort)`. The `model_info` variable is already in scope — Phase 8 only needs to change the argument from `&model_info.slug` to `&model_info` (and update the function signature). No structural changes to the call site needed.

---

## Review Board Resolutions (2026-03-23)

A 5-agent review board examined the plan from Backend/Rust, Performance, Production Readiness, DX, and Conventions perspectives. All findings below were resolved in the plan.

### Blocker Resolved

| # | Issue | Flagged By | Resolution |
|---|-------|-----------|------------|
| 1 | Phase 10 v2 field attributes unspecified — risk of Convention 20 violation | Conventions | Added exact field declaration: `#[serde(default)] #[ts(type = "number \| null")]`, no `skip_serializing_if` |

### High-Priority Items Resolved

| # | Issue | Flagged By | Resolution |
|---|-------|-----------|------------|
| 2 | Sequential fetching doubles 5s timeout to 10s | Backend, Performance | Phase 6 now mandates `tokio::join!` for concurrent provider fetches |
| 3 | Config precedence inverted (global > provider) | Backend, Prod Readiness | Phase 9 dropped entirely — per-provider config blocked by reserved-ID validation; capping handles it |
| 4 | OAuth 401 logged at `error!`, retries waste ~20s | Prod Readiness | Phase 6 specifies `info!` for 401, `warn!` for other failures, retry suppression for 4xx |
| 5 | Error boundary: `fetch_anthropic_models()` must catch-and-log | Backend, Prod Readiness | Phase 6 specifies catch-all at top level, no propagation to callers |
| 6 | `pub mod models` violates Convention 1 | Conventions | Phase 3 fixed: `mod models;` + selective `pub use` re-exports |

### Medium Items Resolved

| # | Issue | Flagged By | Resolution |
|---|-------|-----------|------------|
| 7 | `manager.rs` at 575 LoC, will exceed 800 after Phase 6 | Conventions | Phase 6 added LoC mitigation strategy (extract fetch methods) |
| 8 | `ThinkingStyle` missing `Display`, `EnumIter`, `strum` | Conventions | Phase 1 updated derive set to match sibling `ReasoningEffort` |
| 9 | max_output_tokens 32K→128K/64K is user-visible | DX | Phase 8 documents behavioral change with effects analysis |
| 10 | New test files must use `pretty_assertions` + `wiremock` | Conventions | Phases 3+4 test sections updated with convention requirements |
| 11 | New source files need `BUILD.bazel` entries | Conventions | Phase 10 added BUILD.bazel note for new files |

### Confirmed Non-Issues

- Crate boundaries: correct (anthropic for client, core for mapping)
- Backward compat: `#[serde(default)]` handles old caches/payloads
- Status display transition: catalog values match current workaround — zero visual difference
- Model switching mid-session: `get_model_info()` re-resolves correctly
- Graceful degradation: bundled catalog always available
- Legacy cache migration: read-time fallback, no destructive write
- Memory: ~32 bytes/model, sub-2KB total
- Pagination: `limit=1000` covers all Anthropic models in one request
- Hot path: field reads replace string matching (faster)
- Cache I/O: two sub-millisecond async reads (negligible)

---

---

## Audit Resolutions (2026-03-24)

A design/code audit (`reviews/anthropic-model-metadata-pipeline.audit.md`) found 3 critical flaws in the plan. All have been resolved.

### Critical Issue 1: Provider-Owned Snapshots Required — FIXED (2 audit rounds)

**Problem (audit 1):** `apply_remote_models()` at `manager.rs:463-475` always rebuilds from the **bundled file**, not the RwLock state. Second provider write discards first's overlay.

**Interim fix (insufficient):** Merge into RwLock state instead of bundled file. Solved last-writer-wins but left stale remote-only entries that cannot be removed when a provider stops advertising a model.

**Problem (audit 2):** One merged snapshot means a later refresh returning fewer models cannot clean up stale provider-owned entries.

**Final fix:** Provider-owned snapshots. `ModelsManager` now holds `bundled_models` (immutable), `openai_remote` (RwLock), `anthropic_remote` (RwLock), and derives `remote_models` via `rebuild_merged_catalog()` after any provider snapshot changes. Each provider refresh replaces its own snapshot as a unit, then rebuilds. This solves concurrent refreshes, stale entry cleanup, and bundled-reversion in one design.

### Critical Issue 2: Reserved Provider IDs Block Per-Provider Config — FIXED (Phase 9 dropped)

**Problem:** `config/mod.rs:152-157` defines `RESERVED_MODEL_PROVIDER_IDS` including `"anthropic"`. `validate_reserved_model_provider_ids()` at line 1967-1984 rejects any user config with these IDs. The plan's `[model_providers.anthropic] model_context_window = 200000` would be rejected at config load time.

**Fix:** Dropped Phase 9 entirely. Per-provider config is not needed — the `with_config_overrides` capping behavior already handles cross-provider config correctly. A global `model_context_window = 1000000` is capped to each model's catalog maximum (Opus=1M, Haiku=200K). Per-provider overrides can be added in a future plan after redesigning the config loader's reserved-ID handling.

### Critical Issue 3: Unknown Claude Model Fallback Is Unusable — FIXED

**Problem:** `model_info_from_slug()` at `model_info.rs:69-101` produces a generic fallback with `visibility: None`, `shell_type: Default`, `apply_patch_tool_type: None`, and non-Claude instructions. A newly released Claude model discovered via `/v1/models` would be hidden from the picker and misconfigured.

**Fix:** Phase 4's `model_info_from_anthropic_api()` now starts from a Claude-specific default template (picker-visible, ShellCommand, Freeform apply_patch, Claude instructions, 200K default context) instead of the generic fallback. API capabilities are overlaid on top. New models are immediately usable.

### Recommended Improvements Also Addressed

| # | Audit Recommendation | Resolution |
|---|---------------------|------------|
| 1 | Treat max_output_tokens rollout as explicit behavior change | Already documented in Phase 8 (review board fix #9) |
| 2 | Update TUI status tests to hit real bundled catalog path | Added to Phase 10 verification plan |
| 3 | BUILD.bazel guidance inaccurate (srcs not enumerated) | Corrected — `orbit_code_rust_crate()` auto-discovers `.rs` files |

### Edge Cases Now Addressed

| Edge Case | Resolution |
|-----------|-----------|
| Both providers refresh in same startup pass | Provider-owned snapshots — each writes its own RwLock, then `rebuild_merged_catalog()` |
| Provider refresh returns fewer models | Snapshot replacement removes stale entries; rebuild reverts to bundled for dropped slugs |
| Stale remote-only entries in cache | Cache loads replace provider snapshot as a unit; rebuild only includes current snapshot |
| Both OAuth and API key exist for Anthropic | `auth_cached_for_provider` prefers OAuth → 401 → falls back to bundled (documented; API-key-only models refresh is a follow-up) |
| New Claude model returned by API but not in bundled catalog | `model_info_from_anthropic_api()` uses Claude-specific defaults, not generic fallback |
| Both legacy and new cache files exist | `for_openai()` reads `_openai.json` first; legacy is only tried if new file missing |

---

### Implications for Implementation

1. **Phases 1+2 are the critical foundation.** Once `ModelInfo` has capability fields and the catalog has values, the capping in `with_config_overrides` ensures correct context windows for all Claude models without any display-layer workarounds.

2. **Phase 10 becomes a pure deletion.** The `anthropic_model_context_window()` workaround was added because the catalog lacked Claude context windows. With catalog values + capping, the workaround is provably unnecessary.

3. **Phase 8 is a safe refactor.** The bridge function's hardcoded lists will produce identical behavior when driven from catalog data — the capability values in Phase 2's table match the current hardcoded logic exactly.
