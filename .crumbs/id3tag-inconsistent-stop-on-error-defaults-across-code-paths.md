---
id: id3-cye
title: 'id3tag: inconsistent stop_on_error defaults across code paths'
status: open
type: bug
priority: 3
tags:
- id3tag
- medium
- consistency
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3tag/src/formats/mod.rs:84 uses unwrap_or(true) for stop_on_error, while mp3.rs:42 uses unwrap_or(false). Same config field has different defaults depending on code path.
---

# id3tag: inconsistent stop_on_error defaults across code paths

id3tag/src/formats/mod.rs:84 uses unwrap_or(true) for stop_on_error, while mp3.rs:42 uses unwrap_or(false). Same config field has different defaults depending on code path.
