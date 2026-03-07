---
id: id3-8mv
title: 'id3tag: MP3 tag write failure silently discarded'
status: closed
type: bug
priority: 1
tags:
- id3tag
- high
- data-loss
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3tag/src/formats/mp3.rs:156-161 — write_to_path failure is silently discarded. User believes tags were saved when they were not. Log the error and propagate it.
---

# id3tag: MP3 tag write failure silently discarded

id3tag/src/formats/mp3.rs:156-161 — write_to_path failure is silently discarded. User believes tags were saved when they were not. Log the error and propagate it.
