---
id: id3-utp
title: Check if there is only one music file and one or more `.CUE` or `.cue` file(s) in the same directory. Skip if this is the case.
status: open
type: feature
priority: 2
tags: []
created: 2026-03-07
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Check if there is only one music file and one or more `.CUE` or `.cue` file(s) in the same directory. Skip if this is the case.

This applies to id3tag. In cases like this, the album is ripped to one big FLAC file with the CUE file providing information about tracks. Trying to tag this FLAC just screws things up, so it should be skipped until the tracks have been extracted into individual files. This should apply even if there are multiple FLACs in the folder - a FLAC with the same filename as the .cue file should be skipped (unless explicitly forced).
