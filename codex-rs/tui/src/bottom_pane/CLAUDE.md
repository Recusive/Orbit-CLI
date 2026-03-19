# codex-rs/tui/src/bottom_pane/

The interactive footer pane of the chat UI.

## What this folder does

Implements the `BottomPane` -- the interactive footer area that owns the chat composer (editable prompt input) and a stack of transient overlay views (popups/modals). Input routing is layered: the bottom pane decides which local surface (view vs. composer) receives a key event, while higher-level intent (interrupt, quit) is decided by the parent `ChatWidget`.

## What it plugs into

- **../chatwidget.rs**: `ChatWidget` owns a `BottomPane` and delegates input handling and rendering to it.
- **../app.rs**: `App` reads approval requests, selection views, and feedback targets from the bottom pane's view stack.
- **codex-protocol**: Uses `RequestUserInputEvent`, `Op`, and tool approval types.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module root; declares sub-modules, defines `LocalImageAttachment`, `MentionBinding`, and re-exports key types (`ApprovalRequest`, `McpServerElicitationFormRequest`, etc.). |
| `chat_composer.rs` | `ChatComposer` -- multi-line text input with cursor, paste handling, mention completion, image attachments, and submit semantics. |
| `chat_composer_history.rs` | Input history (up/down arrow) for the composer. |
| `bottom_pane_view.rs` | `BottomPaneView` enum -- the stack of transient views that can replace the composer (approval overlay, selection list, etc.). |
| `approval_overlay.rs` | `ApprovalOverlay` -- modal for exec/patch approval prompts with accept/reject/always-approve actions. |
| `mcp_server_elicitation.rs` | `McpServerElicitationOverlay` -- modal for MCP server elicitation form requests. |
| `command_popup.rs` | Slash-command popup (e.g., `/help`, `/status`, `/model`). |
| `file_search_popup.rs` | File search popup triggered by `@` mentions. |
| `skill_popup.rs` | Skill selection popup. |
| `feedback_view.rs` | Feedback submission view. |
| `list_selection_view.rs` | Generic list selection view for choices. |
| `footer.rs` | Footer bar rendering with key hints. |
| `status_line_setup.rs` | `StatusLineSetupView` -- status line configuration and display. |
| `unified_exec_footer.rs` | Unified footer for exec-mode operations. |
| `pending_input_preview.rs` | Preview widget for pending/in-flight user input. |
| `pending_thread_approvals.rs` | Pending thread approval indicators. |
| `scroll_state.rs` | Scroll position state for scrollable views. |
| `selection_popup_common.rs` | Shared row measurement and rendering for selection popups. |
| `paste_burst.rs` | Paste burst detection and handling. |
| `prompt_args.rs` | Prompt argument parsing helpers. |
| `popup_consts.rs` | Shared constants and helpers for popup sizing. |
| `custom_prompt_view.rs` | Custom prompt editing view. |
| `experimental_features_view.rs` | Feature flag toggle view. |
| `skills_toggle_view.rs` | Skills enable/disable toggle view. |
| `multi_select_picker.rs` | Multi-select picker widget. |
| `app_link_view.rs` | App link suggestion/elicitation view. |
| `slash_commands.rs` | Slash command definitions and routing. |
| `textarea.rs` | Underlying text area widget. |

## Sub-directories

| Directory | Purpose |
|-----------|---------|
| `request_user_input/` | Request-user-input overlay state machine for multi-question forms. |
| `snapshots/` | Insta test snapshots for bottom pane component tests. |

## Imports from

- `crate::app_event`, `crate::render`, `crate::key_hint`, `crate::chatwidget`
- `codex-protocol`, `codex-core`, `codex-file-search`
- `ratatui`, `crossterm`
