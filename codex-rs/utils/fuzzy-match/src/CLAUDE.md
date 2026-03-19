# codex-rs/utils/fuzzy-match/src/

Source directory for the `codex-utils-fuzzy-match` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `fuzzy_match(haystack: &str, needle: &str) -> Option<(Vec<usize>, i32)>` -- the core algorithm:
    - Lowercases both strings, maintaining a mapping from lowered char indices back to original haystack char indices
    - Greedily matches needle characters as a subsequence in the lowered haystack
    - Scores based on span window minus needle length; -100 bonus for prefix matches
    - Empty needle matches everything with `i32::MAX` score
  - `fuzzy_indices(haystack, needle) -> Option<Vec<usize>>` -- returns only the deduped, sorted indices
  - Tests covering ASCII, Unicode (Turkish dotted-I, German sharp-s), contiguous vs spread matches, prefix bonuses, and multi-char lowercase expansion
