# Plan: Full Inline Auth Management in TUI Popups (Phase 2)

> **Extension of:** `docs/tracked/done/multi-provider-auth-switching-tui-wiring.md`
> **Original plan:** `docs/superpowers/plans/2026-03-21-multi-provider-auth-switching.md`
> **Design spec:** `docs/superpowers/specs/2026-03-21-multi-provider-auth-switching-design.md`
> **Audit:** `reviews/multi-provider-auth-switching-inline-auth-phase2.audit.md`

## Summary

This plan extends the standalone `tui` auth-switching work so `/auth` and `/model` become fully self-contained auth-management surfaces. After this plan lands, users can add, replace, switch, or remove OpenAI and Anthropic credentials inline from the popup flow, without falling back to CLI login, `/logout`, or a restart.

This is a **standalone `tui` only** follow-up. `tui_app_server` uses RPC-driven auth and needs a separate parity plan once the standalone TUI flow has stabilized. This is an **explicit convention-54 exception**: the `tui_app_server` mirror is deferred to a tracked follow-up because the app-server auth path (`tui_app_server/src/onboarding/auth.rs`) is fundamentally RPC-driven and cannot share the standalone async login flows introduced here. Document this exception in `tui_app_server/CLAUDE.md` with a link back to this plan.

## Prerequisite

This plan assumes `multi-provider-auth-switching-tui-wiring.md` has landed, including:

- `/auth` provider-management popup
- `/model` cross-provider auth popup
- `ManageAuthProvider` event wiring
- `logout_provider()`-based delete flow
- env-var-aware provider switching
- snapshot coverage for the phase-1 popup flow

## Goals

- Replace the current placeholder `"Enter new API Key"` and `"OAuth Login"` actions with real inline flows
- Keep the user inside the popup flow for both `/auth` and `/model`
- Reuse existing onboarding auth logic where possible instead of copying it again
- Preserve current storage semantics: `providers`, `alternate_credentials`, `preferred_auth_modes`
- Apply model switches immediately after successful inline auth when the flow was launched from `/model`

## Non-Goals

- `tui_app_server` parity (tracked separately)
- New auth storage formats or protocol/schema changes
- New CLI flags or login commands
- Reworking provider semantics beyond OpenAI and Anthropic

## Architecture

### 1. Extract shared auth actions with attempt-scoped lifecycle

Create a new shared module:

- `codex-rs/tui/src/auth_flow.rs`

This module owns the provider-scoped auth operations used by both onboarding and popup flows. It is **not** a bag of stateless helper functions — it must model attempt-scoped async operations with explicit lifecycle boundaries.

#### Core types

```rust
/// Uniquely identifies one auth attempt. Used to guard against stale completions.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct AuthAttemptId(u64);

/// Handle for an in-progress async auth operation.
/// Drop cancels the underlying task.
pub(crate) struct AuthAttemptHandle {
    id: AuthAttemptId,
    cancel: CancellationToken,  // tokio_util::sync::CancellationToken
}

/// Result delivered by an async auth operation.
pub(crate) struct AuthAttemptResult {
    id: AuthAttemptId,
    outcome: Result<AuthSuccess, AuthError>,
}

pub(crate) enum AuthSuccess {
    ApiKeySaved { provider: ProviderName },
    OAuthComplete { provider: ProviderName },
}

pub(crate) enum AuthError {
    Cancelled,
    ExchangeFailed(String),
    StorageFailed(String),
}
```

#### Operations

Synchronous (return `Result` directly):
- `save_api_key(home, provider, key, store_mode) -> Result<()>` — validates non-empty, persists via `set_provider_auth`, updates `preferred_auth_modes`, saves
- `remove_credentials(home, provider, store_mode) -> Result<bool>` — delegates to `logout_provider()`

Async (return `AuthAttemptHandle` + deliver `AuthAttemptResult` via channel):
- `start_chatgpt_browser_login(...)` — wraps `run_login_server()` + `ShutdownHandle` cancellation
- `start_chatgpt_device_code_login(...)` — wraps the device-code polling loop + `Notify`-based cancellation
- `start_anthropic_oauth(...)` — opens browser, returns code-entry state; exchange is a separate call
- `exchange_anthropic_oauth_code(...)` — async exchange, delivers result via channel

#### Cancellation contract

- Dropping `AuthAttemptHandle` signals cancellation via the `CancellationToken`
- The `AuthFlowView` holds exactly one `Option<AuthAttemptHandle>` at a time
- On Esc, the view drops the handle (cancels the task) and transitions based on `AuthLaunchContext`
- On receiving `AuthAttemptResult`, the view checks `result.id == held_handle.id` before applying. If the IDs don't match (stale completion), the result is silently discarded
- This mirrors the protections in `onboarding/auth.rs` (`device_code_attempt_matches`, `ShutdownHandle` drop) and `onboarding/auth/headless_chatgpt_login.rs`

#### Persistence side effects (shared by all operations)

After any successful credential acquisition:
```rust
fn apply_auth_success(home, provider, auth, store_mode, auth_manager) {
    // 1. set_provider_auth (auto-preserves alternate on method change)
    // 2. Update preferred_auth_modes
    // 3. save_auth_v2
    // 4. auth_manager.reload()
}
```

Refactor `codex-rs/tui/src/onboarding/auth.rs` to call these extracted helpers so onboarding and popup flows share one implementation for persistence and async login work.

### 2. Add a dedicated popup auth view with explicit stack policy

Create a new bottom-pane view:

- `codex-rs/tui/src/bottom_pane/auth_flow_view.rs`

This view implements `BottomPaneView` and owns a single popup auth state machine:

- `ApiKeyEntry { provider, masked_text: TextArea, prefilled: bool }`
- `OpenAiBrowserPending { handle: AuthAttemptHandle }`
- `OpenAiDeviceCodePending { handle: AuthAttemptHandle, device_code: DeviceCode }`
- `AnthropicOAuthCodeEntry { auth_url: String, code_text: TextArea }`
- `AnthropicOAuthExchanging { handle: AuthAttemptHandle }`
- `Working { message: String }`
- `InlineError { message: String }`

Do **not** try to cram editable or async auth state into `SelectionView`. Use a dedicated view, similar to `CustomPromptView` and `FeedbackNoteView`, and reuse the existing `TextArea` infrastructure.

### 3. Introduce an explicit auth launch context with stack policy

Add an internal enum:

```rust
/// Determines success/cancel behavior and popup-stack strategy.
pub(crate) enum AuthLaunchContext {
    /// From `/auth` → provider management sub-popup.
    /// Stack: push AuthFlowView above the provider-management SelectionView.
    /// Esc: pop AuthFlowView, reveal provider-management popup beneath.
    /// Success: pop AuthFlowView, re-open provider-management popup with refreshed state.
    ManageProvider { provider: ProviderName },

    /// From `/model` → cross-provider auth selection.
    /// Stack: replace the auth-selection SelectionView with AuthFlowView (not push).
    /// Esc: dismiss AuthFlowView, do NOT reveal the auth-selection popup. Cancel model switch entirely.
    /// Success: dismiss AuthFlowView, apply model+effort via AppEvent.
    ModelSwitch {
        provider: ProviderName,
        model: String,
        effort: Option<ReasoningEffortConfig>,
    },
}
```

#### BottomPane integration

The key behavioral difference between the two contexts:

| | ManageProvider | ModelSwitch |
|---|---|---|
| **Open** | `bottom_pane.show_view()` (push) | `bottom_pane.view_stack.clear()` then `show_view()` (replace) |
| **Esc** | `is_complete() = true` → normal pop reveals provider popup | `is_complete() = true` → pop to empty (composer) |
| **Success** | Pop + re-open provider management with fresh data | Pop + send `UpdateModel` / `UpdateReasoningEffort` / `PersistModelSelection` |

For `ModelSwitch`, the view must call `view_stack.clear()` before pushing itself (via a new `bottom_pane.replace_all_views(view)` method), so that Esc doesn't reveal the underlying auth-selection or model-selection popups. This requires adding one method to `codex-rs/tui/src/bottom_pane/mod.rs`:

```rust
/// Replace all active views with the given view.
/// Used when the new view should not stack above existing popups.
pub(crate) fn replace_all_views(&mut self, view: Box<dyn BottomPaneView>) {
    self.view_stack.clear();
    self.push_view(view);
}
```

### 4. Error recovery and edge cases

#### Storage/keyring save failure

If `save_auth_v2()` or keyring-backed storage fails after the popup has advanced to a working/success state:

- Transition to `InlineError { message }` state
- Show the error inline in the popup with a "Press Esc to go back" hint
- Do NOT leave the UI in a half-mutated state — if the save failed, the `AuthManager` was not reloaded and credentials were not changed
- Log the error via `tracing::error!`

#### Late async completion after Esc

Handled by the attempt-scoped lifecycle (section 1): the `AuthAttemptHandle` is dropped on Esc, which cancels the underlying task via `CancellationToken`. If the task completes between the drop and the cancel propagation, the `AuthAttemptResult` is discarded because its `AuthAttemptId` no longer matches any held handle.

#### Inline save with both stored credentials and env var

After an inline save when both stored credentials and env var exist for the same provider:

- The provider-management popup shows all three sources: active stored, alternate stored (if present), and env var
- The newly saved credential becomes the active stored credential
- The env var continues to be detected and displayed
- The `preferred_auth_modes` entry determines which is used for requests

#### `/model` auth success + `PersistModelSelection` failure

The `PersistModelSelection` event only writes to config TOML — it does not affect the in-memory model. If it fails:

- The model switch is already applied in memory (via `UpdateModel` + `UpdateReasoningEffort`)
- The persistence failure is logged but does not roll back the in-memory state
- On next startup, the model selection falls back to the config default
- This matches existing `/model` behavior — the same failure mode exists today

#### Background model refresh finishing after popup closes

The refresh is fire-and-forget. If it completes after the popup closes or while another popup is active, the `ModelsManager` cache is simply updated in place. No UI is directly affected — the model picker will reflect the updated catalog next time it opens.

## UX Flows

### `/auth` provider management

The provider-management popup should expose fully functional rows.

For **OpenAI**:

- Active stored credential
- Stored alternate credential
- Env-var credential when relevant
- `Enter new API Key`
- `Sign in with ChatGPT`
- `Sign in with Device Code`
- `Remove credentials`

For **Anthropic**:

- Active stored credential
- Stored alternate credential
- Env-var credential when relevant
- `Enter new API Key`
- `Sign in with Claude (OAuth)`
- `Remove credentials`

Selecting any inline auth-acquisition row should open `AuthFlowView` instead of printing a history-cell message.

### `/model` cross-provider switching

Reuse the same inline auth flows for `/model`, but omit destructive `Remove credentials`.

Rules:

- If env-var auth is the only source, keep the existing fast path and apply the model switch immediately
- If the user explicitly selects a stored or inline auth-acquisition row, stay inside the popup flow until success or cancel
- On success, apply the selected model and effort immediately
- On Esc, cancel the model switch entirely — dismiss ALL stacked popups, return to composer

## Detailed Behavior

### API key entry

The popup API key entry must be **masked** while rendered, but the real value stays in memory.

Use provider-specific behavior:

- OpenAI prefill: `OPENAI_API_KEY` when present
- Anthropic prefill: `ANTHROPIC_API_KEY` when present
- Enter saves
- Esc goes back (ManageProvider) or cancels model switch entirely (ModelSwitch)
- Validation stays aligned with current onboarding behavior: reject empty values, do not invent stricter format validation unless the onboarding flow is updated first

On success:

- Persist provider auth via `apply_auth_success()`
- Return to the proper launch context (see stack policy in section 3)

### OpenAI OAuth

The popup must expose **two explicit OpenAI OAuth actions**:

- `Sign in with ChatGPT`
- `Sign in with Device Code`

Do not collapse both into a single generic OAuth row.

`Sign in with ChatGPT`:

- Calls `auth_flow::start_chatgpt_browser_login()`, receives `AuthAttemptHandle`
- Transitions to `OpenAiBrowserPending` state, showing "Waiting for browser sign-in..."
- Supports Esc cancellation (drops the handle, cancels the task)

`Sign in with Device Code`:

- Calls `auth_flow::start_chatgpt_device_code_login()`, receives `AuthAttemptHandle` + `DeviceCode`
- Transitions to `OpenAiDeviceCodePending`, showing the device code and URL
- Supports Esc cancellation (drops the handle, cancels the polling loop)
- Mirrors existing onboarding behavior and messaging

On success:

- Store ChatGPT auth as the active OpenAI method via `apply_auth_success()`
- Return to launch context

### Anthropic OAuth

`Sign in with Claude (OAuth)` should:

- Open the authorization URL in the browser
- Transition to `AnthropicOAuthCodeEntry` state, rendering the code-entry `TextArea` inline
- Accept both typing and paste
- On Enter, transition to `AnthropicOAuthExchanging`, call `auth_flow::exchange_anthropic_oauth_code()`
- Show inline working and inline error states
- On exchange failure, transition to `InlineError` with "Press Esc to try again"

On success:

- Store `AnthropicOAuth` as active via `apply_auth_success()`
- Return to launch context

### Removal

Keep the current phase-1 removal behavior:

- `logout_provider()`
- `auth_manager.reload()`

Do not regress to merge-based delete logic.

### Post-auth UI refresh

After any successful inline auth mutation:

- Refresh provider-management popup contents (when returning to it via `ManageProvider` context)
- Update model header/status if the auth change was part of a `/model` switch
- Trigger a best-effort local model catalog rebuild via `ModelsManager::list_models(RefreshStrategy::OnlineIfUncached)` in a background task. **Note:** this only produces a remote refresh for ChatGPT-backed OpenAI auth; for API-key and Anthropic auth modes, `OnlineIfUncached` effectively returns cached/bundled data. This is acceptable — the goal is to warm the local picker state, not to perform provider-specific remote discovery.
- Do not block the popup completion on catalog refresh

If the auth flow was launched from `/model`, apply the selected model immediately after `AuthManager::reload()`, not after refresh completes.

## File Changes

### New files

- `codex-rs/tui/src/auth_flow.rs` — shared auth operations with attempt-scoped lifecycle
- `codex-rs/tui/src/auth_flow_tests.rs` — unit tests for masking, save, cancel, stale-attempt handling
- `codex-rs/tui/src/bottom_pane/auth_flow_view.rs` — dedicated popup view with state machine
- `codex-rs/tui/src/bottom_pane/auth_flow_view_tests.rs` — unit tests for view state transitions

### Existing files to update

- `codex-rs/tui/src/lib.rs` — register `auth_flow` module
- `codex-rs/tui/src/onboarding/auth.rs` — refactor to call `auth_flow` helpers
- `codex-rs/tui/src/bottom_pane/mod.rs` — add `replace_all_views()`, register `auth_flow_view` module
- `codex-rs/tui/src/chatwidget/auth_popup.rs` — replace placeholder closures with `AuthFlowView` launches
- `codex-rs/tui/src/chatwidget.rs` — wire `AppEvent` variants for auth flow results
- `codex-rs/tui/src/chatwidget/tests.rs` — popup integration snapshots and behavior tests

## Tests

### Unit tests adjacent to new modules

Add in `codex-rs/tui/src/auth_flow_tests.rs`:

- `save_api_key_persists_and_updates_preferred_mode`
- `save_api_key_rejects_empty`
- `save_api_key_preserves_alternate_on_method_change`
- `remove_credentials_delegates_to_logout_provider`
- `apply_auth_success_reloads_auth_manager`
- `auth_attempt_id_mismatch_discards_result`

Add in `codex-rs/tui/src/bottom_pane/auth_flow_view_tests.rs`:

- `api_key_entry_masks_input`
- `api_key_entry_esc_returns_manage_provider`
- `api_key_entry_esc_cancels_model_switch`
- `browser_pending_esc_drops_handle`
- `device_code_pending_renders_code_and_url`
- `oauth_code_entry_enter_transitions_to_exchanging`
- `inline_error_esc_returns_to_previous_state`

### Popup integration snapshots in `chatwidget/tests.rs`

- OpenAI API key entry view
- Anthropic API key entry view
- OpenAI browser-login pending view
- OpenAI device-code view
- Anthropic OAuth code-entry view
- Provider-management popup after successful inline auth change
- Env-var-only `/model` fast path

### Behavior tests in `chatwidget/tests.rs`

- Successful OpenAI API key save from `/auth`
- Successful Anthropic API key save from `/model` with target model applied
- Successful OpenAI browser-login completion from `/model`
- Successful OpenAI device-code completion from `/auth`
- Successful Anthropic OAuth exchange from `/model`
- Esc from `ManageProvider` returning to the provider-management popup
- Esc from `ModelSwitch` leaving the current model unchanged
- Inline validation errors keeping the auth view open
- `logout_provider()` preserving env-var fallback after removal

### Onboarding regressions

Add onboarding regressions in the onboarding auth tests to confirm the extraction into `auth_flow.rs` does not change onboarding behavior.

## Validation

Run:

```bash
cd /Users/no9labs/Developer/Recursive/codex/codex-rs
cargo test -p orbit-code-tui
cargo insta pending-snapshots
just fmt
just fix -p orbit-code-tui
```

If implementation touches shared auth/runtime code paths, also run:

```bash
cargo test -p orbit-code-core
cargo test -p orbit-code-tui-app-server
```

## Acceptance Criteria

- `/auth` no longer prints placeholder guidance for API-key or OAuth rows
- `/model` can complete a cross-provider switch even when the required auth must be acquired inline first
- OpenAI supports both browser login and device code inline from popup
- Anthropic supports API key and OAuth inline from popup
- Successful inline auth from `/auth` returns to provider management with updated state
- Successful inline auth from `/model` applies the selected model immediately
- Esc from `/auth` auth flow returns to provider-management popup (not composer)
- Esc from `/model` auth flow cancels the model switch entirely (returns to composer)
- Late async completions after Esc are silently discarded (no stale mutation)
- Storage save failures show inline error without leaving half-mutated state
- No restart or CLI login is required for popup-based auth acquisition in standalone `tui`

## Notes

- Keep this as a strict follow-on to the phase-1 plan. Do not re-spec the already-implemented pieces.
- If this grows beyond a single reasonable implementation, split the work into: extraction first, popup view second, integration/tests third. Keep all three inside this tracked plan.
- The `tui_app_server` mirror is explicitly deferred. Track as a separate follow-up plan once this flow stabilizes.
