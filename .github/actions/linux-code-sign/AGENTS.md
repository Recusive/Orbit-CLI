# .github/actions/linux-code-sign/

This file applies to `.github/actions/linux-code-sign/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Keep action inputs, outputs, and side effects compatible with the calling workflows. If you rename an input or artifact path, update every workflow that references this action.
- Shell steps should stay explicit and fail-fast; prefer `set -euo pipefail` in inline bash.

## Validate
- No dedicated local build step for this directory; validate by checking the workflows or callers that reference it.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Composite GitHub Action that signs Linux release binaries using Sigstore cosign.

### Purpose

Signs the `codex` and `codex-responses-api-proxy` Linux binaries with keyless OIDC-based signing via Sigstore. Produces `.sigstore` bundle files alongside each binary that can be used for offline signature verification.

### Key Files

- **`action.yml`** -- Composite action definition. Installs cosign via `sigstore/cosign-installer@v3.7.0`, then iterates over each binary in the artifacts directory and runs `cosign sign-blob` to create `.sigstore` bundles.

### Inputs

| Input | Required | Description |
|-------|----------|-------------|
| `target` | Yes | Target triple for the artifacts being signed (e.g., `x86_64-unknown-linux-musl`) |
| `artifacts-dir` | Yes | Absolute path to the directory containing the built binaries |

### Signing Details

- Uses experimental keyless mode (`COSIGN_EXPERIMENTAL=1`) with OIDC.
- OIDC client: `sigstore`, issuer: `https://oauth2.sigstore.dev/auth`.
- Each binary gets a `.sigstore` bundle file (e.g., `codex.sigstore`).

### Plugs Into

- Called by `rust-release.yml` build job for Linux targets.
- Referenced as `./.github/actions/linux-code-sign` in workflow files.

### Imports / Dependencies

- `sigstore/cosign-installer@v3.7.0` -- Installs the cosign CLI tool.
- Requires the GitHub Actions OIDC token (`id-token: write` permission on the calling job).
