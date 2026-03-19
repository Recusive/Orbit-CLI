# codex-rs/login/src/assets/

This file applies to `codex-rs/login/src/assets/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-login` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-login`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Static HTML templates for the login callback server's browser-facing responses.

### What this folder does

Contains HTML files that are embedded into the binary at compile time via `include_str!()` in `server.rs`. They are served as HTTP responses during the OAuth login callback flow.

### Key files

| File | Purpose |
|------|---------|
| `success.html` | Branded success page displayed after a successful OAuth login. Shows a confirmation message and redirects the user back to the CLI |
| `error.html` | Branded error page displayed when login fails. Contains template placeholders (`__ERROR_TITLE__`, `__ERROR_MESSAGE__`, `__ERROR_CODE__`, `__ERROR_DESCRIPTION__`, `__ERROR_HELP__`) that are replaced at runtime with HTML-escaped error details. Handles both generic errors and specific cases like missing Codex entitlement |

### Where it plugs in

- **Consumed by**: `server.rs` via `include_str!("assets/success.html")` and `include_str!("assets/error.html")`
- These files are compiled into the binary; no runtime file access is needed
