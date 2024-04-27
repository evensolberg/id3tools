# ID3Export

This utilitiy is used to export ID3 tags from MP3 files to a CSV file for use in other applications. For example, the CSV file can be used to import the tags into a database or a spreadsheet. Other usages can be data analysis or wrangling using tools like [QSV](https://github.com/jqnatividad/qsv) or [topfew](https://github.com/timbray/topfew/tree/main).

Example:

```sh
id3export -c detail.csv **/*.flac
cat summary.csv | qsv select title | sort
```

A later version may also output data in JSON format.

## Usage

```console
Usage: id3export [OPTIONS] [FILE(S)]...

Arguments:
  [FILE(S)]...
          One or more file(s) to process. Globs, wildcards and multiple files (e.g. *.mp3 Genesis/**/*.flac) are supported.

Options:
  -p, --print-summary
          Print summary detail for each session processed.

  -c, --csv-file [<csv-file>...]
          The name of the CSV into which the detailed information for each file is to be written. Default is 'details.csv' if not specified.

  -o, --show-detail
          Show detailed information about each file processed.

  -s, --summary-file [<summary-file>...]
          The name of the CSV into which summary information is to be written. Default is 'summary.csv' if not specified.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To filter out Unknown files using [QSV](https://github.com/jqnatividad/qsv), use the following syntax:

```sh
qsv search -s "file_format" -v "Unknown" < detail.csv > filtered.csv
```
