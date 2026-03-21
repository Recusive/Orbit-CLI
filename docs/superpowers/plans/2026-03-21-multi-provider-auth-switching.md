# Multi-Provider Auth Switching Implementation Plan (v2)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let users switch between API key and OAuth authentication for both OpenAI and Anthropic providers mid-session, via a third step in the `/model` flow and a new `/auth` command.

**Architecture:** Redesign auth storage from single-credential-per-provider (`ProviderAuth` enum) to multi-credential-per-provider (`ProviderCredentialSet` struct). Add auth popup as a new `chatwidget/auth_popup.rs` submodule. Phase 1 covers standalone `tui` only — `tui_app_server` deferred.

**Tech Stack:** Rust, ratatui, orbit-code-core auth module, orbit-code-tui SelectionView popups.

**Spec:** `docs/superpowers/specs/2026-03-21-multi-provider-auth-switching-design.md`

---

## File Structure

| File | Responsibility |
|------|---------------|
| `core/src/auth/storage.rs` | New `ProviderCredentialSet`, `OAuthCredential`, `ChatgptTokenData` types. V2->V3 migration. |
| `core/src/auth/storage_tests.rs` | V3 round-trip, v2->v3 migration, credential set operations |
| `core/src/auth/persistence.rs` | Field-level merge on save. Load with v3 support. |
| `core/src/auth_tests.rs` | `preferred_mode` resolution in `auth_cached_for_provider` |
| `core/src/auth/manager.rs` | `auth_cached_for_provider()` respects `preferred_mode` |
| `tui/src/slash_command.rs` | Add `Auth` variant |
| `tui/src/chatwidget/auth_popup.rs` | NEW: auth popup logic, API key input, provider detection |
| `tui/src/chatwidget.rs` | Wire auth step into model-switch, wire `/auth` command |
| `tui/src/chatwidget/tests.rs` | Snapshot tests for auth popups |

---

### Task 1: Define `ProviderCredentialSet` and V3 storage types

**Files:**
- Modify: `codex-rs/core/src/auth/storage.rs`
- Modify: `codex-rs/core/src/auth/storage_tests.rs`

- [ ] **Step 1: Write failing test for `ProviderCredentialSet` round-trip**

In `storage_tests.rs`:

```rust
#[test]
fn v3_credential_set_round_trips() {
    let mut v3 = AuthDotJsonV3::new();
    let mut creds = ProviderCredentialSet::new();
    creds.api_key = Some("sk-ant-test123".to_string());
    creds.preferred_mode = Some(AuthMode::AnthropicApiKey);
    v3.set_credentials(ProviderName::Anthropic, creds);

    let json = serde_json::to_string_pretty(&v3).expect("serialize");
    let loaded: AuthDotJsonV3 = serde_json::from_str(&json).expect("deserialize");

    let loaded_creds = loaded.credentials(ProviderName::Anthropic).expect("anthropic");
    assert_eq!(loaded_creds.api_key.as_deref(), Some("sk-ant-test123"));
    assert_eq!(loaded_creds.preferred_mode, Some(AuthMode::AnthropicApiKey));
    assert!(loaded_creds.oauth.is_none());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p orbit-code-core -- storage_tests::v3_credential_set_round_trips`
Expected: FAIL — types don't exist yet.

- [ ] **Step 3: Add new types to `storage.rs`**

Add after existing `AuthDotJsonV2`:

```rust
/// OAuth credential (provider-agnostic).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuthCredential {
    pub access_token: String,
    pub refresh_token: String,
    /// Unix timestamp in seconds when the access token expires.
    pub expires_at: i64,
}

/// ChatGPT-specific token data (OpenAI OAuth and external tokens).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatgptTokenData {
    pub tokens: TokenData,
    pub last_refresh: Option<DateTime<Utc>>,
}

/// Per-provider credential set. Can hold both API key AND OAuth simultaneously.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProviderCredentialSet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_mode: Option<AuthMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<OAuthCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chatgpt_tokens: Option<ChatgptTokenData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_tokens: Option<ChatgptTokenData>,
}

impl ProviderCredentialSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_any_auth(&self) -> bool {
        self.api_key.is_some()
            || self.oauth.is_some()
            || self.chatgpt_tokens.is_some()
            || self.external_tokens.is_some()
    }
}

/// V3 auth storage — multi-credential per provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthDotJsonV3 {
    pub version: u32,
    pub providers: HashMap<ProviderName, ProviderCredentialSet>,
}

impl AuthDotJsonV3 {
    pub fn new() -> Self {
        Self {
            version: 3,
            providers: HashMap::new(),
        }
    }

    pub fn credentials(&self, provider: ProviderName) -> Option<&ProviderCredentialSet> {
        self.providers.get(&provider)
    }

    pub fn credentials_mut(&mut self, provider: ProviderName) -> &mut ProviderCredentialSet {
        self.providers.entry(provider).or_default()
    }

    pub fn set_credentials(&mut self, provider: ProviderName, creds: ProviderCredentialSet) {
        self.providers.insert(provider, creds);
    }

    pub fn remove_credentials(&mut self, provider: ProviderName) -> Option<ProviderCredentialSet> {
        self.providers.remove(&provider)
    }

    pub fn has_any_auth(&self) -> bool {
        self.providers.values().any(ProviderCredentialSet::has_any_auth)
    }
}

impl Default for AuthDotJsonV3 {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p orbit-code-core -- storage_tests::v3_credential_set_round_trips`
Expected: PASS

- [ ] **Step 5: Run `just fmt` and commit**

```bash
just fmt
git add codex-rs/core/src/auth/storage.rs codex-rs/core/src/auth/storage_tests.rs
git commit -m "feat(auth): add ProviderCredentialSet and AuthDotJsonV3 types"
```

---

### Task 2: V2 -> V3 migration

**Files:**
- Modify: `codex-rs/core/src/auth/storage.rs`
- Modify: `codex-rs/core/src/auth/storage_tests.rs`

- [ ] **Step 1: Write failing test for v2->v3 migration**

```rust
#[test]
fn v2_migrates_to_v3() {
    let mut v2 = AuthDotJsonV2::new();
    v2.set_provider_auth(
        ProviderName::Anthropic,
        ProviderAuth::AnthropicApiKey { key: "sk-ant-key".to_string() },
    );
    v2.set_provider_auth(
        ProviderName::OpenAI,
        ProviderAuth::OpenAiApiKey { key: "sk-openai".to_string() },
    );

    let v3 = AuthDotJsonV3::from(v2);

    let anthropic = v3.credentials(ProviderName::Anthropic).expect("anthropic");
    assert_eq!(anthropic.api_key.as_deref(), Some("sk-ant-key"));
    assert!(anthropic.oauth.is_none());

    let openai = v3.credentials(ProviderName::OpenAI).expect("openai");
    assert_eq!(openai.api_key.as_deref(), Some("sk-openai"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Expected: FAIL — `From<AuthDotJsonV2> for AuthDotJsonV3` not implemented.

- [ ] **Step 3: Implement `From<AuthDotJsonV2> for AuthDotJsonV3`**

```rust
impl From<AuthDotJsonV2> for AuthDotJsonV3 {
    fn from(v2: AuthDotJsonV2) -> Self {
        let mut v3 = AuthDotJsonV3::new();
        for (provider, auth) in v2.providers {
            let creds = v3.credentials_mut(provider);
            match auth {
                ProviderAuth::OpenAiApiKey { key } => {
                    creds.api_key = Some(key);
                    creds.preferred_mode = Some(AuthMode::ApiKey);
                }
                ProviderAuth::Chatgpt { tokens, last_refresh } => {
                    creds.chatgpt_tokens = Some(ChatgptTokenData { tokens, last_refresh });
                    creds.preferred_mode = Some(AuthMode::Chatgpt);
                }
                ProviderAuth::ChatgptAuthTokens { tokens, last_refresh } => {
                    creds.external_tokens = Some(ChatgptTokenData { tokens, last_refresh });
                    creds.preferred_mode = Some(AuthMode::ChatgptAuthTokens);
                }
                ProviderAuth::AnthropicApiKey { key } => {
                    creds.api_key = Some(key);
                    creds.preferred_mode = Some(AuthMode::AnthropicApiKey);
                }
                ProviderAuth::AnthropicOAuth { access_token, refresh_token, expires_at } => {
                    creds.oauth = Some(OAuthCredential { access_token, refresh_token, expires_at });
                    creds.preferred_mode = Some(AuthMode::AnthropicOAuth);
                }
            }
        }
        v3
    }
}
```

- [ ] **Step 4: Update `deserialize_auth()` to try v3 first**

```rust
pub(super) fn deserialize_auth(json: &str) -> Result<AuthDotJsonV3, serde_json::Error> {
    // Try v3 first
    if let Ok(v3) = serde_json::from_str::<AuthDotJsonV3>(json)
        && v3.version == 3
    {
        return Ok(v3);
    }
    // Try v2 and migrate
    if let Ok(v2) = serde_json::from_str::<AuthDotJsonV2>(json)
        && v2.version == 2
    {
        return Ok(AuthDotJsonV3::from(v2));
    }
    // Fall back to v1 -> v2 -> v3
    let v1: AuthDotJson = serde_json::from_str(json)?;
    Ok(AuthDotJsonV3::from(AuthDotJsonV2::from(v1)))
}
```

- [ ] **Step 5: Write test for v1->v3 migration chain**

```rust
#[test]
fn v1_migrates_through_v2_to_v3() {
    let json = r#"{"auth_mode":"apikey","openai_api_key":"sk-old"}"#;
    let v3 = deserialize_auth(json).expect("deserialize");
    let openai = v3.credentials(ProviderName::OpenAI).expect("openai");
    assert_eq!(openai.api_key.as_deref(), Some("sk-old"));
}
```

- [ ] **Step 6: Run all tests, `just fmt`, commit**

```bash
just fmt
cargo test -p orbit-code-core -- storage_tests
git add codex-rs/core/src/auth/storage.rs codex-rs/core/src/auth/storage_tests.rs
git commit -m "feat(auth): v2->v3 migration with multi-credential storage"
```

---

### Task 3: Update `AuthStorageBackend` and persistence to use V3

**Files:**
- Modify: `codex-rs/core/src/auth/storage.rs` (backend trait + impls)
- Modify: `codex-rs/core/src/auth/persistence.rs`
- Modify: `codex-rs/core/src/auth_tests.rs`

- [ ] **Step 1: Update `AuthStorageBackend` trait from V2 to V3**

Change all `load() -> Option<AuthDotJsonV2>` to `load() -> Option<AuthDotJsonV3>` and all `save(&AuthDotJsonV2)` to `save(&AuthDotJsonV3)`. Update `FileAuthStorage`, `KeyringAuthStorage`, `AutoAuthStorage`, `EphemeralAuthStorage` implementations.

- [ ] **Step 2: Update `save_auth_v2` to field-level merge on V3**

Rename to `save_auth_v3` (keep `save_auth_v2` as thin wrapper for callers). The merge logic becomes field-level:

```rust
pub fn save_auth_v3(
    orbit_code_home: &Path,
    auth: &AuthDotJsonV3,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let storage = create_auth_storage(orbit_code_home.to_path_buf(), auth_credentials_store_mode);
    let merged = match storage.load() {
        Ok(Some(mut existing)) => {
            for (provider, new_creds) in &auth.providers {
                let target = existing.credentials_mut(*provider);
                // Field-level merge: only overwrite fields that are Some in new_creds
                if new_creds.api_key.is_some() {
                    target.api_key = new_creds.api_key.clone();
                }
                if new_creds.oauth.is_some() {
                    target.oauth = new_creds.oauth.clone();
                }
                if new_creds.chatgpt_tokens.is_some() {
                    target.chatgpt_tokens = new_creds.chatgpt_tokens.clone();
                }
                if new_creds.external_tokens.is_some() {
                    target.external_tokens = new_creds.external_tokens.clone();
                }
                if new_creds.preferred_mode.is_some() {
                    target.preferred_mode = new_creds.preferred_mode;
                }
            }
            existing
        }
        Ok(None) | Err(_) => auth.clone(),
    };
    storage.save(&merged)
}
```

- [ ] **Step 3: Update `load_auth_dot_json_v2` -> `load_auth_dot_json_v3`**

Keep backward-compat wrapper.

- [ ] **Step 4: Update `to_v1_openai()` on V3 for backward compat**

Implement `AuthDotJsonV3::to_v1_openai()` that maps credential set fields back to the v1 `AuthDotJson` struct.

- [ ] **Step 5: Update `delete_provider` to clear entire credential set**

- [ ] **Step 6: Write test for field-level merge**

```rust
#[test]
fn save_auth_v3_merges_credential_fields() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Save API key for Anthropic
    let mut initial = AuthDotJsonV3::new();
    initial.credentials_mut(ProviderName::Anthropic).api_key = Some("sk-ant-key".to_string());
    save_auth_v3(dir.path(), &initial, AuthCredentialsStoreMode::File).expect("save");

    // Save OAuth for Anthropic (should preserve API key)
    let mut update = AuthDotJsonV3::new();
    let creds = update.credentials_mut(ProviderName::Anthropic);
    creds.oauth = Some(OAuthCredential {
        access_token: "at".to_string(),
        refresh_token: "rt".to_string(),
        expires_at: 999,
    });
    save_auth_v3(dir.path(), &update, AuthCredentialsStoreMode::File).expect("save");

    // Both should exist
    let loaded = load_auth_dot_json_v3(dir.path(), AuthCredentialsStoreMode::File)
        .expect("load").expect("some");
    let anthropic = loaded.credentials(ProviderName::Anthropic).expect("anthropic");
    assert_eq!(anthropic.api_key.as_deref(), Some("sk-ant-key"));
    assert!(anthropic.oauth.is_some());
}
```

- [ ] **Step 7: Run tests, `just fmt`, commit**

```bash
just fmt
cargo test -p orbit-code-core
git add codex-rs/core/src/auth/
git commit -m "feat(auth): update storage backend and persistence to V3 format"
```

---

### Task 4: Update `AuthManager.auth_cached_for_provider()` for `preferred_mode`

**Files:**
- Modify: `codex-rs/core/src/auth/manager.rs:169-238`
- Modify: `codex-rs/core/src/auth_tests.rs`

- [ ] **Step 1: Write failing test for preferred_mode resolution**

```rust
#[test]
fn auth_cached_for_provider_respects_preferred_mode() {
    // Setup: Anthropic has both API key and OAuth, preferred_mode = ApiKey
    // Assert: auth_cached_for_provider returns ApiKey auth, not OAuth
}
```

- [ ] **Step 2: Update `auth_cached_for_provider` to load V3 and respect `preferred_mode`**

When loading from v3 storage, check `preferred_mode` to decide which credential to return. Fall back to existing precedence when `preferred_mode` is `None`.

- [ ] **Step 3: Add helper `codex_auth_from_credential_set()`**

Maps a `ProviderCredentialSet` + `preferred_mode` to the appropriate `CodexAuth` variant.

- [ ] **Step 4: Run tests, `just fmt`, commit**

```bash
just fmt
cargo test -p orbit-code-core
git add codex-rs/core/src/auth/manager.rs codex-rs/core/src/auth_tests.rs
git commit -m "feat(auth): auth_cached_for_provider respects preferred_mode from V3"
```

---

### Task 5: Add `/auth` slash command

**Files:**
- Modify: `codex-rs/tui/src/slash_command.rs`

- [ ] **Step 1: Add `Auth` variant**

Add `Auth` after `Model` in the enum. Description: `"manage authentication for model providers"`. `available_during_task`: false.

- [ ] **Step 2: Run `just fmt` and commit**

```bash
just fmt
git add codex-rs/tui/src/slash_command.rs
git commit -m "feat(tui): add /auth slash command variant"
```

---

### Task 6: Create `chatwidget/auth_popup.rs` submodule

**Files:**
- Create: `codex-rs/tui/src/chatwidget/auth_popup.rs`
- Modify: `codex-rs/tui/src/chatwidget.rs` (add `mod auth_popup;`)

This is the core UI logic. Keeping it in a separate submodule per audit recommendation to avoid growing `chatwidget.rs` further.

- [ ] **Step 1: Create `auth_popup.rs` with provider detection helper**

```rust
//! Auth method selection popup for mid-session provider switching.

use orbit_code_core::auth::storage::ProviderName;

pub(crate) fn provider_for_model(slug: &str) -> ProviderName {
    if slug.starts_with("claude-") {
        ProviderName::Anthropic
    } else {
        ProviderName::OpenAI
    }
}

pub(crate) fn mask_credential(value: &str) -> String {
    if value.len() <= 10 {
        return "*".repeat(value.len());
    }
    let prefix = &value[..7];
    let suffix = &value[value.len() - 3..];
    format!("{prefix}*******{suffix}")
}

pub(crate) fn provider_display_name(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::OpenAI => "OpenAI",
        ProviderName::Anthropic => "Anthropic",
    }
}
```

- [ ] **Step 2: Add `open_auth_popup()` function**

Takes `ChatWidget` context (via `&mut self` on an impl block or as free function taking needed params), `target_provider`, `model`, `effort`, `is_standalone` flag.

Builds `SelectionItem` list based on what credentials exist in `ProviderCredentialSet`. Pre-highlights item matching `preferred_mode`. Uses `SelectionViewParams` pattern from `open_reasoning_popup()`.

- [ ] **Step 3: Add `open_api_key_input()` function**

Masked text input popup. On Enter: validates format, saves via `save_auth_v3()`, updates `preferred_mode`, applies model switch if not standalone.

- [ ] **Step 4: Add `on_slash_auth()` — provider picker + status display**

Shows provider list with credential status summary. On selection, opens `open_auth_popup()` with `is_standalone: true`.

- [ ] **Step 5: Add `mod auth_popup;` to `chatwidget.rs`**

- [ ] **Step 6: Run `just fmt` and commit**

```bash
just fmt
git add codex-rs/tui/src/chatwidget/auth_popup.rs codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): create auth_popup submodule with popup logic"
```

---

### Task 7: Wire auth popup into model-switch flow

**Files:**
- Modify: `codex-rs/tui/src/chatwidget.rs`

- [ ] **Step 1: Modify effort popup selection handler**

After effort is selected, before calling `apply_model_and_effort()`:

```rust
let target_provider = auth_popup::provider_for_model(&selected_model);
let current_provider = auth_popup::provider_for_model(self.current_model());

if target_provider != current_provider {
    self.open_auth_popup(target_provider, selected_model, selected_effort, false);
} else {
    self.apply_model_and_effort(selected_model, selected_effort);
}
```

- [ ] **Step 2: Wire `/auth` command in slash command dispatch**

In the match on `SlashCommand` variants, add:
```rust
SlashCommand::Auth => self.on_slash_auth(),
```

- [ ] **Step 3: Run `just fmt` and commit**

```bash
just fmt
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): wire auth popup into model-switch flow and /auth command"
```

---

### Task 8: Snapshot tests for auth popups

**Files:**
- Modify: `codex-rs/tui/src/chatwidget/tests.rs`

- [ ] **Step 1: Write snapshot test — auth popup no credentials**

- [ ] **Step 2: Write snapshot test — auth popup with existing API key**

- [ ] **Step 3: Write snapshot test — auth popup with both API key and OAuth**

- [ ] **Step 4: Write snapshot test — `/auth` status view**

- [ ] **Step 5: Run and accept snapshots**

```bash
cargo insta test -p orbit-code-tui --accept
```

- [ ] **Step 6: Run `just fmt` and commit**

```bash
just fmt
git add codex-rs/tui/src/chatwidget/tests.rs codex-rs/tui/src/chatwidget/snapshots/
git commit -m "test(tui): add snapshot tests for auth popups"
```

---

### Task 9: Final validation

- [ ] **Step 1: Run clippy on changed crates**

```bash
just fix -p orbit-code-core
just fix -p orbit-code-tui
```

- [ ] **Step 2: Run full test suites**

```bash
cargo test -p orbit-code-core
cargo test -p orbit-code-tui
```

- [ ] **Step 3: Run `just fmt`**

- [ ] **Step 4: Final commit if any lint/format changes**

```bash
just fmt
git add -A
git commit -m "chore: fix lint and format after auth switching implementation"
```
