# codex-rs/utils/fuzzy-match/

Crate `codex-utils-fuzzy-match` -- case-insensitive fuzzy subsequence matching.

## What this folder does

Implements a simple fuzzy matcher that finds a case-insensitive subsequence of needle characters in a haystack string. Returns matched character indices (in the original haystack) and a score (lower is better) that rewards contiguous matches and prefix matches.

## Key types and functions

- `fuzzy_match(haystack, needle) -> Option<(Vec<usize>, i32)>` -- returns matched indices and score; `None` if needle is not a subsequence
- `fuzzy_indices(haystack, needle) -> Option<Vec<usize>>` -- convenience wrapper returning just indices
- Scoring: contiguous matches score 0, spread matches penalized by window gap, prefix matches get a -100 bonus

## Imports from

No external dependencies (std only).

## Exports to

Used by `codex-tui` for fuzzy filtering in interactive selection UIs.

## Key files

- `Cargo.toml` -- crate metadata (no dependencies)
- `src/lib.rs` -- `fuzzy_match`, `fuzzy_indices`, Unicode-aware lowercase mapping, and comprehensive tests including Unicode edge cases
