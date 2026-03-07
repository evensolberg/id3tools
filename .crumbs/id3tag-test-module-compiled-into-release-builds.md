---
id: id3-04f
title: 'id3tag: test module compiled into release builds'
status: open
type: task
priority: 3
tags:
- id3tag
- low
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: 'id3tag/src/formats/images/mod.rs:12 — mod tests; is not #[cfg(test)]-gated, so test code is compiled into release builds.'
---

# id3tag: test module compiled into release builds

id3tag/src/formats/images/mod.rs:12 — mod tests; is not #[cfg(test)]-gated, so test code is compiled into release builds.
