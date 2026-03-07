---
id: id3-2ig
title: 'common: hardcoded fallback log file path'
status: open
type: task
priority: 3
tags:
- common
- low
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: common/src/log.rs:39 — Fallback log file is hardcoded as ./id3tag.log relative to CWD. Consider using XDG_STATE_HOME or a user-configurable path.
---

# common: hardcoded fallback log file path

common/src/log.rs:39 — Fallback log file is hardcoded as ./id3tag.log relative to CWD. Consider using XDG_STATE_HOME or a user-configurable path.
