# codex-rs/rmcp-client/src/

Source code for the `codex-rmcp-client` MCP client library.

## What this folder does

Contains the implementation of the MCP client, OAuth authentication, auth-status discovery, credential persistence, and utility modules.

## Key files

- `lib.rs` -- module declarations and public API re-exports.
- `rmcp_client.rs` -- core `RmcpClient` implementation: manages client state (Connecting/Ready), creates stdio or streamable-HTTP transports, performs MCP initialize handshake, executes tool calls / resource reads / custom requests, handles session recovery on 404 expiry, and manages OAuth token refresh/persistence.
- `auth_status.rs` -- `determine_streamable_http_auth_status()` probes an MCP server to classify its auth mode (BearerToken, OAuth, NotLoggedIn, Unsupported) using RFC 8414 well-known discovery paths.
- `oauth.rs` -- credential storage layer: loads/saves/deletes OAuth tokens using OS keyring (`keyring` crate) with fallback to `CODEX_HOME/.credentials.json`. Includes `OAuthPersistor` for automatic persistence after token refresh.
- `perform_oauth_login.rs` -- implements the full OAuth authorization-code flow: spawns a local HTTP callback server, launches the browser, handles the callback, exchanges the code for tokens, and persists them.
- `program_resolver.rs` -- resolves MCP server executable paths. On Unix this is a no-op; on Windows it uses the `which` crate to resolve extensions (.cmd, .bat).
- `logging_client_handler.rs` -- `LoggingClientHandler` implements the rmcp `ClientHandler` trait, forwarding elicitation requests and logging MCP server notifications at appropriate tracing levels.
- `utils.rs` -- helper functions: `create_env_for_mcp_server()` builds a sanitized environment from an allowlist of variables; `build_default_headers()` / `apply_default_headers()` construct HTTP headers from static and env-sourced values.

## Imports from

- `rmcp` SDK for MCP protocol types and service lifecycle.
- `codex-client`, `codex-protocol`, `codex-keyring-store`, `codex-utils-pty`, `codex-utils-home-dir`.

## Exports to

- Parent crate (`codex-rmcp-client`) re-exports the public API defined here.
