---
id: id3-fho
title: 'common: split_val silently defaults on parse failures'
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
description: common/src/shared.rs:203-204 — split_val uses unwrap_or(1) on parse failures. 'abc/def' silently becomes (1,1). The function returns Result, so parse failures should be propagated.
---

# common: split_val silently defaults on parse failures

common/src/shared.rs:203-204 — split_val uses unwrap_or(1) on parse failures. 'abc/def' silently becomes (1,1). The function returns Result, so parse failures should be propagated.
