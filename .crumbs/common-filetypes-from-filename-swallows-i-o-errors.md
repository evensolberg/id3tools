---
id: id3-ix6
title: 'common: FileTypes::from_filename swallows I/O errors'
status: closed
type: bug
priority: 2
tags:
- common
- medium
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: common/src/file_types.rs:23-42 — from_filename swallows I/O errors (permission denied, file not found) and returns Unknown. Should return Result to distinguish real errors from unknown file types.
---

# common: FileTypes::from_filename swallows I/O errors

common/src/file_types.rs:23-42 — from_filename swallows I/O errors (permission denied, file not found) and returns Unknown. Should return Result to distinguish real errors from unknown file types.
