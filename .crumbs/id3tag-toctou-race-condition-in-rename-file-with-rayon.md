---
id: id3-vd2
title: 'id3tag: TOCTOU race condition in rename_file with rayon'
status: open
type: bug
priority: 2
tags:
- id3tag
- medium
- race-condition
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3tag/src/rename_file.rs:96-112 — Existence check and rename are non-atomic. With rayon parallelism, two files targeting the same name could collide and one overwrites the other.
---

# id3tag: TOCTOU race condition in rename_file with rayon

id3tag/src/rename_file.rs:96-112 — Existence check and rename are non-atomic. With rayon parallelism, two files targeting the same name could collide and one overwrites the other.
