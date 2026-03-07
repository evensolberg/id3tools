---
id: id3-b36
title: 'id3show/id3cli-gen: log::error without initialized logger'
status: open
type: bug
priority: 3
tags:
- id3show
- id3cli-gen
- low
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3show/src/main.rs:85 and id3cli-gen/src/main.rs:29 — log::error! is called but logger may not be initialized on early failure. Errors are silently dropped. Use eprintln! as fallback.
---

# id3show/id3cli-gen: log::error without initialized logger

id3show/src/main.rs:85 and id3cli-gen/src/main.rs:29 — log::error! is called but logger may not be initialized on early failure. Errors are silently dropped. Use eprintln! as fallback.
