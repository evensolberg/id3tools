---
id: id3-sd4
title: 'id3export: panicking unwrap on fs::metadata in tracks.rs'
status: closed
type: bug
priority: 1
tags:
- id3export
- critical
- panic
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3export/src/tracks.rs:237 — metadata.unwrap() on fs::metadata() will panic if file is inaccessible or path is empty. Replace with ? or match.
---

# id3export: panicking unwrap on fs::metadata in tracks.rs

id3export/src/tracks.rs:237 — metadata.unwrap() on fs::metadata() will panic if file is inaccessible or path is empty. Replace with ? or match.
