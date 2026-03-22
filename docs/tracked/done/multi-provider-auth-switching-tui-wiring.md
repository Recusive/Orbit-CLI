# Plan: Complete Multi-Provider Auth Switching TUI Layer

> **Original plan:** `docs/superpowers/plans/2026-03-21-multi-provider-auth-switching.md`
> **Design spec:** `docs/superpowers/specs/2026-03-21-multi-provider-auth-switching-design.md`
> **Testing guide:** `docs/architects/orbit-code-testing-guide.md`
> **Audit:** `reviews/multi-provider-auth-switching-tui-wiring.audit.md`

## Context

Tasks 1–4 (core storage layer) are solid and tested. The TUI layer (Tasks 5–7) compiles but has **stub action closures** — selecting items in the `/auth` popup and model-switch auth popup does nothing. Additionally, the `/auth` status display shows "not configured" for Anthropic even when `ANTHROPIC_API_KEY` env var is set. This plan finishes the TUI wiring to make everything functional end-to-end per the original spec.

**Scope:** Phase 1 standalone `tui` only. `tui_app_server` has no `/auth` command today and its `model/list` results are already filtered by the server's active auth context. App-server v2, SDKs, and protocol changes are not needed.

## What's Already Wired (no changes needed)

- `SlashCommand::Auth` dispatch → `on_slash_auth()` (chatwidget.rs:4505)
- Model-switch provider intercept → `open_auth_popup()` (chatwidget.rs:6940-6954)
- `open_auth_popup()` signature fixed: effort is `Option<ReasoningEffortConfig>` not `Option<String>`
- Core: `alternate_credentials`, `preferred_auth_modes`, `restore_alternate_credential()`, `remove_all_credentials()` — all tested

## Audit-Driven Fixes (Critical Issues)

### C1: Delete flow must use `logout_provider()`, not `save_auth_v2()`

`save_auth_v2()` only merges present entries — it cannot remove. The "Remove credentials" action must call:
```rust
orbit_code_core::auth::logout_provider(&orbit_code_home, provider, store_mode)
```
This reaches the storage backend's `delete_provider()` path which properly clears active + alternate + preferred_auth_modes.

### C2: `AuthManager::reload()` after every same-provider mutation

`AuthManager` caches auth and does not observe external storage changes. After any successful swap or delete from `/auth`:
```rust
// AuthManager is accessible via the thread manager on ChatWidget
widget.reload_auth_manager();
```
Add a `reload_auth_manager()` helper on ChatWidget that reaches the thread manager's auth manager. Without this, `/auth` credential swaps have no effect on the current session.

### C3: Env var display must reflect effective credential, not hard-coded labels

The status display must show two distinct concepts:
- **Effective credential**: derived from `AuthManager::auth_cached_for_provider(provider)` — what's actually used for requests
- **Available credentials**: stored credentials (from `load_auth_dot_json_v2`) + env var presence (probe `std::env::var`)

Display format:
- `"OAuth (active) | API Key: sk-ant-***dk3F (stored) | ANTHROPIC_API_KEY (env)"` — when all three exist
- `"API Key (active via env var)"` — when only env var exists, no stored credentials
- `"not configured"` — when nothing exists

Do NOT label env var auth as "stored" or "active" based on guessing — derive effective status from `AuthManager`.

### C4: Snapshot tests must ship with this change

Repo conventions require `insta` snapshot coverage for any user-visible TUI change. Add popup rendering snapshots in `tui/src/chatwidget/tests.rs`:
1. Auth status popup — no credentials for a provider
2. Auth status popup — API key active + OAuth alternate
3. Auth status popup — env var only
4. Provider management sub-popup — with active + alternate
5. Model-switch auth popup — active credential pre-selected

## Changes

### 1. Add `AppEvent::ManageAuthProvider` variant

**File:** `codex-rs/tui/src/app_event.rs`

```rust
/// Open the auth management popup for a specific provider (from /auth).
ManageAuthProvider {
    provider: orbit_code_core::auth::ProviderName,
},
```

### 2. Add dispatch handler in `app.rs`

**File:** `codex-rs/tui/src/app.rs` (near line 2795, after `OpenAllModelsPopup`)

```rust
AppEvent::ManageAuthProvider { provider } => {
    self.chat_widget.open_auth_provider_management(provider);
}
```

### 3. Rewrite `on_slash_auth()` — wire action closures + env var display

**File:** `codex-rs/tui/src/chatwidget/auth_popup.rs`

Replace the stub closures. Each provider's action sends `AppEvent::ManageAuthProvider { provider }` via `tx.send()`.

For the status display, probe three sources per provider:
1. `v2.provider_auth(provider)` — active stored credential
2. `v2.alternate_credentials.get(&provider)` — alternate stored credential
3. `std::env::var(env_var_name)` — env var presence

Build description string from all three. Use `env_var_for_provider()` helper:
```rust
fn env_var_for_provider(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::Anthropic => "ANTHROPIC_API_KEY",
        ProviderName::OpenAI => "OPENAI_API_KEY",
    }
}
```

### 4. Add `open_auth_provider_management()` method

**File:** `codex-rs/tui/src/chatwidget/auth_popup.rs` (new `pub(super)` function)

Sub-popup for managing a single provider from `/auth`. Shows:

- **Active credential** (if exists) — selecting is a no-op, keeps current
- **Stored alternate** (if exists) — calls `restore_alternate_credential()` + updates `preferred_auth_modes` + saves via `save_auth_v2()` + calls `AuthManager::reload()` via widget
- **"Enter new API Key"** — info message varies by provider:
  - Anthropic: `"Run: just codex login --provider anthropic"`
  - OpenAI: `"Run: printenv OPENAI_API_KEY | just codex login --with-api-key"`
- **"OAuth Login"** — info message: `"Mid-session OAuth switching is not yet available. Use /logout to clear current credentials, then restart Orbit Code to reach the onboarding OAuth flow."` (Anthropic CLI OAuth is not implemented; OpenAI ChatGPT OAuth requires the TUI onboarding flow which only runs when unauthenticated)
- **"Remove credentials"** — calls `logout_provider()` (NOT `remove_all_credentials` + `save_auth_v2`) + calls `AuthManager::reload()` via widget

**File:** `codex-rs/tui/src/chatwidget.rs`

Add delegate method:
```rust
fn open_auth_provider_management(&mut self, provider: ProviderName) {
    auth_popup::open_auth_provider_management(self, provider);
}
```

### 5. Rewrite `open_auth_popup()` — wire model-switch actions + env var awareness

**File:** `codex-rs/tui/src/chatwidget/auth_popup.rs`

**Before building the popup**, check if env-var auth is already effective for the target provider:
```rust
let has_effective_auth = widget.auth_manager_has_auth_for_provider(target_provider);
```
If effective auth exists (from any source — stored, alternate, or env var) and no stored credentials exist, the env var is the effective source. Show it as a selectable option:
- `"API Key (active via env var)"` — selecting applies the model switch directly

**Env-var-only fast path:** If the only auth source is an env var (no stored credentials, no alternate), skip the popup entirely and apply the model switch immediately. The user doesn't need to "choose" when there's only one option.

Replace all stub closures:

**Active credential selected:**
```rust
tx.send(AppEvent::UpdateModel(model.clone()));
tx.send(AppEvent::UpdateReasoningEffort(effort));
tx.send(AppEvent::PersistModelSelection { model, effort });
```

**Alternate credential selected:**
1. Load v2 → `restore_alternate_credential()` → update `preferred_auth_modes` → `save_auth_v2()` (inline in closure, sync I/O)
2. `AuthManager::reload()` via widget
3. Then send the same 3 AppEvents to apply model switch

**Env-var credential selected:**
Apply model switch directly (env var auth is picked up by `auth_cached_for_provider` automatically).

**No credentials at all (no stored, no env var):**
Info message: `"No credentials found for {provider}. Run: just codex login --provider anthropic"` (with correct provider-specific command).

Also add "Enter new API Key" and "OAuth Login" options when credentials exist (spec requires them for switching methods).

### 6. Add auth manager helpers on ChatWidget

**File:** `codex-rs/tui/src/chatwidget.rs`

Two helpers needed:

```rust
/// Reload auth from storage after a mutation (swap, delete).
fn reload_auth_manager(&self) { ... }

/// Check if effective auth exists for a provider (stored, alternate, or env var).
fn auth_manager_has_auth_for_provider(&self, provider: ProviderName) -> bool { ... }
```

Both need to reach the `AuthManager` through whatever path ChatWidget has to the thread manager. Check how existing auth reload works (e.g., after logout at line 4496-4502).

### 7. Local `auth_mode_for_provider_auth()` in auth_popup.rs

Per audit recommendation, keep a local 5-arm match instead of re-exporting from core:

```rust
fn auth_mode_for_provider_auth(auth: &ProviderAuth) -> orbit_code_app_server_protocol::AuthMode {
    use orbit_code_app_server_protocol::AuthMode;
    match auth {
        ProviderAuth::OpenAiApiKey { .. } => AuthMode::ApiKey,
        ProviderAuth::Chatgpt { .. } => AuthMode::Chatgpt,
        ProviderAuth::ChatgptAuthTokens { .. } => AuthMode::ChatgptAuthTokens,
        ProviderAuth::AnthropicApiKey { .. } => AuthMode::AnthropicApiKey,
        ProviderAuth::AnthropicOAuth { .. } => AuthMode::AnthropicOAuth,
    }
}
```

### 8. Snapshot tests

**File:** `codex-rs/tui/src/chatwidget/tests.rs`

Add 5 snapshot tests per C4 above. Follow existing patterns in `chatwidget/tests.rs` — render popup to ratatui `Buffer`, compare with `insta::assert_snapshot!`.

## Files Modified

| File | Change |
|------|--------|
| `tui/src/app_event.rs` | Add `ManageAuthProvider { provider }` variant |
| `tui/src/app.rs` | Add dispatch: `ManageAuthProvider → chat_widget.open_auth_provider_management()` |
| `tui/src/chatwidget/auth_popup.rs` | Rewrite `on_slash_auth()`, `open_auth_popup()`, add `open_auth_provider_management()`, add env var + effective credential display, add local `auth_mode_for_provider_auth()` |
| `tui/src/chatwidget.rs` | Add `open_auth_provider_management` delegate, add `reload_auth_manager()` helper |
| `tui/src/chatwidget/tests.rs` | Add 5 snapshot tests for auth popups |

## Edge Cases To Handle

- **Remove credentials while env var set** → `logout_provider()` clears stored creds, `AuthManager::reload()` picks up env var auth. Status shows `"API Key (active via env var)"`.
- **Cancel remove confirmation / Esc from sub-popup** → no-op, credentials unchanged. `dismiss_on_select: false` for remove option; separate confirmation popup.
- **Storage backend (keyring/auto) save failure** → log error, show info message in TUI, do NOT leave UI in half-mutated state.
- **Preferred mode points to alternate while env var also exists** → `AuthManager::reload()` resolves effective auth correctly; status display shows all three sources.

## What's Deferred to Phase 2

- **Masked API key inline input** — needs a new input widget, not a SelectionView
- **OAuth browser flow trigger** — needs async integration with `headless_chatgpt_login` / Anthropic OAuth
- **Model catalog refresh after auth change** — catalog may show stale models until next refresh cycle

These show honest info messages directing users to the onboarding flow.

## Verification

1. `just fmt` — clean
2. `cargo clippy -p orbit-code-core -p orbit-code-tui` — zero errors/warnings
3. `cargo test -p orbit-code-core` — all 9 new + existing tests pass
4. `cargo test -p orbit-code-tui` — all tests pass including new snapshots
5. `cargo insta pending-snapshots -p orbit-code-tui` — no pending snapshots
6. **Manual TUI test:**
   - `just codex` → `/auth` → verify Anthropic shows `"API Key (active via env var)"` not `"not configured"`
   - `/auth` → select a provider → see management sub-popup with credential options
   - `/auth` → select provider with alternate → select alternate → verify swap persists and session uses new credential
   - `/model` → select a Claude model → auth popup appears if switching from OpenAI → selecting credential applies the model switch
