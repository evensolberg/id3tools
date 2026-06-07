---
id: id3-rwl
title: Square brackets in file names causes the file to be ignored
status: in_progress
type: bug
priority: 2
tags: []
created: 2026-06-06
updated: 2026-06-06
phase: ''
---

# Square brackets in file names causes the file to be ignored

id3tag (and presumably the other utilities) will ignore any files that has [ and ] in it.

[2026-06-06] Root cause: `expand_file_args` in `common/src/shared.rs` (line 28) triggers glob processing for any argument containing `[`. This causes literal filenames with square brackets (e.g. `Song [Live].mp3`) to be fed into `glob::glob()`, where `[Live]` is interpreted as a character class matching any single character from {L, i, v, e} rather than a literal bracket sequence. Because no file matches the mangled pattern, `matched` stays false, the file is silently dropped with only a `warn!` log, and the tool processes nothing. The condition `arg.contains('[')` should not be the sole trigger for glob expansion — `[` without `*` or `?` is commonly present in real-world audio file names.

[2026-06-06] Fix: in `expand_file_args`, add an `exists_literally` check — if `Path::new(arg).exists()` is true, push the arg directly and skip globbing. Only enter the glob branch when the arg looks like a pattern AND does not already exist on disk as a literal path. This preserves full glob functionality (`*.mp3`, `[0-9]*.mp3`) while correctly handling filenames containing `[` and `]`.

[start] 2026-06-06 20:43:39
