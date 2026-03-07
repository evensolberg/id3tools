---
id: id3-d9z
title: 'id3show: MP3 duration minutes calculation is wrong'
status: closed
type: bug
priority: 2
tags:
- id3show
- high
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3show/src/mp3.rs:602 — Minutes calculated as duration/60.0 gives total minutes, not remainder after hours. Same bug as flac.rs:166. Should be ((duration % 3600.0) / 60.0) as u32.
---

# id3show: MP3 duration minutes calculation is wrong

id3show/src/mp3.rs:602 — Minutes calculated as duration/60.0 gives total minutes, not remainder after hours. Same bug as flac.rs:166. Should be ((duration % 3600.0) / 60.0) as u32.
