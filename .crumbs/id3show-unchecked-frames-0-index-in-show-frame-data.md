---
id: id3-4oa
title: 'id3show: unchecked frames[0] index in show_frame_data'
status: open
type: bug
priority: 2
tags:
- id3show
- high
- panic
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3show/src/mp3.rs:238 — meta.frames[0] accessed without checking if frames is empty. Will panic on an MP3 file with no frames parsed.
---

# id3show: unchecked frames[0] index in show_frame_data

id3show/src/mp3.rs:238 — meta.frames[0] accessed without checking if frames is empty. Will panic on an MP3 file with no frames parsed.
