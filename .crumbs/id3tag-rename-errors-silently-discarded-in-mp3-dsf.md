---
id: id3-i0m
title: 'id3tag: rename errors silently discarded in mp3/dsf'
status: closed
type: bug
priority: 3
tags:
- id3tag
- low
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3tag/src/formats/mp3.rs:164-166 and dsf.rs:86 — rename_file errors are silently discarded and processed_ok is set to false with no error reported to user.
---

# id3tag: rename errors silently discarded in mp3/dsf

id3tag/src/formats/mp3.rs:164-166 and dsf.rs:86 — rename_file errors are silently discarded and processed_ok is set to false with no error reported to user.
