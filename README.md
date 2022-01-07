# ID3tag

A simple application for updating ID3 tags in APE, FLAC, MP3 and MP4 files.

The main purpose of this application is to be able to (easily) process files in bulk, so some of the functionality is optimized towards this.

## Usage

`id3tag [FILE(S)] [FLAGS] [OPTIONS]`

Examples:

- `id3tag **/*.flac -r --album-genre Rock` - dry run that would have set all FLAC files in all subdirectories to have the genre "Rock".
- `id3tag *.mp3 -q --disc-number 1 --album-artist Adele --album-title 25` - set album artist to "Adele", album title to "25" and disc number to 1 for all MP3 files in the current directory. Do not output anything other than errors.
- `id3tag *.mp3 -q --dn 1 --aa Adele --at 25` - same as previous, only shorter.

### Flags

|Short|Long|Description|
|-----|----|:---------|
`-c`|`--config-file`|The name of the config file for this application. If not specified, the app will try `~/.id3tag-config.toml`.
`-o`|`--detail-off`|Don't export detailed information about each file processed.
`-r`|`--dry-run`|Iterate through the files and produce output without actually processing anything.
`-h`|`--help`|Prints help information
`-p`|`--print-summary`|Print summary detail after all files are processed.
`-q`|`--quiet`|Don't produce any output except errors while working.
`-s`|`--stop-on-error`|Stop on error. If this flag isn't set, the application will attempt to continue in case of error.
`-V`|`--version`|Prints version information.

### Options

These are the values that can be set for each file. Note that all of these should be in the form `--option=value` or `--option Value`. You can also use the short-form alias: `--aa Artist` or `--aa=Artist`.

|Option|Alias|Description|
|------|-----|-----------|
|`--album-artist`|`--aa`|Set the name of the (main) artist on the album. This is usually set to be the same for all tracks and discs for an album. Use quotation marks for multi-word entries.
|`--album-artist-sort`|`--aas`|The default name on which the album artist is sorted. Example: Artist is "Alicia Keys", but the `artist_sort` may be "Keys, Alicia". This is usually set to be the same for all tracks and discs for an album. Use quotation marks for multi-word entries.
|`--album-title`|`--at`|Sets the name of the album. This is usually set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--album-title-sort`|`--ats`|Album title sort. This is usually set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--disc-number`|`--dn`|Sets the number of the disc from which the files are taken, usually 1.  This is often set to be the same for all tracks on an album.
|`--disc-number-total`|`--dt`|Sets the total number of discs for this album, usually 1. This is often set to be the same for all tracks and discs for an album.
|`--track-artist`|`--ta`|Sets the track artist. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--track-artist-sort`|`--tas`|Track artist sort. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--track-title`|`--tt`|Sets the name of the track. Use quotation marks for multi-word entries.
|`--track-title-sort`|`--tts`|Track title sort. Use quotation marks for multi-word entries.
|`--track-number`|`--tn`|Sets the track number.
|`--track-number-total`|`--to`|Sets the total number of tracks. This is normally set to be the same for all tracks on an album.
|`--track-number-count`|`--tnc`|Counts the number of files with the same extension in the same subdirectory, and uses it as the total number of tracks for the disc. In other words, if there are 5 MP3 files in the same directory, the track total count will be 5.<br>**NOTE:** Conflicts with `--track-number-total`.
|`--track-genre`|`--tg`|Sets the genre for the track, eg. "Rock", "Metal", "R&B", etc. This is often set to be the same for all tracks on an album, and often across discs as well. Use quotation marks for multi-word entries.
|`--track-genre-number`|`--tgn`|Sets the genre for the track, eg. "Rock", "Metal", "R&B", etc. based on the [ID3 Numerical Tag](https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D) (eg. 'Rock'=17, 'R&B'=14, 'Classical'=32). This is usually set to the same value for all tracks on a disc or album. Cannot be combined with '--track-genre'. Note that whichever of the two is passed LAST is used.
|`--track-composer`|`--tc`|Sets the composer(s) for the track, eg. "Ludwig van Beethoven", "Seal", "Keys, Alicia", etc. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--track-composer-sort`|`--tcs`|Track composer sort. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.
|`--track-comment`|`--tm`|Any comments related to the track (or album).
|`--track-date`|`--td`|Sets the release date for the track, eg. "2021", "2010-09-27". This is usually set to be the same for all tracks on an album.
|`--picture-front`|`--pf`|Sets the front cover picture. This is normally set to be the same for all tracks on an album. Looks for the cover picture alongside the music first, then in the invocation directory. **Not supported on APE.**
|`--picture-back`|`--pb`|Sets the back cover picture. This is normally set to be the same for all tracks on an album. Looks for the cover picture alongside the music first, then in the invocation directory. **Not supported on APE.**

Any values omitted are left as-is. Note that for artists and titles, multi-word entries must be surrounded by quotes - eg. "Demi Lovato".

#### A note on Genres

If both the `--track-genre` and `--track-genre-number` are passed, whichever value is passed _last_ is used.

Examples:

- `--track-genre Rock --track-genre-number 9` results in the genre represented by the number `9` (Metal) being used.
- `--track-genre-number 32 --track-genre "Chamber Music"` results in "Chamber Music".

If both `track_genre` and `track_genre_number` are present in a config file, the latter is used.

### Arguments

|Argument|Description|
|--------|:----------|
`<FILE(S)>`|One or more file(s) to process. Wildcards and multiple files (e.g. 2019*.flac 2020*.mp3) are supported.

## Configuration File

This file describes the configuration parameters found in the config file. You can specify a global config file at `~/.id3tag-config.toml` file or a specific version based on the location given:

- `-c` or `--config-file` by itself will use `~/.id3tag-config.toml`
- `-c somefile.toml` or `--config-file somefile.toml` will load the file specified.

|Parameter|Possible Values|Default Value|Description|
|:--------|:--------------|:------------|:----------|
|`detail_off`|`true`/`false`|`false`|Don't export detailed information about each file processed.
|`print_summary`|`true`/`false`|`false`|Print summary detail after all files are processed.
|`quiet`|`true`/`false`|`false`|Don't produce any output except errors while working.
|`stop_on_error`|`true`/`false`|`false`|If this flag isn't set, the application will attempt to continue in case of error.
|`album_artist`|||The name of the album artist.
|`album_artist_sort`|||The name on which the album artist is sorted. Example: Artist is "Alicia Keys", but the artist_sort may be "Keys, Alicia".
|`album_title`|||The title of the album.
|`album_title_sort`|||The sort title of the album. Example: 'The Wall' could be entered as 'Wall, The'. Not commonly used.
|`disc_number`|||The disc number, usually 1.
|`disc_number_total`|||The total number of discs that comprise the album, usually 1.
|`track_artist`|||The track's artist.
|`track_artist_sort`|||The track's artist sort.
|`track_title`|||The track's title.
|`track_title_sort`|||The track's title sort. Not commonly used.
|`track_number`|||The tracks on this disc.
|`track_number_total`|||The total number of tracks on this disc.
|`track_count`|`true`/`false`||Counts the number of tracks.
|`track_genre`|Any text||The track genre. Will be applied to each track.
|`track_genre_number`|`1`-`191`||The track genre number as [defined by ID3](https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D). Will be applied to each track. Overwrites any `track_genre` entries.
|`track_date`|||The release date for the album
|`track_composer`|Any text||The track composer. Will be applied to each track.
|`track_composer_sort`|Any text||The track composer. Will be applied to each track.
|`track_comment`|Any text||The comment(s) for the track. Will be applied to each track.
|`picture_front`|Any file name.||The name of the file which will be used as the front cover for the processed file(s). Looks for the cover picture alongside the music first, then in the invocation directory.
|`picture_back`|Any file name.||The name of the file which will be used as the front cover for the processed file(s). Looks for the cover picture alongside the music first, then in the invocation directory.

Note that any flags or options provided via the command line will override the default from the config file.

### Sample Configuration File

```toml
detail-off=false
print-summary=true
quiet=false
stop-on-error=false
track_genre="Metal"
track_composer="Hendrix, Jimi"
picture_front="cover-small.jpg"
```

## Options and Tags

These are the tags in various formats that are set using the different command line options:

|Option|Config File Value|FLAC Tag|MP3 Tag|MP4 Tag|
|:-----|:-----|:---|:--|:--|
|`--album-artist`|`album_artist`|`ALBUMARTIST`|`TPE2`|`aART`|
|`--album-artist-sort`|`album_artist_sort`|`ALBUMARTISTSORT`|`TSO2`|`soaa`|
|`--album-title`|`album_title`|`ALBUM`|`TALB`|`©alb`|
|`--album-title-sort`|`album_title_sort`|`ALBUMTITLESORT`|`TSOA`|`soal`|
|`--disc-number`|`disc_number`|`DISCNUMBER`|`TPOS`|`disk` [^2]|
|`--disc-number-total`|`disc_number_total`|`DISCTOTAL`|`TPOS` [^2]|`disk` [^2]|
|`--track-artist`|`track_artist`|`ARTIST`|`TPE1`|`©ART`|
|`--track-artist-sort`|`track_artist_sort`|`ARTISTSORT`|`TSOP`|`soar`|
|`--track-title`|`track_title`|`TITLE`|`TIT2`|`©nam`|
|`--track-title-sort`|`track_title_sort`|`TITLESORT`|`TSOT`|`sonm`|
|`--track-genre`|`track_genre`|`GENRE`|`TCON`|`©gen`|
|`--track-genre-number`|`track_genre_number`|[^1]|[^1]|[^1]|
|`--track-date`|`track_date`|`DATE`|`TDRC`|`©day`|
|`--track-composer`|`track_composer`|`COMPOSER`|`TCOM`|`©wrt`|
|`--track-composer-sort`|`track_composer_sort`|`COMPOSERSORT`|`TSOC`|`soco`|
|`--track-comment`|`track_comment`|`DESCRIPTION`|`COMM`|`©cmt`|
|`--picture-front`|`picture_front`|`PICTUREFRONT`|`APIC` [^2]|`covr` [^3]|
|`--picture-back`|`picture_back`|`PICTUREBACK`|`APIC` [^2]|NA [^3]|

[^1]: This looks up a value which is then inserted into `track_genre`.

[^2]: A modified version is actually used in the code, and the value is set using a dedicated function.

[^3]: While MP4 does allow one to set one or more images using the `covr` tag, there is no way to specify whether it's a front cover, back cover, etc. Hence, we currently only allow for the front image to be set.
