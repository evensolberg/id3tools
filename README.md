# ID3tag

A simple application for updating ID3 tags in FLAC and MP3 files.

## Usage

`id3tag [FLAGS] [OPTIONS] [FILE(S)]...`

Examples:

- `id3tag -r --album-genre Rock **/*.flac` - dry run that would have set all FLAC files in all subdirectories to have the genre "Rock".
- `id3tag -q --disc-number 1 --album-artist Adele --album-title 25 *.mp3` - set album artist to "Adele", album title to "25" and disc number to 1 for all MP3 files in the current directory. Do not output anything other than errors.
- `id3tag -q --dn 1 --aa Adele --at 25 *.mp3` - same as previous, only shorter.

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
|`--album-artist`|`--aa`|Set the name of the (main) artist on the album.
|`--album-title`|`--at`|Sets the name of the album.
|`--album-genre`|`--ag`|Sets the genre for the album, eg. "Rock", "Metal", "R&B", etc.
|`--album-composer`|`--ac`|Sets the composer(s) for the album, eg. "Ludwig van Beethoven", "Seal", "Keys, Alicia", etc.
|`--album-date`|`--ad`|Sets the release date for the album, eg. "2021", "2010-09-27".
|`--disc-number`|`--dn`|Sets the number of the disc from which the files are taken, usually 1.
|`--disc-total`|`--dt`|Sets the total number of discs for this album, usually 1.
|`--track-artist`|`--ta`|Sets the track artist.
|`--track-title`|`--tt`|Sets the name of the track.
|`--track-number`|`--tn`|Sets the track number.
|`--track-total`|`--to`|Sets the total number of tracks.
|`--picture-front`|`--pf`|Sets the front cover picture.
|`--picture-back`|`--pb`|Sets the back cover picture.

Any values omitted are left as-is. Note that for artists and titles, multi-word entries must be surrounded by quotes - eg. "Demi Lovato".

### Arguments

|Argument|Description|
|--------|:----------|
`<FILE(S)>`|One or more file(s) to process. Wildcards and multiple files (e.g. 2019*.flac 2020*.mp3) are
                    supported.

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
|`genre`|Any text||The album genre. Will be applied to each track.
|`composer`|Any text||The album composer. Will be applied to each track.
`picture_front`|Any file name.||The name of the file which will be used as the front cover for the processed file(s). If just a filename is given, the application will look in the same folder as the file being processed for a file of that name.
`picture_back`|Any file name.||The name of the file which will be used as the front cover for the processed file(s). If just a filename is given, the application will look in the same folder as the file being processed for a file of that name.

Note that any flags or options provided via the command line will override the default from the config file.

### Sample Configuration File

```toml
detail-off=false
print-summary=true
quiet=false
stop-on-error=false
genre="Metal"
composer="Hendrix, Jimi"
picture-front="cover-small.jpg"
```
