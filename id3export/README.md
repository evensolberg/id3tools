# ID3Export

This utilitiy is used to export ID3 tags from MP3 files to a CSV file. The CSV file can then be used to import the tags into another set of MP3 files. Adduitionally, the utility can be used to export the summary information to a JSON file. This can be used to compare the tags of two sets of MP3 files or to compare the tags of the same set of MP3 files at different times. It is also a useful tool for debugging the ID3 tags of MP3 files or importing the tags into a database or other application.

## Usage

```console
Usage: id3export [OPTIONS] [FILE(S)]...

Arguments:
  [FILE(S)]...  One or more file(s) to process. Globs, wildcards and multiple files (e.g. *.mp3 Genesis/**/*.flac) are supported.

Options:
  -p, --print-summary                           Print summary detail for each session processed.
  -d, --show-detail                             Show detailed information about each file processed.
  -c, --csv-file [<csv-file>...]                The name of the CSV into which information is to be written.
  -j, --json-file [<json-file>...]             The name of the JSON into which information is to be written.
  -l, --log-config-file [<log-config-file>...]  The name of the YAML file containing the logging settings.
  -h, --help                                    Print help (see more with '--help')
  -V, --version                                 Print version
```
