---
tags:
  - rust
  - flac
  - id3show
  - feature
aliases: []
doc_id: plan-id3-a8o
doc_name: audio-info-block-for-flac
crumb_id: id3-a8o
document_title: Show Audio Info block for FLAC (sample rate, bit depth, bitrate)
created_date: 2026-06-30
synopsis: >
  Implementation plan for adding a user-friendly "Audio Info:" block to id3show's FLAC
  output when --show-detail is passed. Surfaces sample rate, bit depth, encoded bitrate,
  channel count, and duration in one place, derived from metaflac StreamInfo and file size.
status: active
type: plan
revision: 3
review_date: 2026-06-30
reviewed_by: []
completed_date:
comments:
revision_history:
  - revision: 1
    date: 2026-06-30
    author: Claude
    notes: Initial plan, based on interactive requirements interview
  - revision: 2
    date: 2026-06-30
    author: Claude
    notes: >
      Fix misleading wording in Step 2 (calc_bitrate_kbps is a pure function — its
      caller supplies duration_secs). Fix Duration label to omit "mm:ss" suffix that
      is wrong for tracks longer than 59:59. Clarify 10 MB = 10_000_000 bytes in test
      arithmetic. Add calc_duration_string test cases (hour-boundary logic is untested).
      Add Notes section flagging the pre-existing mm:ss suffix issue in non-detail mode.
  - revision: 3
    date: 2026-06-30
    author: Claude
    notes: >
      Update decision table to reflect that the mm:ss suffix was removed in this PR
      (not deferred). Update Notes section to describe what was actually done rather
      than flagging a separate clean-up task. Update show_vorbis_comment call site to
      use a named local (show_duration) instead of inline !show_detail.
---

# Plan: Show Audio Info for FLAC (crumb id3-a8o)

## Context

Crumb id3-a8o ("Show sample rate, bit depth and bitrate for FLAC") asks `id3show` to expose key
technical audio properties for FLAC files. Currently, `sample_rate` and `bits_per_sample` are
buried inside the `--show-detail` "Stream Info:" raw-dump block, and bitrate is never shown at all.

The goal is a clean **"Audio Info:"** block that surfaces these properties in a user-friendly format
when `--show-detail` is passed, without touching non-detail output except to stop duplicating
Duration inside Vorbis Comments when detail mode is active.

Scope: FLAC only. Other formats are follow-up crumbs.

---

## Decisions Captured

| Question | Decision |
|---|---|
| Which bitrate? | Encoded (actual) — `file_size × 8 ÷ duration_seconds ÷ 1000` |
| Display when? | Only with `--show-detail` |
| Block name | `Audio Info:` |
| Block position | Before `Vorbis Comments:` (mirrors FLAC block order) |
| Fields | Channels, Sample Rate, Bit Depth, Bitrate, Duration |
| Duration in non-detail | Shown in Vorbis Comments without any suffix (pre-existing `mm:ss` suffix removed as part of this change) |
| Duration in detail | Shown inside `Audio Info:`, NOT repeated in Vorbis Comments |

---

## Files Modified

| File | Purpose |
|---|---|
| `id3show/src/flac.rs` | All feature code and unit tests |
| `.gitignore` | Add `.crumbs/index.csv` (regenerated on every write; should not be tracked) |

---

## Implementation Steps

### 1 — Obtain file size in `show_metadata`

At the top of `show_metadata`, before the block loop, call:

```rust
let file_size = std::fs::metadata(filename)?.len();
```

`std::fs` is already in scope via the standard library; no new dependency needed. The file is
known-good at this point (we just opened its tag), so the metadata call should not fail in
practice, but the `?` propagates any unexpected I/O error cleanly.

### 2 — Add `calc_bitrate_kbps` helper

```rust
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn calc_bitrate_kbps(file_size: u64, duration_secs: f64) -> u32 {
    if duration_secs <= 0.0 {
        return 0;
    }
    (file_size as f64 * 8.0 / duration_secs / 1000.0) as u32
}
```

This is a **pure function** — it receives `duration_secs` as a pre-computed parameter from its
caller (`show_audio_info`). It does not call `calc_duration_seconds` itself.

Clippy lint rationale:

- `cast_precision_loss` — `file_size as f64`: a `u64` larger than 2⁵³ loses low bits; acceptable for bitrate estimation.
- `cast_possible_truncation` — final `as u32`: the decimal fraction of kbps is intentionally discarded.
- `cast_sign_loss` — Clippy flags `f64 as u32` because `f64` can be negative; the `duration_secs <= 0.0` guard makes the result non-negative in practice, but the compiler cannot prove it.

### 3 — Add `show_audio_info` function

```rust
fn show_audio_info(si: &block::StreamInfo, file_size: u64) -> Result<()> {
    let duration_secs = calc_duration_seconds(si.total_samples, si.sample_rate)?;
    let duration_str  = calc_duration_string(si.total_samples, si.sample_rate)?;
    let bitrate       = calc_bitrate_kbps(file_size, duration_secs);

    println!("  Audio Info:");
    println!("    Channels    = {}", si.num_channels);
    println!("    Sample Rate = {} Hz", si.sample_rate);
    println!("    Bit Depth   = {} bits", si.bits_per_sample);
    println!("    Bitrate     = {} kbps", bitrate);
    println!("    Duration    = {duration_str}");
    Ok(())
}
```

**Note on the Duration label:** The formatted string returned by `calc_duration_string` already
encodes its own semantics (`mm:ss` for tracks under an hour; `hh:mm:ss` for longer tracks).
Appending a literal `" mm:ss"` suffix would be misleading for tracks longer than 59:59, so it
is omitted here. The pre-existing `show_vorbis_comment` non-detail path still carries that
suffix; fixing it is tracked as a separate clean-up (see Notes below).

### 4 — Update `StreamInfo` arm in `show_metadata`

Replace the current arm body:

```rust
metaflac::Block::StreamInfo(si) => {
    if show_detail {
        show_audio_info(si, file_size)?;   // NEW — user-friendly block
        show_streaminfo(si);               // EXISTING — raw dump (kept for completeness)
    }
    duration = calc_duration_string(si.total_samples, si.sample_rate)?;
}
```

`show_audio_info` is called first so it appears before "Stream Info:" in the output.
`show_streaminfo` returns `()` (no `?` needed).

### 5 — Suppress duplicate Duration in `show_vorbis_comment`

Change the trailing `println!` in `show_vorbis_comment` from unconditional to:

```rust
if !show_detail {
    println!("    Duration = {duration} mm:ss");
}
```

`show_detail` is already the third parameter of `show_vorbis_comment`, so no signature change
is needed.

### 6 — Add unit tests

Add a `#[cfg(test)]` module at the bottom of `flac.rs`. This will be the first test module in
`id3show`; all four helpers under test are currently untested.

#### `calc_bitrate_kbps` tests

```rust
#[test]
fn test_calc_bitrate_kbps_normal() {
    // 10_000_000 bytes (SI megabytes, not MiB) × 8 bits / 100 s / 1_000 = 800 kbps
    assert_eq!(calc_bitrate_kbps(10_000_000, 100.0), 800);
}

#[test]
fn test_calc_bitrate_kbps_zero_duration() {
    assert_eq!(calc_bitrate_kbps(10_000_000, 0.0), 0);
}
```

> **Arithmetic note:** 10 MiB = 10,485,760 bytes → 838 kbps at 100 s. The test uses
> `10_000_000` (SI MB) to obtain the round 800 kbps figure.

#### `calc_duration_seconds` tests

```rust
#[test]
fn test_calc_duration_seconds_normal() {
    // 44100 samples at 44100 Hz = exactly 1.0 second
    let result = calc_duration_seconds(44100, 44100).unwrap();
    assert!((result - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_calc_duration_seconds_zero_sample_rate() {
    assert!(calc_duration_seconds(44100, 0).is_err());
}
```

#### `calc_duration_string` tests

`calc_duration_string` contains hour-boundary logic that needs separate coverage:

```rust
#[test]
fn test_calc_duration_string_minutes() {
    // 2 minutes exactly → "02:00"
    let result = calc_duration_string(44100 * 120, 44100).unwrap();
    assert_eq!(result, "02:00");
}

#[test]
fn test_calc_duration_string_hours() {
    // 1 hour, 1 minute, 1 second = 3661 seconds → "01:01:01"
    let result = calc_duration_string(44100 * 3661, 44100).unwrap();
    assert_eq!(result, "01:01:01");
}
```

---

## Expected Output (detail mode)

```
some-track.flac
  Audio Info:
    Channels    = 2
    Sample Rate = 44100 Hz
    Bit Depth   = 16 bits
    Bitrate     = 812 kbps
    Duration    = 05:23
  Stream Info:
    Min Block Size: 4096
    ...
  Vorbis Comments:
    ARTIST = Toto
    ALBUM  = Toto IV
    ...
    (Duration no longer repeated here)
```

---

## Verification

1. `cargo build -p id3show` — must compile with zero warnings.
2. `cargo clippy -p id3show -- -D warnings` — must pass.
3. `cargo test -p id3show` — all six new tests must pass.
4. Manual smoke test with a sample FLAC:
   - `id3show --show-detail music/"01-13 Surf's Up.flac"` → "Audio Info:" block appears before
     "Stream Info:", Duration is not duplicated in Vorbis Comments.
   - `id3show music/"01-13 Surf's Up.flac"` (no flag) → Duration still shown in Vorbis Comments,
     no "Audio Info:" section present.

---

## Notes

### `mm:ss` suffix removal in non-detail mode

The pre-existing `show_vorbis_comment` unconditionally printed `"    Duration = {duration} mm:ss"`.
For tracks longer than 59:59, `calc_duration_string` returns `"hh:mm:ss"` format, making the
`mm:ss` label wrong. This was fixed as part of this PR: the suffix is now omitted entirely, and
Duration is printed only when `show_duration` is true (i.e. in non-detail mode). In detail mode,
Duration appears in the `Audio Info:` block instead.
