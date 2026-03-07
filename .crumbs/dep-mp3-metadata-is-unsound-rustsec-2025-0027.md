---
id: id3-lg1
title: 'dep: mp3-metadata is unsound (RUSTSEC-2025-0027)'
status: closed
type: bug
priority: 2
tags:
- dependency
- security
- id3show
- id3export
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: 'mp3-metadata 0.3.4 has RUSTSEC-2025-0027: panic due to missing bounds checking on malformed MP3 files. Used by id3show and id3export. Consider a fork/patch or alternative crate.'
---

# dep: mp3-metadata is unsound (RUSTSEC-2025-0027)

mp3-metadata 0.3.4 has RUSTSEC-2025-0027: panic due to missing bounds checking on malformed MP3 files. Used by id3show and id3export. Consider a fork/patch or alternative crate.
