# codex-rs/codex-client/src/bin/

Binary targets for the `codex-client` crate.

## What this folder does

Contains a test binary used to verify custom CA certificate loading behavior in a subprocess.

## Key files

| File | Role |
|------|------|
| `custom_ca_probe.rs` | Test binary that exercises `build_reqwest_client_for_subprocess_tests` to verify custom CA cert loading from `NODE_EXTRA_CA_CERTS` in an isolated process |
