---
id: id3-syj
title: 'common: tests fail due to missing sample audio files in testdata/'
status: closed
type: bug
priority: 2
tags:
- common
- tests
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: 'Tests reference ../testdata/ which resolves correctly from the common crate. Audio files ARE on disk but gitignored (**.flac, **.mp3, etc). However, sample.flac and sample.mp3 are missing from disk — only sample.ape, sample.dsf, sample.m4a, and sample.mp4 exist. The test_from_filename test also expects sample.mp4 to return M4A but the test asserts against FileTypes::Flac at line 73, so it fails on the Dsf assertion (line 73 returns Unknown instead of Dsf — likely the infer crate version change). Need to: (1) obtain or generate sample.flac and sample.mp3 files, (2) verify test assertions match current infer crate behavior.'
---

# common: tests fail due to missing sample audio files in testdata/

Tests reference ../testdata/ which resolves correctly from the common crate. Audio files ARE on disk but gitignored (**.flac, **.mp3, etc). However, sample.flac and sample.mp3 are missing from disk — only sample.ape, sample.dsf, sample.m4a, and sample.mp4 exist. The test_from_filename test also expects sample.mp4 to return M4A but the test asserts against FileTypes::Flac at line 73, so it fails on the Dsf assertion (line 73 returns Unknown instead of Dsf — likely the infer crate version change). Need to: (1) obtain or generate sample.flac and sample.mp3 files, (2) verify test assertions match current infer crate behavior.
