---
id: id3-pc0
title: 'id3export: clippy warnings in stats.rs'
status: closed
type: task
priority: 3
tags:
- id3export
- clippy
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3export/src/stats.rs:8 — Use .or_default() instead of .or_insert(Statistics::new()). Line 13 — Iterate with .values_mut() instead of for (_, stats).
---

# id3export: clippy warnings in stats.rs

id3export/src/stats.rs:8 — Use .or_default() instead of .or_insert(Statistics::new()). Line 13 — Iterate with .values_mut() instead of for (_, stats).
