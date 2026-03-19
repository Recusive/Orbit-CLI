# codex-rs/responses-api-proxy/npm/

This file applies to `codex-rs/responses-api-proxy/npm/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-responses-api-proxy` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-responses-api-proxy`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

npm package for distributing the `codex-responses-api-proxy` binary.

### What this folder does

Provides the npm package `@openai/codex-responses-api-proxy` that wraps the compiled Rust binary for cross-platform distribution via npm. The package includes a Node.js launcher script that detects the host platform/architecture and spawns the appropriate native binary from the `vendor/` directory.

### Key files

- `package.json` -- npm package definition:
  - Name: `@openai/codex-responses-api-proxy`
  - Type: ESM (`"type": "module"`)
  - Binary entry: `bin/codex-responses-api-proxy.js`
  - Includes `bin/` and `vendor/` directories in the published package
  - Requires Node.js >= 16
- `bin/` -- launcher script directory

### What it plugs into

- Published to npm as part of the Codex CLI release process
- The compiled Rust binaries for each target triple are placed in `vendor/<triple>/codex-responses-api-proxy/` during the build/release pipeline
- The Codex Node.js CLI spawns this package's binary to proxy Responses API requests

### Imports from

- Rust binary compiled from `codex-rs/responses-api-proxy/src/`

### Exports to

- Provides the `codex-responses-api-proxy` CLI command when installed via npm
