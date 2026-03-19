# codex-rs/utils/string/

Crate `codex-utils-string` -- string utility functions for byte-boundary truncation, UUID extraction, and metric sanitization.

## What this folder does

Provides small, focused string manipulation utilities used across the Codex codebase for output truncation, metric tag sanitization, UUID extraction, and markdown location suffix normalization.

## Key types and functions

- `take_bytes_at_char_boundary(s, maxb)` -- truncate a string prefix to a byte budget, respecting char boundaries
- `take_last_bytes_at_char_boundary(s, maxb)` -- take a string suffix within a byte budget, respecting char boundaries
- `sanitize_metric_tag_value(value)` -- replace non-alphanumeric/non-separator characters with `_`; trim; cap at 256 chars; return `"unspecified"` for invalid values
- `find_uuids(s)` -- extract all UUID strings matching the standard 8-4-4-4-12 hex pattern
- `normalize_markdown_hash_location_suffix(suffix)` -- convert markdown `#L74C3-L76C9` style suffixes to terminal-friendly `:74:3-76:9` format

## Imports from

- `regex-lite` -- lightweight regex for UUID extraction

## Exports to

Used by `codex-core` for output truncation, `codex-tui` for display formatting, and telemetry code for metric tag sanitization.

## Key files

- `Cargo.toml` -- crate metadata; depends on `regex-lite`
- `src/lib.rs` -- all functions, `OnceLock`-cached UUID regex, and tests
