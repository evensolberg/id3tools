---
id: id3-6ot
title: 'common: Genre::from_str silently accepts typos as Other'
status: closed
type: task
priority: 3
tags:
- common
- medium
- usability
created: 2026-03-06
updated: 2026-03-07
closed_reason: ''
dependencies: []
description: common/src/genres.rs:614 — FromStr maps all unrecognized genre strings to Other silently. Typos like 'Rocck' are never caught. Consider returning Err for unknown genres. Also, matching is case-sensitive which is inconsistent.
---

# common: Genre::from_str silently accepts typos as Other

common/src/genres.rs:614 — FromStr maps all unrecognized genre strings to Other silently. Typos like 'Rocck' are never caught. Consider returning Err for unknown genres. Also, matching is case-sensitive which is inconsistent.
