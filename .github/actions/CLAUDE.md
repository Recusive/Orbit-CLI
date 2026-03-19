# .github/actions/

Reusable composite GitHub Actions for platform-specific code signing of Codex release binaries.

## Purpose

Provides three composite actions that handle code signing and notarization for each target platform during the release pipeline. These actions are invoked by the `rust-release.yml` and `rust-release-windows.yml` workflows.

## Directory Structure

| Directory | Platform | Signing Method |
|-----------|----------|----------------|
| `linux-code-sign/` | Linux | Sigstore cosign (keyless OIDC-based signing) |
| `macos-code-sign/` | macOS | Apple codesign + notarytool (certificate-based) |
| `windows-code-sign/` | Windows | Azure Trusted Signing (OIDC-based) |

## Binaries Signed

All three actions sign the following binaries (platform-appropriate names):
- `codex` / `codex.exe`
- `codex-responses-api-proxy` / `codex-responses-api-proxy.exe`

Windows additionally signs:
- `codex-windows-sandbox-setup.exe`
- `codex-command-runner.exe`

## Plugs Into

- **`rust-release.yml`** calls `linux-code-sign` and `macos-code-sign` during the build job.
- **`rust-release-windows.yml`** calls `windows-code-sign` during the Windows build job.
- All actions are referenced via `./.github/actions/<name>` in workflow files.

## Imports / Dependencies

- `linux-code-sign` uses `sigstore/cosign-installer@v3.7.0`.
- `macos-code-sign` uses Apple developer certificates and notarization keys passed as secrets.
- `windows-code-sign` uses `azure/login@v2` and `azure/trusted-signing-action@v0`.
