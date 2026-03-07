---
id: id3-yeq
title: 'id3tag: JPEG encoding failure silently produces corrupt image'
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
description: id3tag/src/formats/images/mod.rs:113-116 — JPEG write_to() failures silently produce empty/corrupt image data via unwrap_or_default(). This corrupt data gets embedded in tags. Propagate with ?.
---

# id3tag: JPEG encoding failure silently produces corrupt image

id3tag/src/formats/images/mod.rs:113-116 — JPEG write_to() failures silently produce empty/corrupt image data via unwrap_or_default(). This corrupt data gets embedded in tags. Propagate with ?.
