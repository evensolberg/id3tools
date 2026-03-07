---
id: id3-3kz
title: 'id3export: division by zero in duration_from_samples'
status: open
type: bug
priority: 2
tags:
- id3export
- high
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3export/src/tracks.rs:578-579 — duration_from_samples divides by sample_rate without zero-check. Produces Infinity/NaN cast to u64. Add a guard for sample_rate == 0.
---

# id3export: division by zero in duration_from_samples

id3export/src/tracks.rs:578-579 — duration_from_samples divides by sample_rate without zero-check. Produces Infinity/NaN cast to u64. Add a guard for sample_rate == 0.
