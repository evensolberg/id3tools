---
id: id3-chf
title: Replace `Box<dyn Error>` with `anyhow` for error handling
status: open
type: task
priority: 1
tags: []
created: 2026-03-07
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Replace `Box<dyn Error>` with `anyhow` for error handling

## Rationale

The codebase uses `Box<dyn Error>` everywhere with `format!("...").into()` for error construction. Using `anyhow` is a cleaner approach than hand-rolling a custom error struct:

- Near drop-in replacement for `Box<dyn Error>`
- `.context()` / `.with_context()` for wrapping errors with meaningful messages, replacing `format!("...").into()`
- `anyhow::bail!()` macro for early returns
- Better backtraces in debug builds

## Scope

- Add `anyhow` to workspace dependencies
- Replace `Result<T, Box<dyn Error>>` with `anyhow::Result<T>` across all crates (`common`, `id3tag`, `id3show`, `id3export`)
- Replace `format!("...").into()` patterns with `.context()`
- Replace `return Err(format!(...).into())` with `bail!()`
