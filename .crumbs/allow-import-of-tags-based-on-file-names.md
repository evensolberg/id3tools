---
id: id3-q9a
title: Allow import of tags based on file names.
status: open
type: feature
priority: 2
tags: []
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: |
  If a file is called “01 Hello.flac” then “%tn %tt” would import as:

  - track number: 01
  - track title: Hello

  Likewise, “%ta - %tn - %tt” would get “Artist - 01 - Hello” as

  - track artist: Artist
  - track number: 01
  - track title: Hello

  This would use the same tag codes as the file rename functionality.
---

# Allow import of tags based on file names.

If a file is called “01 Hello.flac” then “%tn %tt” would import as:

- track number: 01
- track title: Hello

Likewise, “%ta - %tn - %tt” would get “Artist - 01 - Hello” as

- track artist: Artist
- track number: 01
- track title: Hello

This would use the same tag codes as the file rename functionality.
