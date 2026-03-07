---
id: id3-vde
title: 'id3export: file_count should be unsigned type'
status: closed
type: task
priority: 3
tags:
- id3export
- low
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3export/src/stats.rs:62 — file_count is i32 but should be u64 or usize since file counts are never negative.
---

# id3export: file_count should be unsigned type

id3export/src/stats.rs:62 — file_count is i32 but should be u64 or usize since file counts are never negative.
