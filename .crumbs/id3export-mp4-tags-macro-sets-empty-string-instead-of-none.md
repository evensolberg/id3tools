---
id: id3-gdi
title: 'id3export: mp4_tags macro sets empty string instead of None'
status: open
type: bug
priority: 3
tags:
- id3export
- low
- consistency
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3export/src/tracks.rs:70 — mp4_tags! macro sets Some("") for missing fields instead of None. Inconsistent CSV output vs FLAC/APE handling.
---

# id3export: mp4_tags macro sets empty string instead of None

id3export/src/tracks.rs:70 — mp4_tags! macro sets Some("") for missing fields instead of None. Inconsistent CSV output vs FLAC/APE handling.
