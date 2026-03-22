//! Auth method selection popup for mid-session provider switching.
//!
//! Provides helpers for the `/auth` command and the auth step in the
//! model-switch flow. Uses the `SelectionView` popup pattern.

use orbit_code_app_server_protocol::AuthMode;
use orbit_code_core::auth::AuthDotJsonV2;
use orbit_code_core::auth::ProviderAuth;
use orbit_code_core::auth::ProviderName;
use orbit_code_core::auth::load_auth_dot_json_v2;
use orbit_code_core::auth::save_auth_v2;

use orbit_code_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;

use super::ChatWidget;

// ── Helpers ────────────────────────────────────────────────────────

/// Determine the provider for a given model slug.
pub(crate) fn provider_for_model(slug: &str) -> ProviderName {
    if slug.starts_with("claude-") || slug.starts_with("claude3") {
        ProviderName::Anthropic
    } else {
        ProviderName::OpenAI
    }
}

/// Display name for a provider.
pub(crate) fn provider_display_name(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::OpenAI => "OpenAI",
        ProviderName::Anthropic => "Anthropic",
    }
}

/// Environment variable name for a provider's API key.
fn env_var_for_provider(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::Anthropic => "ANTHROPIC_API_KEY",
        ProviderName::OpenAI => "OPENAI_API_KEY",
    }
}

/// CLI command to add credentials for a provider.
fn login_command_for_provider(provider: ProviderName) -> &'static str {
    match provider {
        ProviderName::Anthropic => "just codex login --provider anthropic",
        ProviderName::OpenAI => "printenv OPENAI_API_KEY | just codex login --with-api-key",
    }
}

/// Determine the AuthMode that corresponds to a given ProviderAuth variant.
fn auth_mode_for_provider_auth(auth: &ProviderAuth) -> AuthMode {
    match auth {
        ProviderAuth::OpenAiApiKey { .. } => AuthMode::ApiKey,
        ProviderAuth::Chatgpt { .. } => AuthMode::Chatgpt,
        ProviderAuth::ChatgptAuthTokens { .. } => AuthMode::ChatgptAuthTokens,
        ProviderAuth::AnthropicApiKey { .. } => AuthMode::AnthropicApiKey,
        ProviderAuth::AnthropicOAuth { .. } => AuthMode::AnthropicOAuth,
    }
}

/// Mask a credential value for display, showing first 7 and last 3 chars.
fn mask_credential(value: &str) -> String {
    if value.len() <= 10 {
        return "*".repeat(value.len());
    }
    let prefix = &value[..7];
    let suffix = &value[value.len() - 3..];
    format!("{prefix}*******{suffix}")
}

/// Human-readable summary of a credential.
fn credential_summary(auth: &ProviderAuth) -> String {
    match auth {
        ProviderAuth::OpenAiApiKey { key } => format!("API Key: {}", mask_credential(key)),
        ProviderAuth::Chatgpt { .. } => "OAuth (ChatGPT)".to_string(),
        ProviderAuth::ChatgptAuthTokens { .. } => "OAuth (external)".to_string(),
        ProviderAuth::AnthropicApiKey { key } => format!("API Key: {}", mask_credential(key)),
        ProviderAuth::AnthropicOAuth { .. } => "OAuth (Anthropic)".to_string(),
    }
}

/// Build the status description for a provider in the `/auth` overview.
///
/// `has_effective_auth` indicates whether `AuthManager` resolved effective
/// auth for this provider (from any source: stored, alternate, or env var).
/// `env_present` indicates whether the provider's env var is set.
fn provider_status_description(
    provider: ProviderName,
    active: Option<&ProviderAuth>,
    alternate: Option<&ProviderAuth>,
    has_effective_auth: bool,
    env_present: bool,
) -> String {
    let env_var_name = env_var_for_provider(provider);

    let mut parts = Vec::new();

    if let Some(a) = active {
        parts.push(format!("{} (active)", credential_summary(a)));
    }
    if let Some(alt) = alternate {
        parts.push(format!("{} (stored)", credential_summary(alt)));
    }
    if env_present {
        if active.is_none() && alternate.is_none() {
            if has_effective_auth {
                parts.push(format!("API Key (active via {env_var_name})"));
            } else {
                parts.push(format!("{env_var_name} (set but not effective)"));
            }
        } else {
            parts.push(format!("{env_var_name} (env)"));
        }
    }

    if parts.is_empty() {
        "not configured".to_string()
    } else {
        parts.join("  |  ")
    }
}

// ── /auth command ──────────────────────────────────────────────────

/// Handle the `/auth` slash command — shows provider status and management popup.
pub(super) fn on_slash_auth(widget: &mut ChatWidget) {
    let v2 = match load_auth_dot_json_v2(
        &widget.config.orbit_code_home,
        widget.config.cli_auth_credentials_store_mode,
    ) {
        Ok(Some(v2)) => v2,
        Ok(None) => AuthDotJsonV2::new(),
        Err(e) => {
            widget.add_info_message(
                format!("Failed to load auth storage: {e}"),
                /*hint*/ None,
            );
            return;
        }
    };

    let mut items = Vec::new();

    for provider in [ProviderName::OpenAI, ProviderName::Anthropic] {
        let name = provider_display_name(provider);
        let active = v2.provider_auth(provider);
        let alternate = v2.alternate_credentials.get(&provider);

        // Use AuthManager as the source of truth for effective auth.
        let has_effective_auth = widget
            .auth_manager
            .auth_cached_for_provider(provider)
            .is_some();
        let env_present = std::env::var(env_var_for_provider(provider))
            .ok()
            .is_some_and(|v| !v.is_empty());

        let description = provider_status_description(
            provider,
            active,
            alternate,
            has_effective_auth,
            env_present,
        );

        let app_tx = widget.app_event_tx.clone();

        items.push(SelectionItem {
            name: format!("Manage {name}"),
            description: Some(description),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::ManageAuthProvider { provider });
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    widget.bottom_pane.show_selection_view(SelectionViewParams {
        title: Some("Authentication Status".to_string()),
        subtitle: Some("Select a provider to manage credentials.".to_string()),
        footer_hint: Some(standard_popup_hint_line()),
        items,
        ..Default::default()
    });
    widget.request_redraw();
}

// ── Provider management sub-popup ──────────────────────────────────

/// Sub-popup for managing a single provider from `/auth`.
pub(super) fn open_auth_provider_management(widget: &mut ChatWidget, provider: ProviderName) {
    let v2 = match load_auth_dot_json_v2(
        &widget.config.orbit_code_home,
        widget.config.cli_auth_credentials_store_mode,
    ) {
        Ok(Some(v2)) => v2,
        Ok(None) => AuthDotJsonV2::new(),
        Err(e) => {
            widget.add_info_message(
                format!("Failed to load auth storage: {e}"),
                /*hint*/ None,
            );
            return;
        }
    };

    let name = provider_display_name(provider);
    let active = v2.provider_auth(provider).cloned();
    let alternate = v2.alternate_credentials.get(&provider).cloned();

    let mut items = Vec::new();

    // Active credential — selecting is a no-op (keeps current).
    if let Some(ref cred) = active {
        let summary = credential_summary(cred);
        items.push(SelectionItem {
            name: format!("{summary} (active)"),
            description: Some("Currently in use".to_string()),
            actions: vec![Box::new(move |_tx| {
                tracing::info!("auth management: keeping active credential for {provider}");
            })],
            dismiss_on_select: true,
            is_current: true,
            ..Default::default()
        });
    }

    // Alternate credential — swap it in.
    if let Some(ref alt) = alternate {
        let summary = credential_summary(alt);
        let orbit_code_home = widget.config.orbit_code_home.clone();
        let store_mode = widget.config.cli_auth_credentials_store_mode;
        let auth_manager = widget.auth_manager.clone();
        let alt_mode = auth_mode_for_provider_auth(alt);
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: format!("{summary} (stored)"),
            description: Some("Switch to this credential".to_string()),
            actions: vec![Box::new(move |_tx| {
                let v2_result = load_auth_dot_json_v2(&orbit_code_home, store_mode);
                match v2_result {
                    Ok(Some(mut v2)) => {
                        v2.restore_alternate_credential(provider);
                        v2.preferred_auth_modes.insert(provider, alt_mode);
                        if let Err(e) = save_auth_v2(&orbit_code_home, &v2, store_mode) {
                            tracing::error!("failed to save auth after swap: {e}");
                            app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                                crate::history_cell::new_info_event(
                                    format!("Failed to save credential swap: {e}"),
                                    None,
                                ),
                            )));
                            return;
                        }
                        auth_manager.reload();
                        tracing::info!(
                            "auth management: swapped to alternate credential for {provider}"
                        );
                    }
                    Ok(None) => {
                        tracing::error!("auth storage empty during swap for {provider}");
                        app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                            crate::history_cell::new_info_event(
                                "Credential swap failed: auth storage is empty.".to_string(),
                                None,
                            ),
                        )));
                    }
                    Err(e) => {
                        tracing::error!("failed to load auth for swap: {e}");
                        app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                            crate::history_cell::new_info_event(
                                format!("Failed to load auth storage: {e}"),
                                None,
                            ),
                        )));
                    }
                }
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    // "Enter new API Key" — Phase 2 placeholder with CLI instructions.
    {
        let cmd = login_command_for_provider(provider);
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: "Enter new API Key".to_string(),
            description: Some(format!("Run: {cmd}")),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    crate::history_cell::new_info_event(
                        format!("To add a new API key, run:\n  {cmd}"),
                        None,
                    ),
                )));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    // "OAuth Login" — Phase 2 placeholder.
    {
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: "OAuth Login".to_string(),
            description: Some(format!("Sign in with your {name} account")),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    crate::history_cell::new_info_event(
                        "Mid-session OAuth switching is not yet available. \
                         Use /logout to clear current credentials, then restart \
                         Orbit Code to reach the onboarding OAuth flow."
                            .to_string(),
                        None,
                    ),
                )));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    // "Remove credentials" — routes through confirmation popup.
    if active.is_some() || alternate.is_some() {
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: "Remove credentials".to_string(),
            description: Some(format!("Delete all stored {name} credentials")),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::ConfirmRemoveProviderAuth { provider });
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    widget.bottom_pane.show_selection_view(SelectionViewParams {
        title: Some(format!("Manage {name} Authentication")),
        footer_hint: Some(standard_popup_hint_line()),
        items,
        ..Default::default()
    });
    widget.request_redraw();
}

// ── Model-switch auth popup ────────────────────────────────────────

/// Open the auth method picker for a target provider during model switch.
///
/// Uses `AuthManager` as the source of truth for effective auth. If the
/// only auth source is an env var (no stored credentials), the popup is
/// skipped and the model switch is applied immediately.
pub(crate) fn open_auth_popup(
    widget: &mut ChatWidget,
    target_provider: ProviderName,
    selected_model: String,
    selected_effort: Option<ReasoningEffortConfig>,
    _is_standalone: bool,
) {
    let v2 = match load_auth_dot_json_v2(
        &widget.config.orbit_code_home,
        widget.config.cli_auth_credentials_store_mode,
    ) {
        Ok(Some(v2)) => v2,
        Ok(None) => AuthDotJsonV2::new(),
        Err(e) => {
            widget.add_info_message(
                format!("Failed to load auth storage: {e}"),
                /*hint*/ None,
            );
            return;
        }
    };

    let name = provider_display_name(target_provider);
    let active = v2.provider_auth(target_provider).cloned();
    let alternate = v2.alternate_credentials.get(&target_provider).cloned();

    // Use AuthManager as source of truth for whether effective auth exists.
    let has_effective_auth = widget
        .auth_manager
        .auth_cached_for_provider(target_provider)
        .is_some();

    // Fast path: effective auth exists but no stored credentials — env var is the source.
    if active.is_none() && alternate.is_none() && has_effective_auth {
        widget.apply_model_and_effort(selected_model, selected_effort);
        return;
    }

    // No auth at all — show info message with instructions.
    if !has_effective_auth && active.is_none() && alternate.is_none() {
        let cmd = login_command_for_provider(target_provider);
        widget.add_info_message(
            format!("No credentials found for {name}. Run: {cmd}"),
            /*hint*/ None,
        );
        return;
    }

    let mut items = Vec::new();

    // Active stored credential.
    if let Some(ref cred) = active {
        let summary = credential_summary(cred);
        let model_clone = selected_model.clone();
        let effort_clone = selected_effort;
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: format!("{summary} (active)"),
            description: Some("Use current credential".to_string()),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::UpdateModel(model_clone.clone()));
                app_tx.send(AppEvent::UpdateReasoningEffort(effort_clone));
                app_tx.send(AppEvent::PersistModelSelection {
                    model: model_clone.clone(),
                    effort: effort_clone,
                });
            })],
            dismiss_on_select: true,
            is_current: true,
            ..Default::default()
        });
    }

    // Alternate stored credential.
    if let Some(ref alt) = alternate {
        let summary = credential_summary(alt);
        let orbit_code_home = widget.config.orbit_code_home.clone();
        let store_mode = widget.config.cli_auth_credentials_store_mode;
        let model_clone = selected_model.clone();
        let effort_clone = selected_effort;
        let app_tx = widget.app_event_tx.clone();
        let auth_manager = widget.auth_manager.clone();
        let alt_mode = auth_mode_for_provider_auth(alt);
        items.push(SelectionItem {
            name: format!("{summary} (stored)"),
            description: Some("Switch to stored credential".to_string()),
            actions: vec![Box::new(move |_tx| {
                let v2_result = load_auth_dot_json_v2(&orbit_code_home, store_mode);
                match v2_result {
                    Ok(Some(mut v2)) => {
                        v2.restore_alternate_credential(target_provider);
                        v2.preferred_auth_modes.insert(target_provider, alt_mode);
                        if let Err(e) = save_auth_v2(&orbit_code_home, &v2, store_mode) {
                            tracing::error!("failed to save auth after swap: {e}");
                            app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                                crate::history_cell::new_info_event(
                                    format!("Failed to save credential swap: {e}"),
                                    None,
                                ),
                            )));
                            return;
                        }
                        auth_manager.reload();
                        app_tx.send(AppEvent::UpdateModel(model_clone.clone()));
                        app_tx.send(AppEvent::UpdateReasoningEffort(effort_clone));
                        app_tx.send(AppEvent::PersistModelSelection {
                            model: model_clone.clone(),
                            effort: effort_clone,
                        });
                    }
                    Ok(None) | Err(_) => {
                        tracing::error!("failed to load auth for swap");
                        app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                            crate::history_cell::new_info_event(
                                "Credential swap failed: could not load auth storage.".to_string(),
                                None,
                            ),
                        )));
                    }
                }
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    // Env var credential — only shown when stored credentials also exist and
    // AuthManager confirms the env var provides effective auth.
    if has_effective_auth && (active.is_some() || alternate.is_some()) {
        let env_var_name = env_var_for_provider(target_provider);
        let env_set = std::env::var(env_var_name)
            .ok()
            .is_some_and(|v| !v.is_empty());
        if env_set {
            let model_clone = selected_model;
            let effort_clone = selected_effort;
            let app_tx = widget.app_event_tx.clone();
            items.push(SelectionItem {
                name: format!("API Key (via {env_var_name})"),
                description: Some("Use environment variable".to_string()),
                actions: vec![Box::new(move |_tx| {
                    app_tx.send(AppEvent::UpdateModel(model_clone.clone()));
                    app_tx.send(AppEvent::UpdateReasoningEffort(effort_clone));
                    app_tx.send(AppEvent::PersistModelSelection {
                        model: model_clone.clone(),
                        effort: effort_clone,
                    });
                })],
                dismiss_on_select: true,
                ..Default::default()
            });
        }
    }

    // "Enter new API Key" — Phase 2 placeholder.
    {
        let cmd = login_command_for_provider(target_provider);
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: "Enter new API Key".to_string(),
            description: Some(format!("Run: {cmd}")),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    crate::history_cell::new_info_event(
                        format!("To add a new API key, run:\n  {cmd}"),
                        None,
                    ),
                )));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    // "OAuth Login" — Phase 2 placeholder.
    {
        let app_tx = widget.app_event_tx.clone();
        items.push(SelectionItem {
            name: "OAuth Login".to_string(),
            description: Some(format!("Sign in with your {name} account")),
            actions: vec![Box::new(move |_tx| {
                app_tx.send(AppEvent::InsertHistoryCell(Box::new(
                    crate::history_cell::new_info_event(
                        "Mid-session OAuth switching is not yet available. \
                         Use /logout to clear current credentials, then restart \
                         Orbit Code to reach the onboarding OAuth flow."
                            .to_string(),
                        None,
                    ),
                )));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    widget.bottom_pane.show_selection_view(SelectionViewParams {
        title: Some(format!("Select Authentication for {name}")),
        footer_hint: Some(standard_popup_hint_line()),
        items,
        ..Default::default()
    });
    widget.request_redraw();
}
