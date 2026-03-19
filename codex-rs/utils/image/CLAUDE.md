# codex-rs/utils/image/

Crate `codex-utils-image` -- image processing for LLM prompts.

## What this folder does

Loads, validates, optionally resizes, and encodes images for inclusion in LLM prompts. Supports PNG, JPEG, GIF, and WebP formats. Uses a global LRU cache keyed by content SHA-1 to avoid reprocessing identical files.

## Key types and functions

- `EncodedImage` -- struct holding encoded bytes, MIME type, width, and height; has `into_data_url()` for base64 data URLs
- `PromptImageMode` -- enum: `ResizeToFit` (max 2048x768) or `Original`
- `load_for_prompt_bytes(path, file_bytes, mode) -> Result<EncodedImage, ImageProcessingError>` -- main entry point; auto-detects format, optionally resizes, preserves source bytes when possible
- `MAX_WIDTH` / `MAX_HEIGHT` -- 2048x768 resize constraints

## Imports from

- `base64` -- base64 encoding for data URLs
- `image` -- decoding, resizing, and encoding (PNG, JPEG, GIF, WebP)
- `codex-utils-cache` -- `BlockingLruCache` and `sha1_digest` for content-addressed caching
- `mime_guess` -- MIME type detection from file paths
- `thiserror` -- error type derivation

## Exports to

Consumed by `codex-core` when preparing image content for model requests.

## Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- `EncodedImage`, `PromptImageMode`, `load_for_prompt_bytes`, image encoding/resizing logic, global `IMAGE_CACHE`, and tests
- `src/error.rs` -- `ImageProcessingError` enum with variants for read, decode, encode, and unsupported format errors
