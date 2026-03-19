# codex-rs/tui_app_server/src/onboarding/auth/

This file applies to `codex-rs/tui_app_server/src/onboarding/auth/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Headless ChatGPT authentication helper for the onboarding flow.

### What this folder does

Contains the headless ChatGPT login implementation used during the onboarding authentication step. This handles browser-based ChatGPT authentication in environments where a full browser UI is not available.

### What it plugs into

- **../auth.rs**: The `AuthModeWidget` delegates to this module for ChatGPT browser-based login when that sign-in method is selected.
- **codex_app_server_protocol**: Login and account API types.

### Key files

| File | Role |
|------|------|
| `headless_chatgpt_login.rs` | Implements headless ChatGPT login -- opens a browser for OAuth, polls for completion, and returns auth tokens to the onboarding flow. |

### Imports from

- `codex_app_server_protocol` -- login types.
- Standard library and async runtime.

### Exports to

- **../auth.rs**: Login result types used by the authentication widget.
