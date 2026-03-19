# codex-rs/utils/rustls-provider/

Crate `codex-utils-rustls-provider` -- one-time rustls crypto provider initialization.

## What this folder does

Ensures exactly one process-wide rustls crypto provider is installed. This is necessary because rustls cannot auto-select a provider when both `ring` and `aws-lc-rs` features are enabled in the dependency graph. Uses `std::sync::Once` for thread-safe initialization.

## Key types and functions

- `ensure_rustls_crypto_provider()` -- installs the `ring` default provider via `Once::call_once`; safe to call multiple times

## Imports from

- `rustls` -- TLS library whose crypto provider is being configured

## Exports to

Called early in startup by `codex-core` and any crate that needs TLS before making HTTP requests.

## Key files

- `Cargo.toml` -- crate metadata; depends on `rustls`
- `src/lib.rs` -- single function `ensure_rustls_crypto_provider` (13 lines)
