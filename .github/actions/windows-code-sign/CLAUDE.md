# .github/actions/windows-code-sign/

Composite GitHub Action that signs Windows release binaries using Azure Trusted Signing.

## Purpose

Signs all Windows executables (`codex.exe`, `codex-responses-api-proxy.exe`, `codex-windows-sandbox-setup.exe`, `codex-command-runner.exe`) via Azure Trusted Signing with OIDC-based authentication. This two-step action first authenticates to Azure, then invokes the signing action.

## Key Files

- **`action.yml`** -- Composite action with two steps:
  1. **Azure login** -- Uses `azure/login@v2` with OIDC federated credentials (client-id, tenant-id, subscription-id).
  2. **Sign binaries** -- Uses `azure/trusted-signing-action@v0` pointing at the built executables under `codex-rs/target/<target>/release/`. Explicitly disables all credential providers except Azure CLI to ensure OIDC authentication is used.

## Inputs

| Input | Required | Description |
|-------|----------|-------------|
| `target` | Yes | Target triple (e.g., `x86_64-pc-windows-msvc`) |
| `client-id` | Yes | Azure Trusted Signing client ID |
| `tenant-id` | Yes | Azure tenant ID |
| `subscription-id` | Yes | Azure subscription ID |
| `endpoint` | Yes | Azure Trusted Signing endpoint URL |
| `account-name` | Yes | Azure Trusted Signing account name |
| `certificate-profile-name` | Yes | Certificate profile name for signing |

## Plugs Into

- Called by `rust-release-windows.yml` during the `build-windows` job after all binaries are compiled and staged.
- Referenced as `./.github/actions/windows-code-sign` in workflow files.
- All secrets are passed from the release workflow (`AZURE_TRUSTED_SIGNING_*`).

## Imports / Dependencies

- `azure/login@v2` -- Azure OIDC login.
- `azure/trusted-signing-action@v0` -- Azure Trusted Signing integration.
- Requires `id-token: write` permission on the calling job for OIDC.
