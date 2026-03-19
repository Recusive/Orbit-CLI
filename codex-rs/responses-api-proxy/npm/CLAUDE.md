# codex-rs/responses-api-proxy/npm/

npm package for distributing the `codex-responses-api-proxy` binary.

## What this folder does

Provides the npm package `@openai/codex-responses-api-proxy` that wraps the compiled Rust binary for cross-platform distribution via npm. The package includes a Node.js launcher script that detects the host platform/architecture and spawns the appropriate native binary from the `vendor/` directory.

## Key files

- `package.json` -- npm package definition:
  - Name: `@openai/codex-responses-api-proxy`
  - Type: ESM (`"type": "module"`)
  - Binary entry: `bin/codex-responses-api-proxy.js`
  - Includes `bin/` and `vendor/` directories in the published package
  - Requires Node.js >= 16
- `bin/` -- launcher script directory

## What it plugs into

- Published to npm as part of the Codex CLI release process
- The compiled Rust binaries for each target triple are placed in `vendor/<triple>/codex-responses-api-proxy/` during the build/release pipeline
- The Codex Node.js CLI spawns this package's binary to proxy Responses API requests

## Imports from

- Rust binary compiled from `codex-rs/responses-api-proxy/src/`

## Exports to

- Provides the `codex-responses-api-proxy` CLI command when installed via npm
