//! Dedicated popup view for inline auth acquisition.
//!
//! Implements [`BottomPaneView`] with a state machine that handles API key
//! entry, OAuth browser pending, device code display, Anthropic OAuth code
//! entry, and inline error states. Receives async results via a tokio channel
//! and guards against stale completions with [`AuthAttemptId`].

use std::cell::RefCell;
use std::sync::Arc;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use orbit_code_core::AuthManager;
use orbit_code_core::auth::AuthCredentialsStoreMode;
use orbit_code_core::auth::ProviderName;
use orbit_code_login::DeviceCode;
use orbit_code_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::StatefulWidgetRef;
use ratatui::widgets::Widget;

use crate::app_event::AppEvent;
use crate::app_event_sender::AppEventSender;
use crate::auth_flow::AuthAttemptHandle;
use crate::auth_flow::AuthAttemptResult;
use crate::auth_flow::AuthError;
use crate::auth_flow::AuthSuccess;
use crate::auth_flow::provider_display_name;
use crate::render::renderable::Renderable;

use super::CancellationEvent;
use super::bottom_pane_view::BottomPaneView;
use super::textarea::TextArea;
use super::textarea::TextAreaState;

#[cfg(test)]
#[path = "auth_flow_view_tests.rs"]
mod tests;

// ── Launch context ────────────────────────────────────────────────

/// Determines success/cancel behavior and popup-stack strategy.
#[derive(Clone, Debug)]
pub(crate) enum AuthLaunchContext {
    /// From `/auth` → provider management sub-popup.
    /// Esc: pop this view, reveal provider-management popup beneath.
    /// Success: pop this view, re-open provider management with fresh state.
    ManageProvider { provider: ProviderName },

    /// From `/model` → cross-provider auth selection.
    /// Esc: dismiss all popups, cancel model switch entirely.
    /// Success: dismiss all popups, apply model+effort via AppEvent.
    ModelSwitch {
        provider: ProviderName,
        model: String,
        effort: Option<ReasoningEffortConfig>,
    },
}

impl AuthLaunchContext {
    pub(crate) fn provider(&self) -> ProviderName {
        match self {
            Self::ManageProvider { provider } | Self::ModelSwitch { provider, .. } => *provider,
        }
    }
}

// ── View state machine ────────────────────────────────────────────

enum AuthFlowState {
    /// Inline API key entry with masked display.
    ApiKeyEntry { textarea: TextArea },

    /// Waiting for browser-based ChatGPT OAuth to complete.
    OpenAiBrowserPending { _handle: AuthAttemptHandle },

    /// Waiting for device code polling to complete.
    OpenAiDeviceCodePending {
        _handle: AuthAttemptHandle,
        device_code: Option<DeviceCode>,
    },

    /// Anthropic OAuth code entry (user pastes code from browser).
    AnthropicOAuthCodeEntry {
        auth_url: String,
        verifier: String,
        textarea: TextArea,
        textarea_state: RefCell<TextAreaState>,
    },

    /// Exchanging Anthropic OAuth code for tokens.
    AnthropicOAuthExchanging { _handle: AuthAttemptHandle },

    /// Inline error with "Press Esc to go back".
    InlineError { message: String },
}

// ── The view ──────────────────────────────────────────────────────

pub(crate) struct AuthFlowView {
    context: AuthLaunchContext,
    state: AuthFlowState,
    complete: bool,

    // Shared resources for auth operations.
    orbit_code_home: std::path::PathBuf,
    store_mode: AuthCredentialsStoreMode,
    auth_manager: Arc<AuthManager>,
    app_event_tx: AppEventSender,

    // Channel for receiving async auth results.
    result_rx: tokio::sync::mpsc::UnboundedReceiver<AuthAttemptResult>,
    result_tx: tokio::sync::mpsc::UnboundedSender<AuthAttemptResult>,

    // Channel for receiving device codes.
    device_code_rx: Option<tokio::sync::mpsc::UnboundedReceiver<DeviceCode>>,
}

impl AuthFlowView {
    pub(crate) fn new_api_key_entry(
        context: AuthLaunchContext,
        orbit_code_home: std::path::PathBuf,
        store_mode: AuthCredentialsStoreMode,
        auth_manager: Arc<AuthManager>,
        app_event_tx: AppEventSender,
    ) -> Self {
        let provider = context.provider();
        let env_var = match provider {
            ProviderName::OpenAI => "OPENAI_API_KEY",
            ProviderName::Anthropic => "ANTHROPIC_API_KEY",
        };
        let prefill = std::env::var(env_var).ok().filter(|v| !v.is_empty());
        let mut textarea = TextArea::new();
        if let Some(val) = prefill {
            textarea.insert_str(&val);
        }

        let (result_tx, result_rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            context,
            state: AuthFlowState::ApiKeyEntry { textarea },
            complete: false,
            orbit_code_home,
            store_mode,
            auth_manager,
            app_event_tx,
            result_rx,
            result_tx,
            device_code_rx: None,
        }
    }

    pub(crate) fn new_openai_browser_login(
        context: AuthLaunchContext,
        orbit_code_home: std::path::PathBuf,
        store_mode: AuthCredentialsStoreMode,
        auth_manager: Arc<AuthManager>,
        app_event_tx: AppEventSender,
        forced_workspace_id: Option<String>,
    ) -> Result<Self, String> {
        let (result_tx, result_rx) = tokio::sync::mpsc::unbounded_channel();

        let handle = crate::auth_flow::start_chatgpt_browser_login(
            &orbit_code_home,
            store_mode,
            auth_manager.clone(),
            forced_workspace_id,
            result_tx.clone(),
            app_event_tx.clone(),
        )?;

        Ok(Self {
            context,
            state: AuthFlowState::OpenAiBrowserPending { _handle: handle },
            complete: false,
            orbit_code_home,
            store_mode,
            auth_manager,
            app_event_tx,
            result_rx,
            result_tx,
            device_code_rx: None,
        })
    }

    pub(crate) fn new_openai_device_code_login(
        context: AuthLaunchContext,
        orbit_code_home: std::path::PathBuf,
        store_mode: AuthCredentialsStoreMode,
        auth_manager: Arc<AuthManager>,
        app_event_tx: AppEventSender,
        forced_workspace_id: Option<String>,
    ) -> Result<Self, String> {
        let (result_tx, result_rx) = tokio::sync::mpsc::unbounded_channel();

        let (handle, dc_rx) = crate::auth_flow::start_chatgpt_device_code_login(
            &orbit_code_home,
            store_mode,
            auth_manager.clone(),
            forced_workspace_id,
            result_tx.clone(),
            app_event_tx.clone(),
        )?;

        Ok(Self {
            context,
            state: AuthFlowState::OpenAiDeviceCodePending {
                _handle: handle,
                device_code: None,
            },
            complete: false,
            orbit_code_home,
            store_mode,
            auth_manager,
            app_event_tx,
            result_rx,
            result_tx,
            device_code_rx: Some(dc_rx),
        })
    }

    pub(crate) fn new_anthropic_oauth(
        context: AuthLaunchContext,
        orbit_code_home: std::path::PathBuf,
        store_mode: AuthCredentialsStoreMode,
        auth_manager: Arc<AuthManager>,
        app_event_tx: AppEventSender,
    ) -> Result<Self, String> {
        let (auth_url, verifier) = crate::auth_flow::start_anthropic_oauth()?;
        let (result_tx, result_rx) = tokio::sync::mpsc::unbounded_channel();

        Ok(Self {
            context,
            state: AuthFlowState::AnthropicOAuthCodeEntry {
                auth_url,
                verifier,
                textarea: TextArea::new(),
                textarea_state: RefCell::new(TextAreaState::default()),
            },
            complete: false,
            orbit_code_home,
            store_mode,
            auth_manager,
            app_event_tx,
            result_rx,
            result_tx,
            device_code_rx: None,
        })
    }

    /// Poll for async results (device codes, auth completions).
    /// Returns `true` if any state was mutated.
    fn drain_async_results(&mut self) -> bool {
        let mut changed = false;

        // Drain device code channel.
        if let Some(dc_rx) = &mut self.device_code_rx {
            while let Ok(dc) = dc_rx.try_recv() {
                if let AuthFlowState::OpenAiDeviceCodePending { device_code, .. } = &mut self.state
                {
                    *device_code = Some(dc);
                    changed = true;
                }
            }
        }

        // Drain auth result channel.
        while let Ok(result) = self.result_rx.try_recv() {
            self.handle_auth_result(result);
            changed = true;
        }

        changed
    }

    fn handle_auth_result(&mut self, result: AuthAttemptResult) {
        // Verify the attempt ID matches what we're currently holding.
        let held_id = match &self.state {
            AuthFlowState::OpenAiBrowserPending { _handle } => Some(_handle.id),
            AuthFlowState::OpenAiDeviceCodePending { _handle, .. } => Some(_handle.id),
            AuthFlowState::AnthropicOAuthExchanging { _handle } => Some(_handle.id),
            _ => None,
        };

        if held_id != Some(result.id) {
            tracing::debug!("discarding stale auth attempt result");
            return;
        }

        match result.outcome {
            Ok(success) => self.on_auth_success(success),
            Err(AuthError::ExchangeFailed(msg)) | Err(AuthError::StorageFailed(msg)) => {
                tracing::error!("auth flow error: {msg}");
                self.state = AuthFlowState::InlineError { message: msg };
            }
        }
    }

    fn on_auth_success(&mut self, _success: AuthSuccess) {
        let provider = self.context.provider();
        match &self.context {
            AuthLaunchContext::ManageProvider { .. } => {
                // Pop this view and re-open provider management with fresh state.
                self.complete = true;
                self.app_event_tx
                    .send(AppEvent::ManageAuthProvider { provider });
            }
            AuthLaunchContext::ModelSwitch { model, effort, .. } => {
                // Apply the model switch.
                let model = model.clone();
                let effort = *effort;
                self.complete = true;
                self.app_event_tx.send(AppEvent::UpdateModel(model.clone()));
                self.app_event_tx
                    .send(AppEvent::UpdateReasoningEffort(effort));
                self.app_event_tx
                    .send(AppEvent::PersistModelSelection { model, effort });
            }
        }
    }

    fn handle_api_key_submit(&mut self) {
        let text = if let AuthFlowState::ApiKeyEntry { textarea, .. } = &self.state {
            textarea.text().trim().to_string()
        } else {
            return;
        };

        if text.is_empty() {
            self.state = AuthFlowState::InlineError {
                message: "API key cannot be empty.".to_string(),
            };
            return;
        }

        let provider = self.context.provider();
        match crate::auth_flow::save_api_key(
            &self.orbit_code_home,
            provider,
            &text,
            self.store_mode,
        ) {
            Ok(()) => {
                self.auth_manager.reload();
                self.on_auth_success(AuthSuccess::ApiKeySaved);
            }
            Err(e) => {
                self.state = AuthFlowState::InlineError { message: e };
            }
        }
    }

    fn handle_anthropic_oauth_submit(&mut self) {
        let (code_text, verifier) =
            if let AuthFlowState::AnthropicOAuthCodeEntry {
                textarea, verifier, ..
            } = &self.state
            {
                let text = textarea.text().trim().to_string();
                (text, verifier.clone())
            } else {
                return;
            };

        if code_text.is_empty() {
            self.state = AuthFlowState::InlineError {
                message: "Please paste the authorization code.".to_string(),
            };
            return;
        }

        let handle = crate::auth_flow::exchange_anthropic_oauth_code(
            code_text,
            verifier,
            &self.orbit_code_home,
            self.store_mode,
            self.auth_manager.clone(),
            self.result_tx.clone(),
            self.app_event_tx.clone(),
        );

        self.state = AuthFlowState::AnthropicOAuthExchanging { _handle: handle };
    }

    fn title_text(&self) -> String {
        let provider = provider_display_name(self.context.provider());
        match &self.state {
            AuthFlowState::ApiKeyEntry { .. } => format!("Enter {provider} API Key"),
            AuthFlowState::OpenAiBrowserPending { .. } => "Sign in with ChatGPT".to_string(),
            AuthFlowState::OpenAiDeviceCodePending { .. } => "Sign in with Device Code".to_string(),
            AuthFlowState::AnthropicOAuthCodeEntry { .. } => {
                "Sign in with Claude (OAuth)".to_string()
            }
            AuthFlowState::AnthropicOAuthExchanging { .. } => "Exchanging code...".to_string(),
            AuthFlowState::InlineError { .. } => "Error".to_string(),
        }
    }
}

// ── BottomPaneView implementation ────────────────────────────────

impl BottomPaneView for AuthFlowView {
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        self.drain_async_results();

        // Esc is handled by BottomPane via on_ctrl_c (pop one view).
        // handle_key_event only receives non-Esc keys.
        match &mut self.state {
            AuthFlowState::ApiKeyEntry { textarea } => match key_event {
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    self.handle_api_key_submit();
                }
                other => {
                    textarea.input(other);
                }
            },
            AuthFlowState::AnthropicOAuthCodeEntry { textarea, .. } => match key_event {
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    self.handle_anthropic_oauth_submit();
                }
                other => {
                    textarea.input(other);
                }
            },
            // Pending and error states have no non-Esc key handling.
            AuthFlowState::InlineError { .. }
            | AuthFlowState::OpenAiBrowserPending { .. }
            | AuthFlowState::OpenAiDeviceCodePending { .. }
            | AuthFlowState::AnthropicOAuthExchanging { .. } => {}
        }
    }

    fn on_ctrl_c(&mut self) -> CancellationEvent {
        self.complete = true;
        CancellationEvent::Handled
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn handle_paste(&mut self, pasted: String) -> bool {
        if pasted.is_empty() {
            return false;
        }
        match &mut self.state {
            AuthFlowState::ApiKeyEntry { textarea, .. }
            | AuthFlowState::AnthropicOAuthCodeEntry { textarea, .. } => {
                textarea.insert_str(&pasted);
                true
            }
            _ => false,
        }
    }

    // prefer_esc_to_handle_key_event defaults to false:
    // Esc goes through on_ctrl_c → pop (not clear), which preserves
    // the provider-management popup underneath for ManageProvider context.
    // For ModelSwitch, replace_all_views already cleared the stack during
    // construction, so pop returns to the composer as intended.

    fn poll_async_state(&mut self) -> bool {
        self.drain_async_results()
    }
}

// ── Rendering ─────────────────────────────────────────────────────

impl Renderable for AuthFlowView {
    fn desired_height(&self, width: u16) -> u16 {
        match &self.state {
            AuthFlowState::ApiKeyEntry { textarea, .. } => {
                let usable = width.saturating_sub(2);
                let text_h = textarea.desired_height(usable).clamp(1, 4);
                // title + blank + textarea_header + textarea + blank + hint = 1+1+1+h+1+1
                5u16.saturating_add(text_h)
            }
            AuthFlowState::AnthropicOAuthCodeEntry { textarea, .. } => {
                let usable = width.saturating_sub(2);
                let text_h = textarea.desired_height(usable).clamp(1, 4);
                // title + blank + url + blank + textarea_header + textarea + blank + hint
                7u16.saturating_add(text_h)
            }
            AuthFlowState::OpenAiDeviceCodePending { device_code, .. } => {
                if device_code.is_some() {
                    8 // title + blank + code + url + blank + waiting + blank + hint
                } else {
                    5 // title + blank + waiting + blank + hint
                }
            }
            _ => 5, // title + blank + message + blank + hint
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Clear the area first.
        Clear.render(area, buf);

        let title = self.title_text();
        let mut y = area.y;

        // Title line.
        let title_line: Line<'static> = Line::from(vec![gutter(), title.bold()]);
        render_line_at(area.x, y, area.width, &title_line, buf);
        y = y.saturating_add(1);

        // Blank line.
        y = y.saturating_add(1);

        match &self.state {
            AuthFlowState::ApiKeyEntry { textarea } => {
                let hint: Line<'static> =
                    Line::from(vec![gutter(), "Paste or type your API key:".dim()]);
                render_line_at(area.x, y, area.width, &hint, buf);
                y = y.saturating_add(1);

                // Render textarea (masked).
                let usable_w = area.width.saturating_sub(2);
                let text_h = textarea.desired_height(usable_w).clamp(1, 4);
                if usable_w > 0 && text_h > 0 {
                    // Render gutter for each textarea row.
                    for row in 0..text_h {
                        render_line_at(
                            area.x,
                            y.saturating_add(row),
                            2,
                            &Line::from(vec![gutter()]),
                            buf,
                        );
                    }
                    let ta_rect = Rect {
                        x: area.x.saturating_add(2),
                        y,
                        width: usable_w,
                        height: text_h,
                    };

                    // Render masked text instead of the actual textarea content.
                    let masked = mask_for_display(textarea.text());
                    if masked.is_empty() {
                        Paragraph::new(Line::from("Enter API key...".dim())).render(ta_rect, buf);
                    } else {
                        Paragraph::new(Line::from(masked)).render(ta_rect, buf);
                    }
                    y = y.saturating_add(text_h);
                }
            }
            AuthFlowState::OpenAiBrowserPending { .. } => {
                let msg: Line<'static> =
                    Line::from(vec![gutter(), "Waiting for browser sign-in...".dim()]);
                render_line_at(area.x, y, area.width, &msg, buf);
                y = y.saturating_add(1);
            }
            AuthFlowState::OpenAiDeviceCodePending { device_code, .. } => {
                if let Some(dc) = device_code {
                    let code_line: Line<'static> = Line::from(vec![
                        gutter(),
                        "Code: ".into(),
                        dc.user_code.clone().cyan().bold(),
                    ]);
                    render_line_at(area.x, y, area.width, &code_line, buf);
                    y = y.saturating_add(1);

                    let url_line: Line<'static> = Line::from(vec![
                        gutter(),
                        "URL:  ".into(),
                        dc.verification_url.clone().cyan().underlined(),
                    ]);
                    render_line_at(area.x, y, area.width, &url_line, buf);
                    y = y.saturating_add(1);

                    y = y.saturating_add(1);
                    let wait: Line<'static> =
                        Line::from(vec![gutter(), "Waiting for authorization...".dim()]);
                    render_line_at(area.x, y, area.width, &wait, buf);
                    y = y.saturating_add(1);
                } else {
                    let msg: Line<'static> =
                        Line::from(vec![gutter(), "Requesting device code...".dim()]);
                    render_line_at(area.x, y, area.width, &msg, buf);
                    y = y.saturating_add(1);
                }
            }
            AuthFlowState::AnthropicOAuthCodeEntry {
                auth_url,
                textarea,
                textarea_state,
                ..
            } => {
                let url_line: Line<'static> =
                    Line::from(vec![gutter(), auth_url.clone().cyan().underlined()]);
                render_line_at(area.x, y, area.width, &url_line, buf);
                y = y.saturating_add(1);

                y = y.saturating_add(1);
                let hint: Line<'static> = Line::from(vec![
                    gutter(),
                    "Paste the authorization code from your browser:".dim(),
                ]);
                render_line_at(area.x, y, area.width, &hint, buf);
                y = y.saturating_add(1);

                let usable_w = area.width.saturating_sub(2);
                let text_h = textarea.desired_height(usable_w).clamp(1, 4);
                if usable_w > 0 && text_h > 0 {
                    for row in 0..text_h {
                        render_line_at(
                            area.x,
                            y.saturating_add(row),
                            2,
                            &Line::from(vec![gutter()]),
                            buf,
                        );
                    }
                    let ta_rect = Rect {
                        x: area.x.saturating_add(2),
                        y,
                        width: usable_w,
                        height: text_h,
                    };
                    let mut state = textarea_state.borrow_mut();
                    StatefulWidgetRef::render_ref(&textarea, ta_rect, buf, &mut state);
                    if textarea.text().is_empty() {
                        Paragraph::new(Line::from("Paste code here...".dim())).render(ta_rect, buf);
                    }
                    y = y.saturating_add(text_h);
                }
            }
            AuthFlowState::AnthropicOAuthExchanging { .. } => {
                let msg: Line<'static> =
                    Line::from(vec![gutter(), "Exchanging code for tokens...".dim()]);
                render_line_at(area.x, y, area.width, &msg, buf);
                y = y.saturating_add(1);
            }
            AuthFlowState::InlineError { message } => {
                let msg: Line<'static> = Line::from(vec![gutter(), message.clone().red()]);
                render_line_at(area.x, y, area.width, &msg, buf);
                y = y.saturating_add(1);
            }
        }

        // Blank line before hint.
        y = y.saturating_add(1);

        // Hint line.
        let hint = match &self.state {
            AuthFlowState::ApiKeyEntry { .. } => "Enter to save  |  Esc to cancel",
            AuthFlowState::AnthropicOAuthCodeEntry { .. } => "Enter to exchange  |  Esc to cancel",
            AuthFlowState::InlineError { .. } => "Press Esc to go back",
            _ => "Esc to cancel",
        };
        if y < area.y.saturating_add(area.height) {
            let hint_line: Line<'static> = Line::from(vec![gutter(), hint.dim()]);
            render_line_at(area.x, y, area.width, &hint_line, buf);
        }
    }

    fn cursor_pos(&self, area: Rect) -> Option<(u16, u16)> {
        match &self.state {
            AuthFlowState::ApiKeyEntry { textarea, .. } => {
                // Cursor after the masked text on the textarea line.
                let text_len = textarea.text().len() as u16;
                let x = area
                    .x
                    .saturating_add(2)
                    .saturating_add(text_len.min(area.width.saturating_sub(3)));
                let y = area.y.saturating_add(3); // title + blank + hint + textarea
                Some((x, y))
            }
            AuthFlowState::AnthropicOAuthCodeEntry {
                textarea,
                textarea_state,
                ..
            } => {
                let usable_w = area.width.saturating_sub(2);
                let text_h = textarea.desired_height(usable_w).clamp(1, 4);
                let ta_y = area.y.saturating_add(5); // title+blank+url+blank+hint+textarea
                let ta_rect = Rect {
                    x: area.x.saturating_add(2),
                    y: ta_y,
                    width: usable_w,
                    height: text_h,
                };
                let state = *textarea_state.borrow();
                textarea.cursor_pos_with_state(ta_rect, state)
            }
            _ => None,
        }
    }
}

// ── Rendering helpers ─────────────────────────────────────────────

fn gutter() -> Span<'static> {
    "▌ ".cyan()
}

fn render_line_at(x: u16, y: u16, width: u16, line: &Line<'_>, buf: &mut Buffer) {
    if width == 0 {
        return;
    }
    let rect = Rect {
        x,
        y,
        width,
        height: 1,
    };
    Paragraph::new(line.clone()).render(rect, buf);
}

/// Mask a string for display: show first 4 chars, then asterisks.
fn mask_for_display(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let len = text.len();
    if len <= 4 {
        return "*".repeat(len);
    }
    let prefix = &text[..4];
    let masked = "*".repeat(len.saturating_sub(4).min(20));
    format!("{prefix}{masked}")
}
