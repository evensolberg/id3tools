---
id: id3-q2n
title: 'all crates: add forbid(unsafe_code) to all crate roots'
status: open
type: task
priority: 3
tags:
- all-crates
- low
- hardening
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: 'Only the common crate has #![forbid(unsafe_code)]. Consider adding it to all crate roots (id3tag, id3show, id3export, id3cli-gen) for defense in depth.'
---

# all crates: add forbid(unsafe_code) to all crate roots

Only the common crate has #![forbid(unsafe_code)]. Consider adding it to all crate roots (id3tag, id3show, id3export, id3cli-gen) for defense in depth.
