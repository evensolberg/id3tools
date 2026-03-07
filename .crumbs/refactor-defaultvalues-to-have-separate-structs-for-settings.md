---
id: id3-8ga
title: Refactor `DefaultValues` to have separate structs for settings and values, at the very least.
status: closed
type: task
priority: 1
tags: []
created: 2026-03-07
updated: 2026-03-07
closed_reason: ''
dependencies: []
---

# Refactor `DefaultValues` to have separate structs for settings and values, at the very least.

## Implementation

Split the 32-field `DefaultValues` into 3 nested sub-structs using `#[serde(flatten)]` for TOML backward compatibility:

- **`ExecutionConfig`** (5 fields): `detail_off`, `print_summary`, `stop_on_error`, `dry_run`, `single_thread`
- **`PictureConfig`** (6 fields): `picture_front`, `picture_back`, `picture_front_candidates`, `picture_back_candidates`, `picture_search_folders`, `picture_max_size` — plus getter methods `search_folders()`, `picture_front_candidates()`, `picture_back_candidates()`
- **`TagValues`** (20 fields): all album/track/disc/genre/composer/date/comments fields + `disc_count`, `track_count`

`rename_file` and `log_config_file` remain at the top level of `DefaultValues`.

Macro changes:
- Read-only macros (`tag!`, `pic!`, etc.): changed `$cfg:ident` → `$cfg:expr`, call sites pass sub-structs (`dv.tags`, `dv.pictures`)
- `check_flag!`: hardcoded `.execution` path into macro body, call sites unchanged
- `split!` in flac.rs: updated body to use `$cfg.tags.$field`

Files modified: `default_values.rs`, `tag_macros.rs`, `mod.rs`, `ape.rs`, `flac.rs`, `mp3.rs`, `mp4.rs`, `dsf.rs`, `rename_file.rs`, `images/mod.rs`, `images/paths.rs`, `main.rs`
