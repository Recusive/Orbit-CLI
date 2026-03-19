# codex-rs/codex-client/tests/fixtures/

Test certificates for custom CA handling tests.

## What this folder does

Contains PEM-encoded test certificates used by `ca_env.rs` tests to verify custom CA certificate loading and chain validation behavior.

## Key files

| File | Role |
|------|------|
| `test-ca.pem` | Self-signed test CA certificate |
| `test-ca-trusted.pem` | Trusted test CA certificate |
| `test-intermediate.pem` | Intermediate certificate for chain validation tests |
