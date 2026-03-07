---
id: id3-wik
title: 'id3tag: clean_filename missing backslash/null sanitization'
status: closed
type: bug
priority: 3
tags:
- id3tag
- medium
- security
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: id3tag/src/rename_file.rs:78 — clean_filename does not sanitize backslash (Windows path separator) or null bytes from tag values used in filenames.
---

# id3tag: clean_filename missing backslash/null sanitization

id3tag/src/rename_file.rs:78 — clean_filename does not sanitize backslash (Windows path separator) or null bytes from tag values used in filenames.
