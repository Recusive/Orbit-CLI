# codex-rs/login/tests/suite/

This file applies to `codex-rs/login/tests/suite/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-login` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-login`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test module directory aggregated by `tests/all.rs`.

### What this folder does

Contains integration test modules that exercise the full login flows end-to-end with mock auth servers.

### Key files

| File | What it tests |
|------|---------------|
| `mod.rs` | Aggregates `device_code_login` and `login_server_e2e` modules |
| `device_code_login.rs` | Device code flow integration tests using wiremock: successful login with token persistence, workspace mismatch rejection, HTTP failure handling, login without API key exchange, error payload handling. Constructs fake JWTs and validates `auth.json` contents after each flow |
| `login_server_e2e.rs` | Browser OAuth flow end-to-end tests using a tiny_http mock issuer: full callback flow with token persistence, directory creation for missing codex home, workspace mismatch blocking, missing Codex entitlement error page rendering, generic OAuth denial error page, and cancellation of a previous login server occupying the port |

### Test patterns

- All tests use `skip_if_no_network!()` macro to skip when sandbox network is disabled.
- Mock JWTs are created with `make_jwt()` helper (base64-encoded header/payload/signature).
- `ServerOptions` is constructed with `open_browser: false` and a temporary codex home directory.
- Tests validate both HTTP response content and persisted `auth.json` state.
