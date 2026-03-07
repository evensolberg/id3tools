---
id: id3-3si
title: 'common: panicking unwrap in count_files iterator'
status: closed
type: bug
priority: 1
tags:
- common
- critical
- panic
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: common/src/shared.rs:244 — .map(Result::unwrap) in count_files iterator will panic on permission errors or race conditions. Replace with .filter_map(Result::ok).
---

# common: panicking unwrap in count_files iterator

common/src/shared.rs:244 — .map(Result::unwrap) in count_files iterator will panic on permission errors or race conditions. Replace with .filter_map(Result::ok).
