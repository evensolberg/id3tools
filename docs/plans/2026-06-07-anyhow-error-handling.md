---
tags:
  - rust
  - error-handling
  - refactoring
  - anyhow
aliases: []
doc_id: plan-id3-chf
doc_name: anyhow-migration
crumb_id: id3-chf
document_title: Replace Box<dyn Error> with anyhow for error handling
created_date: 2026-06-07
synopsis: >
  Implementation plan for migrating all five crates in the id3tools workspace
  from Box<dyn Error> to anyhow::Result, bail!(), and .context() / .with_context().
status: active
type: plan
revision: 5
review_date: 2026-06-07
reviewed_by: []
completed_date:
comments:
revision_history:
  - revision: 1
    date: 2026-06-07
    author: Claude
    notes: Initial plan
  - revision: 2
    date: 2026-06-07
    author: Claude
    notes: >
      Fixed directory() return type (PathBuf not String);
      fixed get_file_type() error count (2 not 3);
      added Setup section (branch + crumbs start);
      added git-mit es to all commit steps;
      added PR creation and code review loop to Task 7;
      added Pattern C match-arm note;
      expanded flac.rs step to show all 3 error constructions;
      clarified workspace dep insertion point.
  - revision: 3
    date: 2026-06-07
    author: Claude
    notes: >
      Fixed mp3.rs Step 5: before snippet was missing `return` and comment
      incorrectly said "expression position — no return" (actual code is a
      statement in a `_ =>` arm with return);
      fixed dsf.rs Step 9: same wrong "expression position" comment;
      fixed images/mod.rs Step 10: before snippet was missing `return` on
      second error, removed incorrect "expression position" note, clarified
      that Pattern A applies to all 4 functions not just read_cover;
      fixed mp3.rs Step 5 description: not all 8 occurrences are in
      Err(err) => arms — one is in a _ => arm.
  - revision: 4
    date: 2026-06-07
    author: Claude
    notes: >
      Fixed Task 6 Step 1: id3cli-gen/src/main.rs has no `use std::error::Error;`
      import — it uses the fully-qualified `Box<dyn std::error::Error>` inline;
      removed incorrect "Remove use std::error::Error;" instruction, corrected
      Before snippet to show the fully-qualified form;
      fixed Task 4 Step 2 (id3show/src/mp3.rs): explicitly named both functions
      requiring Pattern A (show_metadata line 65 and private helper open_mp3
      line 454); clarified "expression form" label — the Err(...) is the last
      expression in a match arm, not the function body directly.
  - revision: 5
    date: 2026-06-07
    author: Claude
    notes: >
      Corrected Scope section counts: format!().into() is 34 (was wrong 20),
      "...".into() is 11 literal + 1 variable = 12 (was wrong 26), total 46
      unchanged;
      fixed Task 3 Step 4 wording "all three replacements" → "all four" and
      enumerated all 5 Pattern A function signatures plus doc-comment note;
      fixed Task 3 Step 6 to note doc comment at line ~55 in id3tag/flac.rs;
      fixed Task 3 Step 8: clarified 3 Pattern A function signatures in ape.rs
      (not 2), added note to preserve #[allow(clippy::unnecessary_wraps)] on
      rename_file;
      fixed Task 4 Step 4 to note doc comment in id3show/ape.rs;
      updated Task 7 Step 6 PR body to use HEREDOC instead of escaped string.
---

# Replace `Box<dyn Error>` with `anyhow` — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all `Box<dyn Error>` return types and ad-hoc `format!(...).into()` / `"...".into()` error construction across five Rust crates with `anyhow::Result<T>`, `bail!()`, `.context()`, and `.with_context()`.

**Architecture:** Three mechanics applied uniformly across the workspace: (1) add `anyhow` to workspace dependencies and each crate's `[dependencies]`; (2) per file, swap the return type, add the correct `use anyhow::{...}` import, remove the now-redundant `use std::error::Error`, and replace error construction patterns; (3) upgrade `main()` error display from `err.to_string()` to `{err:#}` for full cause-chain output. No new files are created; no custom error types are introduced; no application behaviour changes except richer error messages.

**Tech Stack:** Rust 2021 edition, `anyhow` 1.x, Cargo workspace

---

## Scope

- **60 function signatures** across 28 files in 5 crates
- **0 custom error types** — pure type-alias swap
- **34 `format!(...).into()` constructions** → `bail!()`
- **11 `"...".into()` literal constructions** → `bail!()`
- **1 `Err(msg.into())` variable construction** → `bail!("{msg}")`
- **3 `.map_err(|e| format!(...))` chains** → `.with_context(|| format!(...))`
- **4 `main()` error display blocks** upgraded to `{err:#}`

---

## File Map

All changes are modifications to existing files. No files are created or deleted.

| File | Changes |
|------|---------|
| `Cargo.toml` | Add `anyhow = "1"` to `[workspace.dependencies]` |
| `common/Cargo.toml` | Add `anyhow.workspace = true` |
| `common/src/log.rs` | Pattern A + B (import only `Result`) |
| `common/src/shared.rs` | Pattern A + B + C + D (all four patterns) |
| `id3tag/Cargo.toml` | Add `anyhow.workspace = true` |
| `id3tag/src/main.rs` | Pattern A + B + C, display upgrade |
| `id3tag/src/default_values.rs` | Pattern A + B + C + D |
| `id3tag/src/rename_file.rs` | Pattern A + B + C |
| `id3tag/src/formats/mod.rs` | Pattern A + B + C |
| `id3tag/src/formats/mp3.rs` | Pattern A + B + C |
| `id3tag/src/formats/flac.rs` | Pattern A + B + C |
| `id3tag/src/formats/mp4.rs` | Pattern A + B + C |
| `id3tag/src/formats/ape.rs` | Pattern A + B + C |
| `id3tag/src/formats/dsf.rs` | Pattern A + B + C |
| `id3tag/src/formats/images/mod.rs` | Pattern A + B + C |
| `id3tag/src/formats/images/paths.rs` | Pattern A + B + C |
| `id3show/Cargo.toml` | Add `anyhow.workspace = true` |
| `id3show/src/main.rs` | Pattern A + B, display upgrade |
| `id3show/src/mp3.rs` | Pattern A + B + C |
| `id3show/src/flac.rs` | Pattern A + B + C |
| `id3show/src/ape.rs` | Pattern A + B |
| `id3show/src/mp4.rs` | Pattern A + B |
| `id3show/src/dsf.rs` | Pattern A + B + C |
| `id3export/Cargo.toml` | Add `anyhow.workspace = true` |
| `id3export/src/main.rs` | Pattern A + B, display upgrade |
| `id3export/src/tracks.rs` | Pattern A + B + C |
| `id3cli-gen/Cargo.toml` | Add `anyhow.workspace = true` |
| `id3cli-gen/src/main.rs` | Pattern A + B, display upgrade |

---

## Patterns Reference

These four patterns are applied repeatedly. Individual task steps reference them by letter.

### Pattern A — Return type

```rust
// Before
fn foo() -> Result<T, Box<dyn Error>> {
// After  (with use anyhow::Result in scope)
fn foo() -> Result<T> {
```

### Pattern B — Import swap

Remove the `std::error::Error` import (or restructure the combined `use` if needed), and add an anyhow import using only the items the file needs:

```rust
// Before (various forms)
use std::error::Error;
use std::{error::Error, time::Instant};

// After — add one of these based on what the file uses
use anyhow::Result;                        // file uses only ?-propagation
use anyhow::{bail, Result};               // file also uses bail!()
use anyhow::{bail, Context, Result};      // file also uses .context() / .with_context()
```

**Rule for choosing:** import `bail` if the file calls `bail!()`, import `Context` if the file calls `.context()` or `.with_context()`.

### Pattern C — Error construction → bail!

```rust
// Before — two forms
return Err(format!("Unable to save {m_file}").into());
return Err("File type not supported".into());

// After — bail! constructs the error AND returns, so drop the `return Err(...)` wrapper
bail!("Unable to save {m_file}");
bail!("File type not supported");
```

`bail!()` expands to `return Err(anyhow!(...))`. It works in statement position (`return Err(...)`) and expression position (last expression in a block without semicolon) because the never type (`!`) coerces to any type.

**Pattern C in match arms (`Err(err) =>`):** Several files construct a new error string from an existing error variable inside a match arm. The idiomatic anyhow approach would be `return Err(err).context(...)`, but this plan keeps `bail!("{err}")` for consistency — the error detail stays visible in the message and the scope is a pure text-substitution refactor. No chaining is lost because the original code also embedded the error in the string rather than chaining it.

```rust
// Before — inside Err(err) => arm
return Err(format!("Unable to rename {filename} to {npl}. Error: {err}").into());

// After — bail! with the embedded error; identical information, no chain created or lost
bail!("Unable to rename {filename} to {npl}. Error: {err}");
```

### Pattern D — map_err → with_context

```rust
// Before — error message embeds {e} in the string
.map_err(|e| format!("Unable to parse count '{}': {e}", val))?

// After — drop {e} from the message; anyhow preserves the original error
//         as a cause in the chain, shown by {:#} in main()
.with_context(|| format!("Unable to parse count '{}'", val))?
```

### Display upgrade (main() only)

```rust
// Before
Err(err) => {
    let msg = err.to_string().replace('\"', "");
    log::error!("{msg}");
    eprintln!("Error: {msg}");
    1
}

// After — {err:#} prints the full cause chain; no intermediate variable needed
Err(err) => {
    log::error!("{err:#}");
    eprintln!("Error: {err:#}");
    1
}
```

---

## Setup

Before starting any task, run these two commands once:

```bash
crumbs start id3-chf
git switch -c refactor/anyhow-migration
```

The branch name keeps all seven task-commits together and makes the PR diff easy to review.

---

## Task 1: Wire anyhow into all Cargo.toml files

**Files:**
- Modify: `Cargo.toml`
- Modify: `common/Cargo.toml`
- Modify: `id3tag/Cargo.toml`
- Modify: `id3show/Cargo.toml`
- Modify: `id3export/Cargo.toml`
- Modify: `id3cli-gen/Cargo.toml`

- [ ] **Step 1: Add anyhow to workspace dependencies**

In `Cargo.toml`, insert one line at the top of `[workspace.dependencies]` (the existing entries are not alphabetically sorted; placing `anyhow` first keeps it easy to find):

```toml
[workspace.dependencies]
anyhow = "1"
ape = "0"
# ... rest unchanged
```

- [ ] **Step 2: Add anyhow to each crate's [dependencies]**

In each of the five crates' `Cargo.toml` files, add one line to `[dependencies]` (alphabetical position doesn't matter, but near the top is conventional):

```toml
anyhow.workspace = true
```

This applies identically to `common/Cargo.toml`, `id3tag/Cargo.toml`, `id3show/Cargo.toml`, `id3export/Cargo.toml`, and `id3cli-gen/Cargo.toml`.

- [ ] **Step 3: Verify it compiles**

```bash
cargo check --workspace
```

Expected: zero errors, zero warnings. Source files still use `Box<dyn Error>` — that's fine, `anyhow` is just now an available dep.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock common/Cargo.toml id3tag/Cargo.toml id3show/Cargo.toml id3export/Cargo.toml id3cli-gen/Cargo.toml
git-mit es
git commit -m "chore(deps): add anyhow 1.x to workspace dependencies

Prereq for crumb id3-chf. No source changes yet.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 2: Migrate common crate

**Files:**
- Modify: `common/src/log.rs`
- Modify: `common/src/shared.rs`

### common/src/log.rs

`log.rs` has one function (`build_logger`) that uses `Box<dyn Error>` and relies entirely on `?` propagation — no custom error construction. Only Patterns A and B (import `Result` only) are needed.

- [ ] **Step 1: Update log.rs imports**

Line 14 currently reads:
```rust
use std::error::Error;
```

Remove it entirely. Add after the existing `use log4rs::{...}` block:

```rust
use anyhow::Result;
```

- [ ] **Step 2: Update log.rs function signature (line 22)**

```rust
// Before
pub fn build_logger(config_filename: &str) -> Result<(), Box<dyn Error>> {
// After
pub fn build_logger(config_filename: &str) -> Result<()> {
```

No other changes needed — all errors propagate via `?`.

### common/src/shared.rs

`shared.rs` has five `Box<dyn Error>` functions and uses all four patterns (A–D). This is the highest-complexity file in `common`.

- [ ] **Step 3: Update shared.rs imports**

Line 6 currently reads:
```rust
use std::{error::Error, time::SystemTime};
```

Replace with (removing `error::Error`, keeping `time::SystemTime`):
```rust
use std::time::SystemTime;
```

After the existing `use infer::{MatcherType, Type};` line, add:
```rust
use anyhow::{bail, Context, Result};
```

- [ ] **Step 4: Update get_mime_type (line 131) — Pattern A + C**

```rust
// Before
pub fn get_mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    let Some(file_type) = infer::get_from_path(filename)? else {
        return Err("File type not supported".into());
    };
    Ok(file_type.mime_type().to_string())
}

// After
pub fn get_mime_type(filename: &str) -> Result<String> {
    let Some(file_type) = infer::get_from_path(filename)? else {
        bail!("File type not supported");
    };
    Ok(file_type.mime_type().to_string())
}
```

- [ ] **Step 5: Update get_file_type (line 157) — Pattern A + C**

Two `Err("...".into())` calls inside this function:

```rust
// Before
pub fn get_file_type(filename: &str) -> Result<FileTypes, Box<dyn Error>> {
    // ...
    let Some(file_type) = file_type else {
        return Err("File type not supported".into());
    };
    // ... later in the else branch:
        return Err("File type not supported".into());
    // ...
}

// After
pub fn get_file_type(filename: &str) -> Result<FileTypes> {
    // ...
    let Some(file_type) = file_type else {
        bail!("File type not supported");
    };
    // ... later in the else branch:
        bail!("File type not supported");
    // ...
}
```

- [ ] **Step 6: Update split_val (line 308) — Pattern A + C + D**

This function has one `"...".into()` return and two `map_err` chains:

```rust
// Before
pub fn split_val(value: &str) -> Result<(u16, u16), Box<dyn Error>> {
    // ...
    } else {
        return Err("Split pattern not found.".into());
    }
    let count = split_str[0]
        .trim()
        .parse::<u16>()
        .map_err(|e| format!("Unable to parse count '{}': {e}", split_str[0].trim()))?;
    let total = split_str[1]
        .trim()
        .parse::<u16>()
        .map_err(|e| format!("Unable to parse total '{}': {e}", split_str[1].trim()))?;
    Ok((count, total))
}

// After
pub fn split_val(value: &str) -> Result<(u16, u16)> {
    // ...
    } else {
        bail!("Split pattern not found.");
    }
    let count = split_str[0]
        .trim()
        .parse::<u16>()
        .with_context(|| format!("Unable to parse count '{}'", split_str[0].trim()))?;
    let total = split_str[1]
        .trim()
        .parse::<u16>()
        .with_context(|| format!("Unable to parse total '{}'", split_str[1].trim()))?;
    Ok((count, total))
}
```

- [ ] **Step 7: Update count_files (line 347) — Pattern A + C**

```rust
// Before
pub fn count_files(filename: &str) -> Result<String, Box<dyn Error>> {
    // ...
    if !dir.is_dir() {
        return Err(format!("Unable to get directory name from filename {filename}.").into());
    }
    // ...
}

// After
pub fn count_files(filename: &str) -> Result<String> {
    // ...
    if !dir.is_dir() {
        bail!("Unable to get directory name from filename {filename}.");
    }
    // ...
}
```

- [ ] **Step 8: Update directory (line 452) — Pattern A only**

This function only uses `?` propagation — no custom error construction:

```rust
// Before
pub fn directory(filename: &str) -> Result<std::path::PathBuf, Box<dyn Error>> {
// After
pub fn directory(filename: &str) -> Result<std::path::PathBuf> {
```

- [ ] **Step 9: Verify common compiles and all tests pass**

```bash
cargo test -p common
```

Expected: all ~13 tests pass (they cover `split_val`, glob expansion, Roman numerals, file splitting, etc.). Zero failures.

```bash
cargo clippy -p common -- -D warnings
```

Expected: zero warnings.

- [ ] **Step 10: Commit**

```bash
git add common/src/log.rs common/src/shared.rs
git-mit es
git commit -m "refactor(common): replace Box<dyn Error> with anyhow::Result

- log.rs: use anyhow::Result, remove std::error::Error import
- shared.rs: use anyhow::{bail, Context, Result}
  - bail!() replaces Err(\"...\".into()) and Err(format!(...).into())
  - .with_context() replaces .map_err(|e| format!(...)) in split_val
  - five functions updated: get_mime_type, get_file_type, split_val,
    count_files, directory

Part of crumb id3-chf.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 3: Migrate id3tag crate

**Files:**
- Modify: `id3tag/src/main.rs`
- Modify: `id3tag/src/default_values.rs`
- Modify: `id3tag/src/rename_file.rs`
- Modify: `id3tag/src/formats/mod.rs`
- Modify: `id3tag/src/formats/mp3.rs`
- Modify: `id3tag/src/formats/flac.rs`
- Modify: `id3tag/src/formats/mp4.rs`
- Modify: `id3tag/src/formats/ape.rs`
- Modify: `id3tag/src/formats/dsf.rs`
- Modify: `id3tag/src/formats/images/mod.rs`
- Modify: `id3tag/src/formats/images/paths.rs`

- [ ] **Step 1: Migrate id3tag/src/main.rs — Pattern A + B + C + display upgrade**

Line 12 currently reads:
```rust
use std::error::Error;
```
Remove it. Add after the existing `use clap::ArgMatches;` line:
```rust
use anyhow::{bail, Result};
```

Update `run()` signature (line 29):
```rust
// Before
fn run() -> Result<(), Box<dyn Error>> {
// After
fn run() -> Result<()> {
```

Replace the `return Err(format!(...).into())` around lines 39–41:
```rust
// Before
return Err(
    format!("File rename pattern {pattern} likely won't create unique files.").into(),
);
// After
bail!("File rename pattern {pattern} likely won't create unique files.");
```

Upgrade `main()` error display (lines 128–132):
```rust
// Before
Err(err) => {
    let msg = err.to_string().replace('\"', "");
    log::error!("{msg}");
    eprintln!("Error: {msg}");
    1
}
// After
Err(err) => {
    log::error!("{err:#}");
    eprintln!("Error: {err:#}");
    1
}
```

- [ ] **Step 2: Migrate id3tag/src/default_values.rs — Pattern A + B + C + D**

Add import (after existing `use` statements):
```rust
use anyhow::{bail, Context, Result};
```

Apply Pattern A to all three function signatures (`build_config`, `load_config`, and `check_for_file_rename`).

Key error replacements inside `load_config` (~line 244–308):

```rust
// Line 248 — Pattern D: map_err → with_context
.map_err(|err| format!("Config file {filename} not found. Error: {err}"))?
// becomes
.with_context(|| format!("Config file {filename} not found"))?

// Line 252 — Pattern C: format!().into() → bail!
return Err(format!("Unable to read the contents of {filename}").into());
// becomes
bail!("Unable to read the contents of {filename}");

// Line 307 — Pattern C: format!().into() → bail!
return Err(
    format!("File rename pattern {pat} likely won't create unique files.").into(),
);
// becomes
bail!("File rename pattern {pat} likely won't create unique files.");
```

- [ ] **Step 3: Migrate id3tag/src/rename_file.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A. There are **2** `Err(...).into()` error constructions to replace.

First — literal string error (~line 36):

```rust
// Before
return Err("No filename pattern presented. Unable to continue.".into());
// After
bail!("No filename pattern presented. Unable to continue.");
```

Second — `format!().into()` error (~line 113, inside `Err(err) =>` match arm):

```rust
// Before
return Err(
    format!("Unable to rename {filename} to {npl}. Error: {err}").into(),
);
// After
bail!("Unable to rename {filename} to {npl}. Error: {err}");
```

- [ ] **Step 4: Migrate id3tag/src/formats/mod.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to all 5 function signatures: `process`, `get_tag_details`, `genre_name`, `disc_number`, `disc_count`. Note that `disc_number` and `disc_count` use only `?` propagation — just the return type changes, no error constructions to replace.

There are 4 Pattern C replacements:

```rust
// Unknown file type (inside FileTypes::Unknown => arm, no trailing semicolon)
return Err(format!("{filename} is unknown file type.").into())
// becomes
bail!("{filename} is unknown file type.");

// Process error (inside Err(err) => match arm)
return Err(format!("Unable to process {filename}. Error: {err}").into());
// becomes
bail!("Unable to process {filename}. Error: {err}");

// Tag parse error (inside Err(err) => match arm)
return Err(format!("Unable to parse tags for {filename}. Error: {err}").into());
// becomes
bail!("Unable to parse tags for {filename}. Error: {err}");

// Genre guard in genre_name() (~line 219)
return Err("Incorrect value supplied. Must be 0-191.".into());
// becomes
bail!("Incorrect value supplied. Must be 0-191.");
```

**Note on doc comments:** Line ~40 has `/// - \`Box<dyn Error>\` if we run into problems` in the `process_file` doc block. Lines ~501 and ~508 contain `Box<dyn Error>` in a doc comment and a doc example for `disc_count`. Update these to reference `anyhow::Error` or remove the explicit return-type annotation from the doc when applying Pattern B.

- [ ] **Step 5: Migrate id3tag/src/formats/mp3.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A. All 8 occurrences are multi-line `Err(format!(...).into())` constructions inside match arms (7 in `Err(err) =>` arms, 1 in a `_ =>` arm). Representative replacements:

```rust
// Cover art errors (lines ~44–62)
return Err(format!("Unable to set front cover for {filename}. Error: {err}").into());
// becomes
bail!("Unable to set front cover for {filename}. Error: {err}");

return Err(format!("Unable to set back cover for {filename}. Error: {err}").into());
// becomes
bail!("Unable to set back cover for {filename}. Error: {err}");

// Disc / track number errors (lines ~78–139, same shape)
return Err(format!("Unable to set disc number to {value}. Error: {err}").into());
// becomes
bail!("Unable to set disc number to {value}. Error: {err}");
// (identical pattern for "total discs", "track number", "total tracks")

// Tag unwrap error (~line 280, inside `_ =>` match arm)
return Err(format!(
    "Unknown tag {tag_name} encountered when unwrapping disc/track information."
)
.into())
// becomes
bail!("Unknown tag {tag_name} encountered when unwrapping disc/track information.");

// Rename error (~line 301)
return Err(format!(
    "Unable to rename {filename} with tags \"{pattern}\". Error: {err}"
)
.into());
// becomes
bail!("Unable to rename {filename} with tags \"{pattern}\". Error: {err}");
```

Apply the same `bail!` conversion to all 8 occurrences in this file.

- [ ] **Step 6: Migrate id3tag/src/formats/flac.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to all 3 function signatures (`process`, `set_picture`, `rename_file`). There is also a fourth `Box<dyn Error>` mention — a doc comment at line ~55 (`/// \`Result<(), Box<dyn Error>>\``) — update it to `anyhow::Error` when applying Pattern B. All 3 error constructions are `format!().into()`:

```rust
// ~lines 107–110: inside Err(err) => arm for set_picture call
return Err(format!(
    "Unable to set {cover_type:?} to {v}. Error message: {err}"
).into());
// becomes
bail!("Unable to set {cover_type:?} to {v}. Error message: {err}");

// ~line 132: standalone guard after tags.save() failure
return Err(format!("Unable to save {m_file}").into());
// becomes
bail!("Unable to save {m_file}");

// ~lines 196–199: inside Err(err) => arm for rename_file call
return Err(format!(
    "Unable to rename {filename} with tags \"{pattern}\". Error: {err}"
).into());
// becomes
bail!("Unable to rename {filename} with tags \"{pattern}\". Error: {err}");
```

- [ ] **Step 7: Migrate id3tag/src/formats/mp4.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A. Four error replacements (three single-line, one multi-line):

```rust
// ~line 65
return Err(format!("Unknown key: {key}").into());
// becomes
bail!("Unknown key: {key}");

// ~line 79
return Err(format!("Unable to save tags to {filename}. Error: {err}").into());
// becomes
bail!("Unable to save tags to {filename}. Error: {err}");

// ~line 92
return Err(format!("Unable to rename {filename}. Error: {err}").into());
// becomes
bail!("Unable to rename {filename}. Error: {err}");

// ~lines 139–142 (multi-line, inside Err(err) => / stop_on_error branch)
return Err(format!(
    "Unable to rename {filename} with tags \"{pattern}\". Error: {err}"
)
.into());
// becomes
bail!("Unable to rename {filename} with tags \"{pattern}\". Error: {err}");
```

- [ ] **Step 8: Migrate id3tag/src/formats/ape.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to all 3 function signatures: `process`, `set_picture`, `rename_file`. The `rename_file` function already carries `#[allow(clippy::unnecessary_wraps)]` — **preserve this attribute**; the function still returns only `Ok(())` after migration so the suppression remains valid.

The 2 Pattern C replacements are both multi-line `format!().into()` constructions inside `Err(err) =>` match arms:

```rust
// Cover-art set error (~lines 42–45)
return Err(format!(
    "Unable to set {ape_key} to {value}. Error: {err}"
).into());
// becomes
bail!("Unable to set {ape_key} to {value}. Error: {err}");

// Generic tag set error (~lines 64–67)
return Err(format!(
    "Unable to set {key} to {value}. Error message: {err}"
).into());
// becomes
bail!("Unable to set {key} to {value}. Error message: {err}");
```

Apply to all 2 occurrences.

- [ ] **Step 9: Migrate id3tag/src/formats/dsf.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A. All 3 occurrences are multi-line `format!().into()` constructions:

```rust
// Tag unwrap error (~lines 154–157, inside `_ =>` match arm)
return Err(format!(
    "Unknown tag {tag_name} encountered when unwrapping disc/track information."
)
.into())
// becomes
bail!("Unknown tag {tag_name} encountered when unwrapping disc/track information.");

// Rename error (~lines 175–178, inside Err(err) => match arm)
return Err(format!(
    "Unable to rename {filename} with tags \"{pattern}\". Error: {err}"
).into());
// becomes
bail!("Unable to rename {filename} with tags \"{pattern}\". Error: {err}");

// Number-parse error in to_number() (~line 209, inside Err(err) => match arm)
return Err(format!("Unable to set {item} to {value}. Error: {err}").into());
// becomes
bail!("Unable to set {item} to {value}. Error: {err}");
```

- [ ] **Step 10: Migrate id3tag/src/formats/images/mod.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to all 4 functions (`get_cover_filenames`, `find_cover`, `read_cover`, `to_jpeg`). Only `read_cover` has custom error constructions — 2 Pattern C replacements:

```rust
return Err(format!("Image {cover_file} is outside the expected ratio.").into());
// becomes
bail!("Image {cover_file} is outside the expected ratio.");

return Err(
    format!("Image {cover_file} is too small. (Less than 1/2 the cover size.)").into(),
);
// becomes
bail!("Image {cover_file} is too small. (Less than 1/2 the cover size.)");
```

Apply to all 2 Pattern C occurrences.

- [ ] **Step 11: Migrate id3tag/src/formats/images/paths.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to `find_first_image` — the only function in this file with a `Box<dyn Error>` return type. (`gather_cover_candidates` returns a plain `Vec<String>` and is unchanged.)

Replace the one `format!().into()` error (~line 102):

```rust
return Err(format!("Music file {m_file} does not appear to exist.").into());
// becomes
bail!("Music file {m_file} does not appear to exist.");
```

**Note on doc comments:** The file has three `Box<dyn Error>` occurrences in total, but two are in doc comments. The doc comment on `gather_cover_candidates` still references a `Result<Vec<String>, Box<dyn Error>>` return type that does not match the actual `Vec<String>` signature — remove or correct that stale comment when applying Pattern B.

- [ ] **Step 12: Verify id3tag compiles and tests pass**

```bash
cargo test -p id3tag
```

Expected: all existing tests pass.

```bash
cargo clippy -p id3tag -- -D warnings
```

Expected: zero warnings.

- [ ] **Step 13: Commit**

```bash
git add id3tag/
git-mit es
git commit -m "refactor(id3tag): replace Box<dyn Error> with anyhow::Result

- 11 source files updated across main, default_values, rename_file,
  and all format modules (mp3, flac, mp4, ape, dsf, images)
- bail!() replaces Err(\"...\".into()) and Err(format!(...).into())
- .with_context() replaces .map_err() in default_values
- main() now displays full error chain via {err:#}

Part of crumb id3-chf.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 4: Migrate id3show crate

**Files:**
- Modify: `id3show/src/main.rs`
- Modify: `id3show/src/mp3.rs`
- Modify: `id3show/src/flac.rs`
- Modify: `id3show/src/ape.rs`
- Modify: `id3show/src/mp4.rs`
- Modify: `id3show/src/dsf.rs`

Pattern D is not needed in this crate — there are no `map_err` usages.

- [ ] **Step 1: Migrate id3show/src/main.rs — Pattern A + B + display upgrade**

Line 3 currently reads:
```rust
use std::{error::Error, time::Instant};
```
Replace with (removing only `error::Error`):
```rust
use std::time::Instant;
```

Add after the `mod` declarations block:
```rust
use anyhow::Result;
```

Apply Pattern A to `run()` (line 17):
```rust
// Before
fn run() -> Result<(), Box<dyn Error>> {
// After
fn run() -> Result<()> {
```

Upgrade `main()` error display (lines 87–91):
```rust
// Before
Err(err) => {
    let msg = err.to_string().replace('\"', "");
    log::error!("{msg}");
    eprintln!("Error: {msg}");
    1
}
// After
Err(err) => {
    log::error!("{err:#}");
    eprintln!("Error: {err:#}");
    1
}
```

- [ ] **Step 2: Migrate id3show/src/mp3.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to **both** functions with `Box<dyn Error>` return types:

1. `show_metadata` (line 65) — public entry point
2. `open_mp3` (line 454) — private helper (`fn open_mp3(filename: &str) -> Result<MP3Metadata, Box<dyn Error>>`)

Two error replacements:

```rust
// Statement form — inside `_ =>` match arm in show_metadata:
return Err(format!("Unknown content type in file {filename}").into());
// becomes
bail!("Unknown content type in file {filename}");

// Expression form — last expression in Err(e) => match arm in open_mp3
// (no `return` keyword; bail! supplies it):
Err(format!("Unable to open {filename} for to read stream info. Error: {e}").into())
// becomes
bail!("Unable to open {filename} for to read stream info. Error: {e}")
```

- [ ] **Step 3: Migrate id3show/src/flac.rs — Pattern A + B + C**

`flac.rs` has one custom error construction in addition to `?` propagation.

- Remove `use std::error::Error;`
- Add `use anyhow::{bail, Result};`
- Apply Pattern A to all 3 function signatures (`calc_duration_seconds`, `calc_duration_string`, `show_metadata`; the fourth `Box<dyn Error>` mention in this file is a doc comment)

Replace the one `"...".into()` error in `calc_duration_seconds` (~line 156):

```rust
return Err("Sample rate is zero".into());
// becomes
bail!("Sample rate is zero");
```

- [ ] **Step 4: Migrate id3show/src/ape.rs — Pattern A + B**

`ape.rs` uses `Box<dyn Error>` return types but only `?` propagation — no custom error construction. The two `Box<dyn Error>` occurrences in this file are 1 actual function signature (`show_metadata`) and 1 doc comment at line ~16 (`/// * \`Result<(), Box<dyn Error>>\``).

- Remove `use std::error::Error;` (or equivalent)
- Add `use anyhow::Result;`
- Apply Pattern A to the 1 function signature (`show_metadata`)
- Update the doc comment to reference `anyhow::Error` (or remove the return-type annotation)

- [ ] **Step 5: Migrate id3show/src/mp4.rs — Pattern A + B**

Same as Step 4 — one function, `?` propagation only.

- [ ] **Step 6: Migrate id3show/src/dsf.rs — Pattern A + B + C**

Add import: `use anyhow::{bail, Result};`

Apply Pattern A. Two error replacements:

```rust
return Err(format!("Unable to read DSF file {filename}. Error: {error}").into());
// becomes
bail!("Unable to read DSF file {filename}. Error: {error}");

return Err(format!("Unable to read DSF file {filename}").into());
// becomes
bail!("Unable to read DSF file {filename}");
```

- [ ] **Step 7: Verify id3show compiles and tests pass**

```bash
cargo test -p id3show
cargo clippy -p id3show -- -D warnings
```

Expected: zero errors, zero warnings.

- [ ] **Step 8: Commit**

```bash
git add id3show/
git-mit es
git commit -m "refactor(id3show): replace Box<dyn Error> with anyhow::Result

- 6 source files updated
- bail!() replaces ad-hoc Err constructions in mp3.rs, flac.rs, and dsf.rs
- main() now displays full error chain via {err:#}

Part of crumb id3-chf.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 5: Migrate id3export crate

**Files:**
- Modify: `id3export/src/main.rs`
- Modify: `id3export/src/tracks.rs`

- [ ] **Step 1: Migrate id3export/src/main.rs — Pattern A + B + display upgrade**

Remove `use std::error::Error;`. Add `use anyhow::Result;`.

Apply Pattern A to **both** functions that use `Box<dyn Error>`:

1. `run()` — the main entry point
2. `write_csv(filename: &str, tracks: Vec<tracks::Track>)` — a helper that uses only `?` propagation; just the signature changes, no error constructions to replace

Upgrade `main()` error display using the same display upgrade pattern as previous crates.

- [ ] **Step 2: Migrate id3export/src/tracks.rs — Pattern A + B + C**

`tracks.rs` has 12 `Box<dyn Error>` occurrences across the `Track` impl's read methods, plus 5 `Err(...).into()` custom error constructions that need Pattern C.

Add import: `use anyhow::{bail, Result};`

Apply Pattern A to all method signatures. These are **trait methods** (`fn`, not `pub fn`) with no `filename` parameter — the file path is stored in `self`. All 12 occurrences (6 trait declarations + 6 implementations) need updating. Representative examples:

```rust
// Before
fn read_mp3(&mut self) -> Result<(), Box<dyn Error>>
fn read_flac(&mut self) -> Result<(), Box<dyn Error>>
fn read(&mut self) -> Result<(), Box<dyn Error>>
// etc.

// After
fn read_mp3(&mut self) -> Result<()>
fn read_flac(&mut self) -> Result<()>
fn read(&mut self) -> Result<()>
// etc.
```

Apply Pattern C for the 5 `Err(...).into()` constructions:

```rust
// Literal errors (2 occurrences of "No path provided", ~lines 234 and 260)
return Err("No path provided".into());
// becomes
bail!("No path provided");

// Variable error (~line 359) — msg is a pre-formatted debug string from a match arm:
//   let msg = format!("{e:?}");
//   return Err(msg.into());
// becomes
bail!("{msg}");

// Further literal errors (~lines 372 and 513)
return Err("No frames found".into());
// becomes
bail!("No frames found");

return Err("No ID3 tag found".into());
// becomes
bail!("No ID3 tag found");
```

- [ ] **Step 3: Verify id3export compiles and tests pass**

```bash
cargo test -p id3export
cargo clippy -p id3export -- -D warnings
```

Expected: zero errors, zero warnings.

- [ ] **Step 4: Commit**

```bash
git add id3export/
git-mit es
git commit -m "refactor(id3export): replace Box<dyn Error> with anyhow::Result

- tracks.rs: 12 function signatures updated; bail!() replaces 5 Err(...).into() constructions
- main.rs: run() updated, display upgraded to {err:#}

Part of crumb id3-chf.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 6: Migrate id3cli-gen crate

**Files:**
- Modify: `id3cli-gen/src/main.rs`

- [ ] **Step 1: Migrate id3cli-gen/src/main.rs — Pattern A + B + display upgrade**

This file has **no** `use std::error::Error;` import — it uses the fully-qualified path `Box<dyn std::error::Error>` inline. There is nothing to remove. Add at the top of the file (after `#![forbid(unsafe_code)]`):

```rust
use anyhow::Result;
```

Apply Pattern A to `run()`:
```rust
// Before
fn run() -> Result<(), Box<dyn std::error::Error>> {
// After
fn run() -> Result<()> {
```

Upgrade `main()` error display:
```rust
// Before
Err(err) => {
    let msg = err.to_string().replace('\"', "");
    log::error!("{msg}");
    eprintln!("Error: {msg}");
    1
}
// After
Err(err) => {
    log::error!("{err:#}");
    eprintln!("Error: {err:#}");
    1
}
```

- [ ] **Step 2: Verify id3cli-gen compiles and tests pass**

```bash
cargo test -p id3cli-gen
cargo clippy -p id3cli-gen -- -D warnings
```

Expected: zero errors, zero warnings.

- [ ] **Step 3: Commit**

```bash
git add id3cli-gen/
git-mit es
git commit -m "refactor(id3cli-gen): replace Box<dyn Error> with anyhow::Result

Part of crumb id3-chf.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Task 7: Final workspace verification and crumb closure

- [ ] **Step 1: Full workspace test**

```bash
cargo test --workspace
```

Expected: all tests pass across all five crates. Zero regressions.

- [ ] **Step 2: Full workspace clippy**

```bash
cargo clippy --workspace -- -D warnings
```

Expected: zero warnings. If clippy emits any warnings about the anyhow usage (e.g., unused imports), fix them. Do **not** suppress with `#[allow(...)]` without a documented inline reason.

- [ ] **Step 3: Release build smoke test**

```bash
cargo build --workspace --release
```

Expected: all four binaries (`id3tag`, `id3show`, `id3export`, `id3cli-gen`) build successfully.

- [ ] **Step 4: Close the crumb**

```bash
crumbs close id3-chf
```

This updates only the `status` field in `.crumbs/improve-error-handling-by-using-a-proper-error-struct.md` to `closed`. No other fields change.

- [ ] **Step 5: Final commit**

```bash
git add .crumbs/improve-error-handling-by-using-a-proper-error-struct.md
git-mit es
git commit -m "chore: close crumb id3-chf — anyhow migration complete

All five crates now use anyhow::Result, bail!(), and .with_context().
main() error display upgraded to {err:#} in all four binaries.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

- [ ] **Step 6: Push and open a PR**

```bash
git push -u origin refactor/anyhow-migration
gh pr create \
  --title "refactor: replace Box<dyn Error> with anyhow across all crates" \
  --body "$(cat <<'EOF'
Closes crumb id3-chf.

## Summary
- Adds `anyhow = "1"` to workspace dependencies
- Replaces `Result<T, Box<dyn Error>>` with `anyhow::Result<T>` across all five crates (60 function signatures)
- `bail!()` replaces `Err("...".into())` and `Err(format!(...).into())` (46 constructions)
- `.with_context()` replaces `.map_err(|e| format!(...))` (3 sites in common and id3tag)
- `main()` error display upgraded to `{err:#}` in all four binaries

No application behaviour changes except richer error cause-chain output.
EOF
)" \
  --reviewer @copilot
```

- [ ] **Step 7: Code review loop**

Run the code-review skill and iterate until clean:

```
/code-review
```

Address any findings. Re-push and repeat until the review comes back with no new comments. Then run:

```
/simplify
```

Commit and push the simplification result if any changes were made. Run `/code-review` one final time after simplify and confirm it is clean.

---

## Verification

End-to-end correctness checks after the full migration:

1. **All tests green**: `cargo test --workspace` — all crates pass.
2. **Zero clippy warnings**: `cargo clippy --workspace -- -D warnings` — clean.
3. **Release binaries build**: `cargo build --workspace --release` — four binaries produced.
4. **Richer error output (manual smoke test)**: Run `id3tag --rename-file '%n' testdata/sample.mp3`. The error `File rename pattern %n likely won't create unique files` should appear as before. Then deliberately cause a chained error (e.g., point `id3tag` at a read-only file) — with `{err:#}`, the cause chain (e.g., `Unable to save tags: permission denied (os error 13)`) now appears in full instead of just the outermost message.
5. **Existing output unchanged**: `id3show testdata/sample.mp3` and `id3export testdata/` — output identical to before the migration.
