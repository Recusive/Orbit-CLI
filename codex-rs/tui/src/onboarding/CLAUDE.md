# codex-rs/tui/src/onboarding/

First-run onboarding experience for the TUI.

## What this folder does

Implements the multi-step onboarding flow shown on first launch or when authentication/trust decisions are needed. The flow consists of up to three steps: a welcome screen, an authentication screen (login via browser, device code, or API key), and a directory trust decision screen. Steps are shown or hidden based on login status and project trust state.

## What it plugs into

- **../lib.rs**: `run_ratatui_app()` calls `run_onboarding_app()` before starting the main chat if onboarding is needed.
- **codex-core**: Uses `AuthManager`, `CodexAuth`, `Config` for authentication state.
- **codex-login**: Uses `DeviceCode`, `ServerOptions`, `run_login_server` for the OAuth device code flow.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module root; declares sub-modules and re-exports `TrustDirectorySelection`. |
| `onboarding_screen.rs` | `OnboardingScreen` -- the multi-step wizard that sequences Welcome, Auth, and TrustDirectory steps. Defines `KeyboardHandler` and `StepStateProvider` traits for step widgets. Contains `run_onboarding_app()` which drives the TUI event loop for onboarding. |
| `auth.rs` | `AuthModeWidget` -- the authentication step. Provides sign-in options (ChatGPT account via browser, device code login, API key entry). Manages sign-in state transitions and displays login progress with shimmer animations. |
| `trust_directory.rs` | `TrustDirectoryWidget` -- the directory trust decision step. Shows the current working directory and asks the user to trust or not trust it for unrestricted command execution. |
| `welcome.rs` | `WelcomeWidget` -- the initial welcome screen shown to new users. |

## Sub-directories

| Directory | Purpose |
|-----------|---------|
| `auth/` | Additional auth sub-modules (headless ChatGPT login). |
| `snapshots/` | Insta test snapshots for onboarding rendering tests. |

## Flow

```
OnboardingScreen
  1. Welcome (if first run)
  2. Auth (if not authenticated and provider requires auth)
     -> Browser login, device code, or API key
  3. TrustDirectory (if project trust is undecided)
     -> Trust / Don't Trust
```
