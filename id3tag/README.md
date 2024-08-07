# ID3tag

A simple application for updating and renaming ID3 tags in APE, FLAC, MP3 and MP4 files. The application also supports renaming DSF files based on tags [^rename].

[^rename]: You can supply new tags to the application and these will be used when renaming.

The main purpose of this application is to be able to (easily) process files in bulk, so some of the functionality is optimized towards this.

Unless you supply the `-1` flag, files are processed in parallel.

## Usage

`id3tag [FILE(S)] [FLAGS] [OPTIONS]`

Examples:

- `id3tag **/*.flac -r --album-genre Rock` - dry run that would have set all FLAC files in all subdirectories to have the genre "Rock".
- `id3tag *.mp3 -q --disc-number 1 --album-artist Adele --album-title 25` - set album artist to "Adele", album title to "25" and disc number to 1 for all MP3 files in the current directory. Do not output anything other than errors.
- `id3tag *.mp3 -q --dn 1 --aa Adele --at 25` - same as previous, only shorter.

### Flags

| Short | Long              | Description                                                                                                                                                                   |
| ----- | ----------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-c`  | `--config-file`   | The name of the config file for this application. If not specified, the app will try `~/.id3tag-config.toml`.                                                                 |
| `-o`  | `--detail-off`    | Don't export detailed information about each file processed.                                                                                                                  |
| `-r`  | `--dry-run`       | Iterate through the files and produce output without actually processing anything.                                                                                            |
| `-1`  | `--single-thread` | Use single-threaded execution when processing files. This is slower, but has less impact on your system. You may need to use this on systems with hard disks instead of SSDs. |
| `-h`  | `--help`          | Prints help information                                                                                                                                                       |
| `-p`  | `--print-summary` | Print summary detail after all files are processed.                                                                                                                           |
| `-s`  | `--stop-on-error` | Stop on error. If this flag isn't set, the application will attempt to continue in case of error.                                                                             |
| `-V`  | `--version`       | Prints version information.                                                                                                                                                   |
| `-l`  | `--log`           | Configures the logging to suit your requirements.                                                                                                                             |

### Options

These are the values that can be set for each file. Note that all of these should be in the form `--option=value` or `--option Value`. You can also use the short-form alias: `--aa Artist` or `--aa=Artist`.

| Option                      | Alias   | Takes Value | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------------------------- | ------- | :---------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--album-artist`            | `--aa`  |     Yes     | Set the name of the (main) artist on the album. This is usually set to be the same for all tracks and discs for an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                            |
| `--album-artist-sort`       | `--aas` |     Yes     | The default name on which the album artist is sorted. Example: Artist is "Alicia Keys", but the `artist_sort` may be "Keys, Alicia". This is usually set to be the same for all tracks and discs for an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                       |
| `--album-title`             | `--at`  |     Yes     | Sets the name of the album. This is usually set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                           |
| `--album-title-sort`        | `--ats` |     Yes     | Album title sort. This is usually set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                     |
| `--disc-number`             | `--dn`  |     Yes     | Sets the number of the disc from which the files are taken, usually 1. This is often set to be the same for all tracks on an album.                                                                                                                                                                                                                                                                                                                              |
| `--disc-number-count`       | `--dnc` |     No      | Tries to figure out the disc number and total number of discs based on the name of the parent folder. If it contains "CD", "DISC" or "PART" (case insensitive), we'll attempt to discern the disc number and total number of discs. Otherwise these values are set to 1. Note that this conflicts with `--disc-number` and `--disc-number-total`. You can either use those two or this, but not both.                                                            |
| `--disc-number-total`       | `--dt`  |     Yes     | Sets the total number of discs for this album, usually 1. This is often set to be the same for all tracks and discs for an album.                                                                                                                                                                                                                                                                                                                                |
| `--track-artist`            | `--ta`  |     Yes     | Sets the track artist. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                  |
| `--track-album-artist`      | `--taa` |     No      | Sets the album artist and track artist at the same time. Conflicts with `--track-artist` and `--album-artist`.                                                                                                                                                                                                                                                                                                                                                   |
| `--track-artist-sort`       | `--tas` |     Yes     | Track artist sort. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                      |
| `--track-title`             | `--tt`  |     Yes     | Sets the name of the track. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                                                                          |
| `--track-title-sort`        | `--tts` |     Yes     | Track title sort. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                                                                                    |
| `--track-number`            | `--tn`  |     Yes     | Sets the track number.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `--track-number-total`      | `--to`  |     Yes     | Sets the total number of tracks. This is normally set to be the same for all tracks on an album.                                                                                                                                                                                                                                                                                                                                                                 |
| `--track-number-count`      | `--tnc` |     No      | Counts the number of files with the same extension in the same subdirectory, and uses it as the total number of tracks for the disc. In other words, if there are 5 MP3 files in the same directory, the track total count will be 5.<br>**NOTE:** Conflicts with `--track-number-total`.                                                                                                                                                                        |
| `--track-genre`             | `--tg`  |     Yes     | Sets the genre for the track, eg. "Rock", "Metal", "R&B", etc. This is often set to be the same for all tracks on an album, and often across discs as well. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                          |
| `--track-genre-number`      | `--tgn` |     Yes     | Sets the genre for the track, eg. "Rock", "Metal", "R&B", etc. based on the [ID3 Numerical Tag](https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D) (eg. 'Rock'=17, 'R&B'=14, 'Classical'=32). This is usually set to the same value for all tracks on a disc or album. Cannot be combined with `--track-genre`. Note that whichever of the two is passed LAST is used.                                                                               |
| `--track-composer`          | `--tc`  |     Yes     | Sets the composer(s) for the track, eg. "Ludwig van Beethoven", "Seal", "Keys, Alicia", etc. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                            |
| `--track-composer-sort`     | `--tcs` |     Yes     | Track composer sort. This is often set to be the same for all tracks on an album. Use quotation marks for multi-word entries.                                                                                                                                                                                                                                                                                                                                    |
| `--track-comment`           | `--tm`  |     Yes     | Any comments related to the track (or album).                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `--track-date`              | `--td`  |     Yes     | Sets the release date for the track, eg. "2021", "2010-09-27". This is usually set to be the same for all tracks on an album.                                                                                                                                                                                                                                                                                                                                    |
| `--picture-front-candidate` | `--pfc` |     Yes     | Can be used multiple times to specify front cover candidates. Will look alongside the music, in the parent directory, and in the directories specified with the `picture-search-folder` option.                                                                                                                                                                                                                                                                  |
| `--picture-back-candidate`  | `--pbc` |     Yes     | Can be used multiple times to specify back cover candidates. Will look alongside the music, in the parent directory, and in the directories specified with the `picture-search-folder` option.                                                                                                                                                                                                                                                                   |
| `--picture-search-folder`   | `--psf` |     Yes     | Specifies the sub-directories in which to search for picture candidates. These are relative to the music.                                                                                                                                                                                                                                                                                                                                                        |
| `--rename-file`             | `--rf`  |     Yes     | Renames the music file based on a tag pattern provided. Example: "%dn-%tn %tt" or "%disc-number-%track-number %track-name" gives "01-02 Bad Medicine", The tags follow the convention for the tag options listed in this table. Note that for "%disc-number-total" and "%track-number-total" you can also use "%dnt" and "%tnt" as file rename patterns in addition to the options listed above. This is done in an attempt to make it a little more intutitive. |

Any values omitted are left as-is. Note that for artists and titles, multi-word entries must be surrounded by quotes - eg. "Demi Lovato".

#### A note on Genres

If both the `--track-genre` and `--track-genre-number` are passed, whichever value is passed _last_ is used.

Examples:

- `--track-genre Metal --track-genre-number 9` results in the genre represented by the number `9` (Metal) being used.
- `--track-genre-number 32 --track-genre "Chamber Music"` results in "Chamber Music".

If both `track_genre` and `track_genre_number` are present in a config file, the latter is used.

### Arguments

| Argument    | Description                                                                                             |
| ----------- | :------------------------------------------------------------------------------------------------------ |
| `<FILE(S)>` | One or more file(s) to process. Wildcards and multiple files (e.g. 2019*.flac 2020*.mp3) are supported. |

### Rename Patterns

The tool can rename files based on ID3 tags. The tags follow the conventions for the options listed in the Options table above, but are repeated here for clarity. The following patterns are supported:

| Long Form              | Short Form   | Description                    |
| ---------------------- | ------------ | ------------------------------ |
| `%album-artist`        | `%aa`        | The album artist               |
| `%album-artist-sort`   | `%aas`       | The album artist sort          |
| `%album-title`         | `%at`        | The album title                |
| `%album-title-sort`    | `%ats`       | The album title sort           |
| `%disc-number`         | `%dn`        | The disc number                |
| `%disc-number-total`   | `%dt`/`%dnt` | The total number of discs      |
| `%track-artist`        | `%ta`        | The track artist               |
| `%track-artist-sort`   | `%tas`       | The track artist sort          |
| `%track-title`         | `%tt`        | The track title                |
| `%track-title-sort`    | `%tts`       | The track title sort           |
| `%track-number`        | `%tn`        | The track number               |
| `%track-number-total`  | `%to`/`%tnt` | The total number of tracks     |
| `%track-genre`         | `%tg`        | The track genre                |
| `%track-genre-number`  | `%tgn`       | The track genre number [^1]    |
| `%track-date`          | `%td`        | The release date for the album |
| `%track-composer`      | `%tc`        | The track composer             |
| `%track-composer-sort` | `%tcs`       | The track composer sort        |
| `%track-comment`       | `%tm`        | The comment(s) for the track   |

While there are tags for the front and back cover, these are not supported in the rename pattern.

#### Examples

- `--rename-file "%dn-%tn %tt"` will rename the file to "01-02 Bad Medicine".
- `--rename-file "%disc-number-%track-number %track-name"` will rename the file to "01-02 Bad Medicine".
- `--rename-file "%ta - %tn - %tt"` will rename the file to "Bon Jovi - 02 - Bad Medicine".

## Configuration File

This file describes the configuration parameters found in the config file. You can specify a global config file at `~/.id3tag-config.toml` file or a specific version based on the location given:

- `-c` or `--config-file` by itself will use `~/.id3tag-config.toml`
- `-c somefile.toml` or `--config-file somefile.toml` will load the file specified.

| Parameter                 | Possible Values      | Default Value | Description                                                                                                                                                                                                                     |
| :------------------------ | :------------------- | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------- |
| `detail_off`              | `true`/`false`       | `false`       | Don't export detailed information about each file processed.                                                                                                                                                                    |
| `print_summary`           | `true`/`false`       | `false`       | Print summary detail after all files are processed.                                                                                                                                                                             |
| `stop_on_error`           | `true`/`false`       | `false`       | If this flag isn't set, the application will attempt to continue in case of error.                                                                                                                                              |
| `single_thread`           | `true / false`       | `false`       | Use single-threaded execution.                                                                                                                                                                                                  |
| `album_artist`            |                      |               | The name of the album artist.                                                                                                                                                                                                   |
| `album_artist_sort`       |                      |               | The name on which the album artist is sorted. Example: Artist is "Alicia Keys", but the artist_sort may be "Keys, Alicia".                                                                                                      |
| `album_title`             |                      |               | The title of the album.                                                                                                                                                                                                         |
| `album_title_sort`        |                      |               | The sort title of the album. Example: 'The Wall' could be entered as 'Wall, The'. Not commonly used.                                                                                                                            |
| `disc_number`             |                      |               | The disc number, usually 1.                                                                                                                                                                                                     |
| `disc_count`              | `true`/`false`       |               | Tries to figure out the disc number based on the name of the parent folder. If it contains "CD", "DISC" or "PART" (case insensitive), we'll attempt to discern the disc number based on this. Otherwise this value is set to 1. |
| `disc_number_total`       |                      |               | The total number of discs that comprise the album, usually 1.                                                                                                                                                                   |
| `track_artist`            |                      |               | The track's artist.                                                                                                                                                                                                             |
| `track_album_artist`      |                      |               | Set the track artist and album artist at the same time.                                                                                                                                                                         |
| `track_artist_sort`       |                      |               | The track's artist sort.                                                                                                                                                                                                        |
| `track_title`             |                      |               | The track's title.                                                                                                                                                                                                              |
| `track_title_sort`        |                      |               | The track's title sort. Not commonly used.                                                                                                                                                                                      |
| `track_number`            |                      |               | The tracks on this disc.                                                                                                                                                                                                        |
| `track_number_total`      |                      |               | The total number of tracks on this disc.                                                                                                                                                                                        |
| `track_count`             | `true`/`false`       |               | Counts the number of tracks.                                                                                                                                                                                                    |
| `track_genre`             | Any text             |               | The track genre. Will be applied to each track.                                                                                                                                                                                 |
| `track_genre_number`      | `1`-`191`            |               | The track genre number as [defined by ID3](https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D). Will be applied to each track. Overwrites any `track_genre` entries.                                                 |
| `track_date`              |                      |               | The release date for the album                                                                                                                                                                                                  |
| `track_composer`          | Any text             |               | The track composer. Will be applied to each track.                                                                                                                                                                              |
| `track_composer_sort`     | Any text             |               | The track composer. Will be applied to each track.                                                                                                                                                                              |
| `track_comment`           | Any text             |               | The comment(s) for the track. Will be applied to each track.                                                                                                                                                                    |
| `picture_front_candidate` | Any file name.       |               | An array of names of files to look for. These are candidates for the front cover.                                                                                                                                               |
| `picture_back_candidate`  | Any file name.       |               | An array of names of files to look for. These are candidates for the back cover.                                                                                                                                                |
| `picture_search_folders`  | Any folder name.     | `.` & `..`    | An array of folders in which to look for cover candidates. `.` and `..` are added automatically.                                                                                                                                |
| `picture_max_size`        | Any positive number. | 500           | The maximum size (horizontally & vertically) of the cover. If the cover found is bigger, it will be resized to this size.                                                                                                       |
| `rename_file`             |                      |               | Renames the music file based on a tag pattern provided. Example: "%dn-%tn %tt" or "%disc-number-%track-number %track-name" gives "01-02 Bad Medicine", The tags follow the convention for the tag options listed in the         | Options table above. |

Note that any flags or options provided via the command line will override the default from the config file.

### Sample Configuration File

```toml
detail-off=false
print-summary=true
stop-on-error=false
log_config_file="~/.config/id3tag/logs.yml"

track_count=true
disc_count=true
track_genre="Metal"
track_composer="Hendrix, Jimi"
picture_front="cover-small.jpg"
picture_front_candidates=["folder.jpg", "front.jpg", "cover.jpg"]
picture_search_folders=["Scans", "Artwork"]
rename_file="%dn-%tn %tt"
```

## Options and Tags

These are the tags in various formats that are set using the different command line options.

| Option                  | Config File Value     | FLAC Tag          | MP3 Tag       | MP4 Tag     |
| :---------------------- | :-------------------- | :---------------- | :------------ | :---------- |
| `--album-artist`        | `album_artist`        | `ALBUMARTIST`     | `TPE2`        | `aART`      |
| `--album-artist-sort`   | `album_artist_sort`   | `ALBUMARTISTSORT` | `TSO2`        | `soaa`      |
| `--album-title`         | `album_title`         | `ALBUM`           | `TALB`        | `©alb`     |
| `--album-title-sort`    | `album_title_sort`    | `ALBUMTITLESORT`  | `TSOA`        | `soal`      |
| `--disc-number`         | `disc_number`         | `DISCNUMBER`      | `TPOS`        | `disk` [^2] |
| `--disc-number-total`   | `disc_number_total`   | `DISCTOTAL`       | `TPOS` [^2]   | `disk` [^2] |
| `--track-artist`        | `track_artist`        | `ARTIST`          | `TPE1`        | `©ART`     |
| `--track-artist-sort`   | `track_artist_sort`   | `ARTISTSORT`      | `TSOP`        | `soar`      |
| `--track-title`         | `track_title`         | `TITLE`           | `TIT2`        | `©nam`     |
| `--track-title-sort`    | `track_title_sort`    | `TITLESORT`       | `TSOT`        | `sonm`      |
| `--track-number`        | `track_number`        | `TRACKNUMBER`     | `TRCK`        | `trkn` [^2] |
| `--track-number-total`  | `track_number_total`  | `TRACKTOTAL`      | `TRCK-T` [^2] | `trkn` [^2] |
| `--track-genre`         | `track_genre`         | `GENRE`           | `TCON`        | `©gen`     |
| `--track-genre-number`  | `track_genre_number`  | [^1]              | [^1]          | [^1]        |
| `--track-date`          | `track_date`          | `DATE`            | `TDRC`        | `©day`     |
| `--track-composer`      | `track_composer`      | `COMPOSER`        | `TCOM`        | `©wrt`     |
| `--track-composer-sort` | `track_composer_sort` | `COMPOSERSORT`    | `TSOC`        | `soco`      |
| `--track-comment`       | `track_comment`       | `DESCRIPTION`     | `COMM`        | `©cmt`     |
| `--picture-front`       | `picture_front`       | `PICTUREFRONT`    | `APIC` [^2]   | `covr` [^3] |
| `--picture-back`        | `picture_back`        | `PICTUREBACK`     | `APIC` [^2]   | NA [^3]     |

## Logging

The application uses the [`log4rs`](https://crates.io/crates/log4rs) crate for logging. You can configure the logging using a YAML file. You can specify the location of the log file using the `-log`/`-l` flag or the `log_config_file` option in the configuration file. If the flag is used, it will override the value in the configuration file. Also, if the flag is used without a value, the default location is used.

The default location is `~/.config/id3tag/logs.yml`. The file should look like this:

```yaml
# Sample log config file
# Formatters: https://docs.rs/log4rs/latest/log4rs/encode/pattern/index.html

# These decide how to treat logs
appenders:
  # Log information messages and above to stdout
  stdout:
    kind: console
    encoder:
      pattern: "{highlight({level})} {message}{n}"
    filters:
      - kind: threshold
        level: info

  # Log warnings and errors to the local id3tag.log file
  # If you need to debug, the threshold below may be useful. See also the Formatters link
  # on how to add module information etc.
  logfile:
    kind: file
    path: "id3tag.log"
    encoder:
      pattern: "{date(%Y-%m-%d %H:%M:%S)} {highlight({level})} {message}{n}"
    filters:
      - kind: threshold
        level: warn

# This is where we decide where things go. Right now everything from the root module of the
# application and below go to the appenders defined above.
root:
  appenders:
    - stdout
    - logfile
```

---

[^1]: This looks up a value which is then inserted into `track_genre`. See [Wikipedia](https://en.wikipedia.org/wiki/ID3) for details. The Winamp Extended List is supported.

[^2]: A modified version is actually used in the code, and the value is set using a dedicated function.

[^3]: While MP4 does allow one to set one or more images using the `covr` tag, there is no way to specify whether it's a front cover, back cover, etc. Hence, we currently only allow for the front image to be set.
