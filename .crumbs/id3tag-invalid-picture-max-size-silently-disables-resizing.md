---
id: id3-jc9
title: 'id3tag: invalid picture_max_size silently disables resizing'
status: open
type: bug
priority: 3
tags:
- id3tag
- medium
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3tag/src/default_values.rs:284 — Invalid picture_max_size (e.g., 'abc') silently becomes 0 via unwrap_or(0), which disables resizing entirely with no warning to the user.
---

# id3tag: invalid picture_max_size silently disables resizing

id3tag/src/default_values.rs:284 — Invalid picture_max_size (e.g., 'abc') silently becomes 0 via unwrap_or(0), which disables resizing entirely with no warning to the user.
