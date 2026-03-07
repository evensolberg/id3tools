---
id: id3-ptn
title: 'id3show: missing mp3.rs module prevents compilation'
status: closed
type: bug
priority: 1
tags:
- id3show
- critical
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3show/src/main.rs:10 declares 'mod mp3;' but no mp3.rs file exists locally. The file exists in the repository and can be retrieved from there.
---

# id3show: missing mp3.rs module prevents compilation

id3show/src/main.rs:10 declares 'mod mp3;' but no mp3.rs file exists locally. The file exists in the repository and can be retrieved from there.
