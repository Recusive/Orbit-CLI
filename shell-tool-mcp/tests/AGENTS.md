# shell-tool-mcp/tests/

This file applies to `shell-tool-mcp/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `@openai/codex-shell-tool-mcp` package. Keep `package.json` entry points, exports, and scripts aligned with source changes.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/shell-tool-mcp && pnpm build`
- `cd /Users/no9labs/Developer/Recursive/codex/shell-tool-mcp && pnpm test`
- `cd /Users/no9labs/Developer/Recursive/codex/shell-tool-mcp && pnpm format`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Jest test suite for the `@openai/codex-shell-tool-mcp` TypeScript package. Tests validate the platform-aware Bash binary selection logic and OS release file parsing.

### Key Files

| File | Role |
|------|------|
| `bashSelection.test.ts` | Tests for `selectLinuxBash()` and `selectDarwinBash()`. Verifies exact version matching (e.g., Ubuntu 24.04), fallback to first supported variant for unknown distros, Darwin kernel version mapping (e.g., Darwin 24.x maps to macOS 15), and fallback for old Darwin versions. |
| `osRelease.test.ts` | Tests for `parseOsRelease()`. Verifies correct parsing of `ID`, `ID_LIKE`, and `VERSION_ID` fields from `/etc/os-release` format, including quoted values, missing fields, and normalization of `ID_LIKE` entries. |

### Running Tests

```bash
cd shell-tool-mcp
pnpm test               # Run all tests
pnpm test -- --watch    # Watch mode
```

### Imports From

- `../src/bashSelection` (selectLinuxBash, selectDarwinBash)
- `../src/constants` (LINUX_BASH_VARIANTS, DARWIN_BASH_VARIANTS)
- `../src/osRelease` (parseOsRelease)
- `../src/types` (OsReleaseInfo)
