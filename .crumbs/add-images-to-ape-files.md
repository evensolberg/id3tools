---
id: id3-k0e
title: Add images to APE files.
status: closed
type: feature
priority: 1
tags: []
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: |-
  Added `set_picture()` function in ape.rs:104-125 that:

  - Reads the image via `read_cover()` (same as FLAC/MP3 — handles resizing, aspect ratio checks)
  - Builds APE binary cover art format: `\0` prefix + raw JPEG bytes
  - Uses `Item::new(key, ItemType::Binary, data)` with standard APE keys "Cover Art (Front)" / "Cover Art (Back)"
  - Removes existing cover art before writing
  - Follows the same error handling pattern as other format handlers
---

# Add images to APE files.

Added `set_picture()` function in ape.rs:104-125 that:

- Reads the image via `read_cover()` (same as FLAC/MP3 — handles resizing, aspect ratio checks)
- Builds APE binary cover art format: `\0` prefix + raw JPEG bytes
- Uses `Item::new(key, ItemType::Binary, data)` with standard APE keys "Cover Art (Front)" / "Cover Art (Back)"
- Removes existing cover art before writing
- Follows the same error handling pattern as other format handlers
