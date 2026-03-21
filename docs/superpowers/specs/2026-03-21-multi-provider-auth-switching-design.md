# Multi-Provider Auth Switching (v2)

> **Date:** 2026-03-21
> **Revised:** 2026-03-21 (post-audit rework)
> **Status:** Approved
> **Approach:** TUI-Only, Phase 1 (standalone `tui` only, `tui_app_server` deferred)

## Problem

Users cannot switch between auth methods (API key vs OAuth) or add credentials for a new provider mid-session. The backend storage only holds one `ProviderAuth` per provider, so storing both an API key AND OAuth tokens for the same provider is impossible. The TUI has no flow to prompt for credentials when switching to an unauthenticated provider.

## Audit Findings Addressed

| Audit Issue | Resolution |
|-------------|------------|
| Storage only holds one `ProviderAuth` per provider | Redesign to `ProviderCredentialSet` holding both API key + OAuth |
| `tui_app_server` mirror ignores server-driven auth flows | Phase 1 scoped to standalone `tui` only. `tui_app_server` deferred to Phase 2. |
| Env var vs stored credential picker unsupported | Dropped. Env vars keep existing hardcoded precedence. |
| "Paste Token" underspecified for OAuth | Dropped. OAuth = browser flow only. |
| Growing `chatwidget.rs` further | New `chatwidget/auth_popup.rs` submodule instead. |
| Tests pointed at nonexistent `persistence_tests.rs` | Tests go in existing `auth_tests.rs` and `storage_tests.rs`. |
| `just write-config-schema` not relevant to auth storage | Removed from post-change steps. |

## Design Decisions

| Decision | Choice |
|----------|--------|
| When auth picker appears | Always when switching providers, even if credentials exist |
| Auth methods per provider | API key + OAuth for both OpenAI and Anthropic |
| Popup structure | Three-step sequential: model -> effort -> auth method |
| API key entry | Inline masked text input in TUI popup |
| OAuth entry | Browser login flow only (no token paste) |
| Auth persistence | Persist preferred auth mode inside `ProviderCredentialSet` |
| Standalone management | Dedicated `/auth` command |
| Scope | Phase 1: standalone `tui` only. Phase 2: `tui_app_server` |

## Data Model

### Current (single credential per provider)

```
AuthDotJsonV2 {
  version: 2,
  providers: HashMap<ProviderName, ProviderAuth>
}
```

`ProviderAuth` is a tagged enum — only ONE variant stored per provider.

### New (credential set per provider)

```
AuthDotJsonV3 {
  version: 3,
  providers: HashMap<ProviderName, ProviderCredentialSet>
}

ProviderCredentialSet {
  preferred_mode: Option<AuthMode>,
  api_key: Option<String>,                    // OpenAI or Anthropic API key
  oauth: Option<OAuthCredential>,             // OAuth tokens (provider-specific)
  chatgpt_tokens: Option<ChatgptTokenData>,   // OpenAI ChatGPT managed tokens
  external_tokens: Option<ChatgptTokenData>,  // OpenAI external (app-server) tokens
}

OAuthCredential {
  access_token: String,
  refresh_token: String,
  expires_at: i64,  // Unix seconds
}

ChatgptTokenData {
  tokens: TokenData,
  last_refresh: Option<DateTime<Utc>>,
}
```

### Migration

- V1 (legacy `AuthDotJson`) -> V3: existing v1->v2 migration logic, then v2->v3
- V2 -> V3: map each `ProviderAuth` variant into the appropriate field of `ProviderCredentialSet`
  - `OpenAiApiKey { key }` -> `api_key: Some(key)`
  - `Chatgpt { tokens, last_refresh }` -> `chatgpt_tokens: Some(ChatgptTokenData { tokens, last_refresh })`
  - `ChatgptAuthTokens { tokens, last_refresh }` -> `external_tokens: Some(ChatgptTokenData { tokens, last_refresh })`
  - `AnthropicApiKey { key }` -> `api_key: Some(key)`
  - `AnthropicOAuth { access_token, refresh_token, expires_at }` -> `oauth: Some(OAuthCredential { ... })`
- `preferred_mode` is inferred from whichever field was populated during migration

### Compatibility

- `deserialize_auth()` tries v3 first, then v2 (auto-migrates), then v1 (auto-migrates)
- `to_v1_openai()` still works for backward compat with code expecting legacy format
- `save` always writes v3 format
- Merge-on-save: when saving a `ProviderCredentialSet`, merge individual fields (don't replace the whole set)

### `auth_cached_for_provider()` Changes

The `AuthManager` method that resolves credentials for a provider needs to respect `preferred_mode`:

```
auth_cached_for_provider(provider):
  load ProviderCredentialSet for provider
  if preferred_mode == ApiKey and api_key is Some -> return ApiKey auth
  if preferred_mode == OAuth and oauth is Some -> return OAuth auth
  if preferred_mode == Chatgpt and chatgpt_tokens is Some -> return Chatgpt auth
  // fallback: return whichever credential exists (existing precedence logic)
```

Env var precedence is unchanged — env vars still checked per existing hardcoded logic (OpenAI checks env first, Anthropic checks storage first).

## Model Switch Flow

### Current (two steps)

```
/model -> pick model -> pick effort -> apply
```

### New (three steps)

```
/model -> pick model -> pick effort -> auth method popup -> apply
```

### Auth Step Logic

```
User picks effort -> determine target provider from model slug
  |-- Same provider as current? -> skip auth popup, apply immediately
  |-- Different provider?
       -> Load ProviderCredentialSet for target provider
       -> Build auth popup items based on what exists
       -> User picks method
       -> If new credentials needed, show inline input or browser flow
       -> Save credentials + preferred_mode
       -> Apply model switch
```

## Auth Popup UI

### No credentials exist for target provider

```
Select Authentication for Anthropic

> 1. API Key          Enter your Anthropic API key
  2. OAuth Login      Sign in with your Anthropic account

  Press enter to confirm or esc to go back.
```

### API key stored, OAuth also stored

```
Select Authentication for Anthropic

> 1. API Key (current)   sk-ant-*******dk3F
  2. Enter new API Key   Replace with a different key
  3. OAuth               Signed in (use stored OAuth tokens)
  4. OAuth Login          Re-authenticate with OAuth

  Press enter to confirm or esc to go back.
```

### Only API key stored

```
Select Authentication for Anthropic

> 1. API Key (current)   sk-ant-*******dk3F
  2. Enter new API Key   Replace with a different key
  3. OAuth Login         Sign in with your Anthropic account

  Press enter to confirm or esc to go back.
```

### API Key Inline Input

```
Enter Anthropic API Key

  Paste your key and press Enter. It will be stored securely.

  Key: sk-ant-******************|

  Press enter to save or esc to cancel.
```

Key is masked as user types. Basic format validation on Enter:
- Anthropic: starts with `sk-ant-`
- OpenAI: starts with `sk-`
On failure: inline error, retry without leaving popup.

### OAuth Browser Flow

Selecting "OAuth Login" triggers the existing headless OAuth flow:
- OpenAI: existing `headless_chatgpt_login` in `tui/src/onboarding/auth/`
- Anthropic: existing Anthropic OAuth flow (browser URL + callback)

No "paste token" option — OAuth always goes through the browser flow.

## /auth Command

Standalone command for managing credentials without switching models.

### Status View

```
/auth

+--------------------------------------------------+
|  Authentication Status                           |
|                                                  |
|  OpenAI:     OAuth (active)                      |
|              API Key: sk-*****3kF (stored)       |
|                                                  |
|  Anthropic:  API Key (active): sk-ant-***dk3F    |
|              OAuth: not configured               |
|                                                  |
|  1. Manage OpenAI                                |
|  2. Manage Anthropic                             |
|                                                  |
|  Press enter to select or esc to dismiss.        |
+--------------------------------------------------+
```

Shows ALL stored credentials per provider, not just the active one.

### Manage Provider

```
Manage Anthropic

> 1. API Key (current)     sk-ant-***dk3F
  2. Enter new API Key     Replace with a different key
  3. OAuth Login           Sign in with your Anthropic account
  4. Remove credentials    Delete stored Anthropic auth

  Press enter to confirm or esc to go back.
```

- Switching methods: updates `preferred_mode`, both credential sets remain stored
- "Remove credentials": destructive, confirmation prompt, clears entire `ProviderCredentialSet` for that provider

## Error Handling

| Scenario | Behavior |
|----------|----------|
| Invalid API key format | Inline error: "Invalid key format. Anthropic keys start with sk-ant-". Retry without leaving popup. |
| OAuth token expired mid-session | Existing refresh logic handles automatically. If refresh fails: "Anthropic auth expired. Run /auth to re-authenticate." |
| API key rejected (401) | Existing 401 recovery state machine. If recovery fails: "API key rejected. Run /auth to update." |
| Esc at auth step | Cancels entire model switch. No credentials changed. |
| Both API key and OAuth exist | `preferred_mode` determines which is active. Selecting the other updates `preferred_mode`. Both credential sets remain stored. |
| Remove credentials while env var set | Stored credentials removed. Env var still works on next request via existing precedence. |

## File Changes

### Core (storage redesign)

| File | Change |
|------|--------|
| `core/src/auth/storage.rs` | Add `ProviderCredentialSet`, `OAuthCredential`, `ChatgptTokenData`, `AuthDotJsonV3`. V2->V3 migration. Update `deserialize_auth()`. |
| `core/src/auth/storage_tests.rs` | Tests for v3 round-trip, v2->v3 migration, credential set operations. |
| `core/src/auth/persistence.rs` | Update `save_auth_v2` -> `save_auth_v3` with field-level merge. Update `load_auth_dot_json_v2` -> `load_auth_dot_json_v3`. |
| `core/src/auth_tests.rs` | Tests for preferred_mode resolution in auth_cached_for_provider. |
| `core/src/auth/manager.rs` | Update `auth_cached_for_provider()` to respect `preferred_mode` from `ProviderCredentialSet`. |

### TUI (Phase 1 — standalone only)

| File | Change |
|------|--------|
| `tui/src/slash_command.rs` | Add `Auth` variant. |
| `tui/src/chatwidget/auth_popup.rs` | NEW submodule: `open_auth_popup()`, `open_api_key_input()`, masked input view, provider detection helpers. |
| `tui/src/chatwidget.rs` | Modify `apply_model_and_effort()` to insert auth step. Wire `/auth` command. Delegate to `auth_popup` submodule. |
| `tui/src/chatwidget/tests.rs` | Snapshot tests for auth popups. |

### Deferred to Phase 2

- `tui_app_server/` — requires provider-scoped `account/read`, `account/logout`, model catalog refresh
- `app-server-protocol/` — may need provider-scoped account RPC methods
- `app-server/` — server-side provider-scoped auth handling

### No changes to

- Protocol (`Op`, `EventMsg`)
- `anthropic_bridge.rs`
- `client.rs` (uses `auth_cached_for_provider` which we update)
