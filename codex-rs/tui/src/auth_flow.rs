//! Shared auth operations for popup and onboarding flows.
//!
//! This module owns provider-scoped auth operations with an attempt-scoped
//! lifecycle. Sync operations (API key save, credential removal) return
//! `Result` directly. Async operations (browser login, device code, OAuth
//! exchange) return an [`AuthAttemptHandle`] and deliver results via a
//! tokio channel.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use orbit_code_app_server_protocol::AuthMode;
use orbit_code_core::AuthManager;
use orbit_code_core::auth::AuthCredentialsStoreMode;
use orbit_code_core::auth::AuthDotJsonV2;
use orbit_code_core::auth::ProviderAuth;
use orbit_code_core::auth::ProviderName;
use orbit_code_core::auth::load_auth_dot_json_v2;
use orbit_code_core::auth::save_auth_v2;
use orbit_code_login::AnthropicAuthMode;
use orbit_code_login::DeviceCode;
use orbit_code_login::ServerOptions;
use orbit_code_login::run_login_server;
use tokio_util::sync::CancellationToken;

use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;

#[cfg(test)]
#[path = "auth_flow_tests.rs"]
mod tests;

// ── Provider helpers ──────────────────────────────────────────────

/// Display name for a provider.
pub(crate) fn provider_display_name(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::OpenAI => "OpenAI",
        ProviderName::Anthropic => "Anthropic",
    }
}

// ── Attempt-scoped lifecycle types ────────────────────────────────

/// Monotonic counter for unique attempt IDs.
static NEXT_ATTEMPT_ID: AtomicU64 = AtomicU64::new(1);

/// Uniquely identifies one auth attempt. Used to guard against stale
/// completions arriving after the user has cancelled or started a new
/// attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AuthAttemptId(u64);

impl AuthAttemptId {
    fn next() -> Self {
        Self(NEXT_ATTEMPT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Handle for an in-progress async auth operation.
///
/// Dropping this handle signals cancellation via the internal
/// [`CancellationToken`].
pub(crate) struct AuthAttemptHandle {
    pub id: AuthAttemptId,
    cancel: CancellationToken,
}

impl AuthAttemptHandle {
    fn new() -> (Self, CancellationToken) {
        let id = AuthAttemptId::next();
        let cancel = CancellationToken::new();
        let handle = Self {
            id,
            cancel: cancel.clone(),
        };
        (handle, cancel)
    }
}

impl Drop for AuthAttemptHandle {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

/// Result delivered by an async auth operation.
pub(crate) struct AuthAttemptResult {
    pub id: AuthAttemptId,
    pub outcome: Result<AuthSuccess, AuthError>,
}

/// Successful auth acquisition.
pub(crate) enum AuthSuccess {
    ApiKeySaved,
    OAuthComplete,
}

/// Auth operation failure.
pub(crate) enum AuthError {
    ExchangeFailed(String),
    StorageFailed(String),
}

// ── Sync operations ───────────────────────────────────────────────

/// Validate and persist an API key for the given provider.
///
/// Returns `Ok(())` on success. Rejects empty keys.
pub(crate) fn save_api_key(
    orbit_code_home: &Path,
    provider: ProviderName,
    key: &str,
    store_mode: AuthCredentialsStoreMode,
) -> Result<(), String> {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err("API key cannot be empty.".to_string());
    }

    let mut v2 = load_or_new(orbit_code_home, store_mode)?;

    let auth = match provider {
        ProviderName::OpenAI => ProviderAuth::OpenAiApiKey {
            key: trimmed.to_string(),
        },
        ProviderName::Anthropic => ProviderAuth::AnthropicApiKey {
            key: trimmed.to_string(),
        },
    };

    let mode = auth_mode_for_provider_auth(&auth);
    v2.set_provider_auth(provider, auth);
    v2.preferred_auth_modes.insert(provider, mode);

    save_auth_v2(orbit_code_home, &v2, store_mode)
        .map_err(|e| format!("Failed to save credentials: {e}"))
}

/// Shared persistence side-effect after any successful credential
/// acquisition.
pub(crate) fn apply_auth_success(
    orbit_code_home: &Path,
    provider: ProviderName,
    auth: ProviderAuth,
    store_mode: AuthCredentialsStoreMode,
    auth_manager: &AuthManager,
) -> Result<(), String> {
    let mut v2 = load_or_new(orbit_code_home, store_mode)?;
    let mode = auth_mode_for_provider_auth(&auth);
    v2.set_provider_auth(provider, auth);
    v2.preferred_auth_modes.insert(provider, mode);
    save_auth_v2(orbit_code_home, &v2, store_mode)
        .map_err(|e| format!("Failed to save credentials: {e}"))?;
    auth_manager.reload();
    Ok(())
}

// ── Async operations ──────────────────────────────────────────────

/// Start a browser-based ChatGPT OAuth login.
///
/// Opens the browser and waits for the callback. The result is delivered
/// via `result_tx`.
pub(crate) fn start_chatgpt_browser_login(
    orbit_code_home: &Path,
    store_mode: AuthCredentialsStoreMode,
    auth_manager: Arc<AuthManager>,
    forced_workspace_id: Option<String>,
    result_tx: tokio::sync::mpsc::UnboundedSender<AuthAttemptResult>,
    app_event_tx: AppEventSender,
) -> Result<AuthAttemptHandle, String> {
    let opts = ServerOptions::new(
        orbit_code_home.to_path_buf(),
        orbit_code_core::auth::CLIENT_ID.to_string(),
        forced_workspace_id,
        store_mode,
    );

    // run_login_server opens the browser via ServerOptions::open_browser.
    let server =
        run_login_server(opts).map_err(|e| format!("Failed to start login server: {e}"))?;

    let (handle, cancel_token) = AuthAttemptHandle::new();
    let attempt_id = handle.id;
    let shutdown = server.cancel_handle();

    tokio::spawn(async move {
        tokio::select! {
            result = server.block_until_done() => {
                let outcome = match result {
                    Ok(()) => {
                        auth_manager.reload();
                        Ok(AuthSuccess::OAuthComplete)
                    }
                    Err(e) => Err(AuthError::ExchangeFailed(e.to_string())),
                };
                let _ = result_tx.send(AuthAttemptResult { id: attempt_id, outcome });
                app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
            }
            () = cancel_token.cancelled() => {
                shutdown.shutdown();
            }
        }
    });

    Ok(handle)
}

/// Start a device-code ChatGPT login flow.
///
/// Returns the handle and the device code for display. The result is
/// delivered via `result_tx`.
pub(crate) fn start_chatgpt_device_code_login(
    orbit_code_home: &Path,
    store_mode: AuthCredentialsStoreMode,
    auth_manager: Arc<AuthManager>,
    forced_workspace_id: Option<String>,
    result_tx: tokio::sync::mpsc::UnboundedSender<AuthAttemptResult>,
    app_event_tx: AppEventSender,
) -> Result<
    (
        AuthAttemptHandle,
        tokio::sync::mpsc::UnboundedReceiver<DeviceCode>,
    ),
    String,
> {
    let opts = ServerOptions::new(
        orbit_code_home.to_path_buf(),
        orbit_code_core::auth::CLIENT_ID.to_string(),
        forced_workspace_id,
        store_mode,
    );

    let (handle, cancel_token) = AuthAttemptHandle::new();
    let attempt_id = handle.id;
    let (dc_tx, dc_rx) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let device_code = match orbit_code_login::request_device_code(&opts).await {
            Ok(dc) => dc,
            Err(e) => {
                // Mirror the onboarding fallback: if device-code endpoint
                // returns NotFound, fall back to browser-based login.
                if e.kind() == std::io::ErrorKind::NotFound {
                    tracing::info!("device code endpoint not found, falling back to browser login");
                    match run_login_server(opts) {
                        Ok(server) => {
                            let shutdown = server.cancel_handle();
                            tokio::select! {
                                result = server.block_until_done() => {
                                    let outcome = match result {
                                        Ok(()) => {
                                            auth_manager.reload();
                                            Ok(AuthSuccess::OAuthComplete)
                                        }
                                        Err(e) => Err(AuthError::ExchangeFailed(e.to_string())),
                                    };
                                    let _ = result_tx.send(AuthAttemptResult { id: attempt_id, outcome });
                                    app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
                                }
                                () = cancel_token.cancelled() => {
                                    shutdown.shutdown();
                                }
                            }
                        }
                        Err(e) => {
                            let _ = result_tx.send(AuthAttemptResult {
                                id: attempt_id,
                                outcome: Err(AuthError::ExchangeFailed(format!(
                                    "Failed to start login server: {e}"
                                ))),
                            });
                            app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
                        }
                    }
                } else {
                    let _ = result_tx.send(AuthAttemptResult {
                        id: attempt_id,
                        outcome: Err(AuthError::ExchangeFailed(format!(
                            "Failed to request device code: {e}"
                        ))),
                    });
                    app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
                }
                return;
            }
        };

        // Send the device code to the view for rendering.
        let _ = dc_tx.send(device_code.clone());
        app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);

        tokio::select! {
            result = orbit_code_login::complete_device_code_login(opts, device_code) => {
                let outcome = match result {
                    Ok(()) => {
                        auth_manager.reload();
                        Ok(AuthSuccess::OAuthComplete)
                    }
                    Err(e) => Err(AuthError::ExchangeFailed(e.to_string())),
                };
                let _ = result_tx.send(AuthAttemptResult { id: attempt_id, outcome });
                app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
            }
            () = cancel_token.cancelled() => {}
        }
    });

    Ok((handle, dc_rx))
}

/// Start an Anthropic OAuth flow.
///
/// Opens the browser with the authorization URL and returns the verifier
/// for the code-exchange step. The caller collects the pasted code and
/// calls [`exchange_anthropic_oauth_code`].
pub(crate) fn start_anthropic_oauth() -> Result<(String, String), String> {
    let (auth_url, verifier) =
        orbit_code_login::anthropic_authorize_url(AnthropicAuthMode::MaxSubscription)
            .map_err(|e| format!("Failed to generate OAuth URL: {e}"))?;

    let _ = webbrowser::open(&auth_url);
    Ok((auth_url, verifier))
}

/// Exchange an Anthropic OAuth authorization code for tokens.
///
/// The result is delivered via `result_tx`.
pub(crate) fn exchange_anthropic_oauth_code(
    code_with_state: String,
    verifier: String,
    orbit_code_home: &Path,
    store_mode: AuthCredentialsStoreMode,
    auth_manager: Arc<AuthManager>,
    result_tx: tokio::sync::mpsc::UnboundedSender<AuthAttemptResult>,
    app_event_tx: AppEventSender,
) -> AuthAttemptHandle {
    let (handle, cancel_token) = AuthAttemptHandle::new();
    let attempt_id = handle.id;
    let home = orbit_code_home.to_path_buf();

    tokio::spawn(async move {
        tokio::select! {
            result = orbit_code_login::anthropic_exchange_code(&code_with_state, &verifier) => {
                let outcome = match result {
                    Ok(tokens) => {
                        let now = chrono::Utc::now().timestamp();
                        let expires_at = now.saturating_add(
                            i64::try_from(tokens.expires_in).unwrap_or(3600),
                        );
                        let auth = ProviderAuth::AnthropicOAuth {
                            access_token: tokens.access_token,
                            refresh_token: tokens.refresh_token,
                            expires_at,
                        };
                        match apply_auth_success(
                            &home,
                            ProviderName::Anthropic,
                            auth,
                            store_mode,
                            &auth_manager,
                        ) {
                            Ok(()) => Ok(AuthSuccess::OAuthComplete),
                            Err(e) => Err(AuthError::StorageFailed(e)),
                        }
                    }
                    Err(e) => Err(AuthError::ExchangeFailed(e.to_string())),
                };
                let _ = result_tx.send(AuthAttemptResult { id: attempt_id, outcome });
                app_event_tx.send(AppEvent::AuthFlowAsyncUpdate);
            }
            () = cancel_token.cancelled() => {}
        }
    });

    handle
}

// ── Internal helpers ──────────────────────────────────────────────

fn load_or_new(
    orbit_code_home: &Path,
    store_mode: AuthCredentialsStoreMode,
) -> Result<AuthDotJsonV2, String> {
    match load_auth_dot_json_v2(orbit_code_home, store_mode) {
        Ok(Some(v2)) => Ok(v2),
        Ok(None) => Ok(AuthDotJsonV2::new()),
        Err(e) => Err(format!("Failed to load auth storage: {e}")),
    }
}

fn auth_mode_for_provider_auth(auth: &ProviderAuth) -> AuthMode {
    match auth {
        ProviderAuth::OpenAiApiKey { .. } => AuthMode::ApiKey,
        ProviderAuth::Chatgpt { .. } => AuthMode::Chatgpt,
        ProviderAuth::ChatgptAuthTokens { .. } => AuthMode::ChatgptAuthTokens,
        ProviderAuth::AnthropicApiKey { .. } => AuthMode::AnthropicApiKey,
        ProviderAuth::AnthropicOAuth { .. } => AuthMode::AnthropicOAuth,
    }
}
