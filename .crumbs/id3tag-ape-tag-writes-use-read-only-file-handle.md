---
id: id3-wag
title: 'id3tag: APE tag writes use read-only file handle'
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
description: id3tag/src/formats/ape.rs:56 — File::open() opens read-only for a write operation. APE tag writes silently fail every time. Use File::options().read(true).write(true).open().
---

# id3tag: APE tag writes use read-only file handle

id3tag/src/formats/ape.rs:56 — File::open() opens read-only for a write operation. APE tag writes silently fail every time. Use File::options().read(true).write(true).open().
