# codex-rs/keyring-store/src/

Source code for the `codex-keyring-store` crate.

## What this folder does

Contains the single-file implementation of the OS keyring abstraction.

## Key files

- `lib.rs` -- Complete crate implementation:
  - **Error type**: `CredentialStoreError` -- Wraps `keyring::Error` with `Display`, `Debug`, and `Error` impls
  - **Trait**: `KeyringStore` -- `Debug + Send + Sync` trait with `load`, `save`, `delete` methods
  - **Default impl**: `DefaultKeyringStore` -- Uses `keyring::Entry` for platform-native credential storage:
    - `load` returns `Ok(None)` for `NoEntry` errors
    - `delete` returns `Ok(false)` for `NoEntry` errors
    - All operations include `tracing::trace` logging
  - **Mock** (public `tests` module): `MockKeyringStore` -- Thread-safe in-memory store for testing:
    - Backed by `Arc<Mutex<HashMap<String, Arc<MockCredential>>>>`
    - `credential(account)` -- Get or create a mock credential
    - `saved_value(account)` -- Read stored value
    - `set_error(account, error)` -- Inject errors for testing
    - `contains(account)` -- Check existence
    - Implements `KeyringStore` trait

## Imports from / exports to

**Imports:**
- `keyring::{Entry, Error, credential::CredentialApi, mock::MockCredential}`
- `tracing::trace`
- `std::sync::{Arc, Mutex}`

**Exports:**
- `CredentialStoreError`, `KeyringStore`, `DefaultKeyringStore`
- `tests::MockKeyringStore` (publicly accessible for downstream test support)
