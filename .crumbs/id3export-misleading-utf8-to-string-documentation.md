---
id: id3-8o1
title: 'id3export: misleading utf8_to_string documentation'
status: closed
type: task
priority: 3
tags:
- id3export
- low
- docs
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3export/src/tracks.rs:594-614 — utf8_to_string docstring and example are misleading. The function converts bytes to hex representation but docs describe the reverse operation.
---

# id3export: misleading utf8_to_string documentation

id3export/src/tracks.rs:594-614 — utf8_to_string docstring and example are misleading. The function converts bytes to hex representation but docs describe the reverse operation.
