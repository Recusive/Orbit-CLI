# codex-rs/utils/rustls-provider/src/

Source directory for the `codex-utils-rustls-provider` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `ensure_rustls_crypto_provider()` -- uses `std::sync::Once` to call `rustls::crypto::ring::default_provider().install_default()` exactly once per process lifetime
  - The `let _ =` pattern ignores the `Err` returned if another provider was already installed
