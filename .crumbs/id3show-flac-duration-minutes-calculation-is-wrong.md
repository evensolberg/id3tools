---
id: id3-9p2
title: 'id3show: FLAC duration minutes calculation is wrong'
status: open
type: bug
priority: 2
tags:
- id3show
- high
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3show/src/flac.rs:166 — Minutes calculated as duration/60.0 gives total minutes, not remainder after hours. A 2-hour file shows 02:120:ss. Should be ((duration % 3600.0) / 60.0) as u32.
---

# id3show: FLAC duration minutes calculation is wrong

id3show/src/flac.rs:166 — Minutes calculated as duration/60.0 gives total minutes, not remainder after hours. A 2-hour file shows 02:120:ss. Should be ((duration % 3600.0) / 60.0) as u32.
