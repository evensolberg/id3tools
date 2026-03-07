# id3tools

A set of command-line tools for viewing, editing, exporting, and renaming music files based on their metadata tags.

## Supported Formats

| Format | View | Edit Tags | Rename | Add Images | Export |
|--------|:----:|:---------:|:------:|:----------:|:------:|
| FLAC   | Yes  | Yes       | Yes    | Yes        | Yes    |
| MP3    | Yes  | Yes       | Yes    | Yes        | Yes    |
| MP4/M4A| Yes  | Yes       | Yes    | Yes [^1]   | Yes    |
| APE    | Yes  | Yes       | Yes    | No         | Yes    |
| DSF    | Yes  | No [^2]   | Yes    | No         | Yes    |

[^1]: MP4 only supports front cover images (no back cover due to iTunes limitations).
[^2]: DSF tag writing depends on upstream crate support. Tags can be supplied for renaming purposes.

## Tools

### id3tag

Bulk-update metadata tags and rename music files based on tag patterns. Supports parallel processing, config files, dry-run mode, and automatic image discovery with resizing.

See [id3tag/README.md](id3tag/README.md) for full documentation.

```sh
# Set album artist and genre for all FLAC files
id3tag *.flac --album-artist "Pink Floyd" --track-genre "Progressive Rock"

# Rename files based on tags (dry run)
id3tag *.mp3 -r --rename-file "%dn-%tn %tt"

# Set tags with automatic disc/track counting and cover art
id3tag *.flac --dnc --tnc --pfc cover.jpg --pfc folder.jpg
```

### id3show

Display metadata from audio files including tags, stream info, and embedded images.

```sh
id3show *.flac --show-detail --print-summary
```

### id3export

Export metadata to CSV for analysis with tools like [QSV](https://github.com/jqnatividad/qsv).

See [id3export/README.md](id3export/README.md) for full documentation.

```sh
id3export -c detail.csv **/*.flac
```

### Image Handling

id3tag can automatically find and embed cover art:

- **Candidate search**: looks for images by filename (e.g., `cover.jpg`, `front.jpg`, `folder.jpg`) in configurable directories
- **Automatic resizing**: images larger than the configured maximum (default 500px) are resized with Lanczos3 filtering
- **Aspect ratio validation**: rejects images with extreme aspect ratios (outside 1:1.5 to 1.5:1)
- **Front and back covers**: both supported for FLAC and MP3; front only for MP4

Configure via CLI flags (`--pfc`, `--pbc`, `--psf`, `--pms`) or the TOML config file.

## Installation

```sh
cargo install --path id3tag
cargo install --path id3show
cargo install --path id3export
```

## License

See [LICENSE](LICENSE) for details.
