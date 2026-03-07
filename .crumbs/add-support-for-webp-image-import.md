---
id: id3-8zt
title: Add support for WebP image import.
status: closed
type: feature
priority: 2
tags: []
created: 2026-03-07
updated: 2026-03-07
closed_reason: Implemented 2026-03-07
dependencies: []
description: |-
  ## Findings

  The `image` crate (v0.25.9) can already decode WebP. The
  current pipeline forces JPEG output in `read_cover()`.

  ### Per-format support

  - **APE**: Format-agnostic (binary data). No changes needed.
  - **FLAC**: `metaflac::add_picture()` accepts any mime type.
    Just needs `"image/webp"` instead of hardcoded JPEG.
  - **MP3**: `id3` crate accepts any mime type in frames.
    Same fix as FLAC.
  - **MP4**: `mp4ameta::ImgFmt` enum only has `Jpeg`.
    Convert WebP→JPEG before embedding in MP4.

  ### Implementation plan

  1. Enable `"webp"` feature in `id3tag/Cargo.toml`:
     `image = { workspace = true, features = ["jpeg", "png", "webp"] }`
  2. Refactor `read_cover()` to return `(Vec<u8>, String)`
     where String is the mime type (e.g. `"image/webp"`).
     Preserve original format when possible; only convert
     to JPEG when resizing or when target requires it.
  3. Update format handlers to use returned mime type:
     - FLAC: pass mime type to `add_picture()`
     - MP3: pass mime type to id3 frame
     - APE: no change (binary)
     - MP4: convert to JPEG if input is WebP
  4. Update all call sites of `read_cover()` for new
     return type.

  ### Key files

  - `id3tag/Cargo.toml` — feature flags
  - `id3tag/src/formats/images/mod.rs` — `read_cover()`
  - `id3tag/src/formats/mp3.rs` — mime type (line ~204)
  - `id3tag/src/formats/flac.rs` — mime type (line ~157)
  - `id3tag/src/formats/mp4.rs` — `ImgFmt::Jpeg` (line ~105)
  - `id3tag/src/formats/ape.rs` — no changes needed
---

# Add support for WebP image import.

## Findings

The `image` crate (v0.25.9) can already decode WebP. The
current pipeline forces JPEG output in `read_cover()`.

### Per-format support

- **APE**: Format-agnostic (binary data). No changes needed.
- **FLAC**: `metaflac::add_picture()` accepts any mime type.
  Just needs `"image/webp"` instead of hardcoded JPEG.
- **MP3**: `id3` crate accepts any mime type in frames.
  Same fix as FLAC.
- **MP4**: `mp4ameta::ImgFmt` enum only has `Jpeg`.
  Convert WebP→JPEG before embedding in MP4.

### Implementation plan

1. Enable `"webp"` feature in `id3tag/Cargo.toml`:
   `image = { workspace = true, features = ["jpeg", "png", "webp"] }`
2. Refactor `read_cover()` to return `(Vec<u8>, String)`
   where String is the mime type (e.g. `"image/webp"`).
   Preserve original format when possible; only convert
   to JPEG when resizing or when target requires it.
3. Update format handlers to use returned mime type:
   - FLAC: pass mime type to `add_picture()`
   - MP3: pass mime type to id3 frame
   - APE: no change (binary)
   - MP4: convert to JPEG if input is WebP
4. Update all call sites of `read_cover()` for new
   return type.

### Key files

- `id3tag/Cargo.toml` — feature flags
- `id3tag/src/formats/images/mod.rs` — `read_cover()`
- `id3tag/src/formats/mp3.rs` — mime type (line ~204)
- `id3tag/src/formats/flac.rs` — mime type (line ~157)
- `id3tag/src/formats/mp4.rs` — `ImgFmt::Jpeg` (line ~105)
- `id3tag/src/formats/ape.rs` — no changes needed
