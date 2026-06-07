---
id: id3-9mu
title: Refactor expand_file_args to use glob::Pattern::escape()
status: open
type: feature
priority: 3
tags:
- id3tools
- common
- refactor
created: 2026-06-06
updated: 2026-06-06
phase: ''
---

# Refactor expand_file_args to use glob::Pattern::escape()

The current exists_literally guard is a filesystem-layer proxy for a parsing-layer decision. glob 0.3.x ships Pattern::escape(s) which wraps metacharacters so the pattern matches only that literal string. Using it would eliminate the TOCTOU window, work correctly for dangling symlinks and permission-denied paths, and remove the silent-suppression edge case where a file literally named "*.mp3" causes glob expansion to be skipped. See code review findings on fix/brackets-in-filenames (id3-rwl).
