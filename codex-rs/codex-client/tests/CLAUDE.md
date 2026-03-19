# codex-rs/codex-client/tests/

Tests for the `codex-client` crate.

## What this folder does

Contains tests for custom CA certificate handling, including environment variable-based cert loading and certificate chain validation.

## Key files

| File | Role |
|------|------|
| `ca_env.rs` | Tests for `NODE_EXTRA_CA_CERTS` environment variable handling and custom CA loading |
| `fixtures/` | Test certificate files |
