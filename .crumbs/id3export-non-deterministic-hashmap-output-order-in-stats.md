---
id: id3-5tj
title: 'id3export: non-deterministic HashMap output order in stats'
status: open
type: task
priority: 3
tags:
- id3export
- low
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3export/src/stats.rs:22,47 — HashMap iteration order varies between runs. Use BTreeMap for stable CSV/summary output.
---

# id3export: non-deterministic HashMap output order in stats

id3export/src/stats.rs:22,47 — HashMap iteration order varies between runs. Use BTreeMap for stable CSV/summary output.
