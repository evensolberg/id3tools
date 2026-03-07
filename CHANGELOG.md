# Changelog

All notable changes to this project will be documented in this file.

## [0.15.1] - 2026-03-07

### Bug Fixes

- Id3tag CLI flags not set properly
- Track-genre-number bug
- Filename doesn't get output when ratio is unacceptable

### CICD

- Remove Lint PR title

### Chore

- Dependencies update
- Fix lints
- Mergify autoupdates
- Autoinherit crates from workspace

### Doc

- Changelog Update
- Update changelog
- Add some documentation
- Update macro documentation
- Changelog
- README update
- Changelog

### Documentation

- Changelog Update
- Changelog update

### Feat

- Add the track_number function
- Export APE and DSF
- Output summary
- Added M4A support
- Added M4A support

### Features

- ID3Show updated with more detail
- Check for empty tags before renaming
- Show duration for FLAC and MP3
- Initial coding
- Read FLAC into Track
- Export FLAC to CSV working
- MP3 parse working

### Fix

- Fixed seconds output
- Fixed the elapsed time output
- Fixed all-zero MD5 in FLAC
- More detail to Cover Too Small message

### Miscellaneous Tasks

- Bump spin from 0.9.5 to 0.9.8
- Changelog
- Changelog
- Updated dependencies, added deployment
- Version updates
- Bump rustix from 0.37.19 to 0.37.25
- Maintenance and dependencies update
- Bump actions/download-artifact in /.github/workflows (#34)

### Refactor

- Upgrade id3tag to Clap4
- Default logging format changed
- Version bump after bug fix
- Use println for Show, tag outputs in seconds if > 1000 ms
- Move printed variables into brackets
- Clean up a bunch of code
- Format default_values.rs
- Clean up code further
- Dependency update
- Lint fixes
- Id3export version bump

### Security

- Update Dependabot settings

### Style

- Formatting

### Testing

- Test update for image resize result

## [0.14.4] - 2023-02-26

### Bug Fixes

- Disc count 0
- --dnc didn't include disc number
- Flags not working correctly
- Trying to figure out a bug
- Disc count bug
- Bug fixes
- Disc count bug fixed. Hopefully
- Trackinfo not set correctly for FLAC
- Image assignment fixed - hopefully
- Disc number bug
- Config not loading picture candidates

### Chore

- Update Changelog

### Documentation

- Updated docs to reflect changes
- README update

### Features

- Multi-threading
- Multithreading
- Add --taa flag
- -taa sets both track and album artist
- Check if filename is unchanged
- Add DSF (rename) support
- Create id3show application
- Imaging cont
- Image search and resize
- Minor version bump
- Imaging working - hopefully
- Add CLI dependency
- Gather_cover_paths function
- Image search for cover
- Improved function test slightly

### Miscellaneous Tasks

- CHANGELOG
- Clean up the code a bit
- Changelog update
- Changelog update
- Tidy up the code a little
- Code maintenance
- Changelog
- GH Action and code cleanup
- Cargo.lock
- Changelog update
- Clean up lints
- Update changelog
- CHANGELOG
- Tidy up
- CHANGELOG update
- Changelog update
- Changelog update
- Changelog update
- Changelog update
- Changelog Update
- Clean up lint
- Changelog update
- Update Changelog
- Dependencies update
- Code cleanup
- Changelog Update
- Added notes to refactor some functions
- CHANGELOG update
- Update gitignore
- Remove test output file
- Update Justfile
- Justfile update
- Update CHANGELOG
- CHANGELOG update
- Bump tokio from 1.18.2 to 1.24.1
- Bump tokio from 1.24.1 to 1.25.0
- Lockfile rebuild and Changelog Update
- Remove unused debugs
- Changelog
- Changelog
- Components update and debug removal

### Refactor

- Separated tags, moved funcs
- Simplified the config CLI check
- Minor changes
- Minor updates
- Cleaned up pedantic lints
- Move id3tag to a sub-project
- Reduce FLAC processing fn size
- Function rename for clarity
- Improved test, refactored image module
- Cleaned up lints and added tests
- Fixed a bunch of lints
- Filetypes optimization
- Fixed a bunch of lints
- Cleaned up some lints
- Change numerous functions
- Simplify complex functions
- Change the images module
- Move out more functions
- Update parse_options
- Simplify FLAC processing code
- Don't save resized images
- Simplify cover search
- Imaging and tagging rewrite

### Testing

- Added a bunch of tests
- Add tests for functions
- Add various tests in the images crate
- Remove Assay

### Refact

- Refactor and clean functions

## [0.9.4] - 2022-02-12

### Bug Fixes

- Handle the case where no log config is specified

### Features

- Logging update
- Disc numbering upgrade
- Rename ensure unique names
- Set total number of discs automagically

### Miscellaneous Tasks

- Comments update
- CHANGELOG update
- Update CHANGELOG
- Clean up some lint
- CHANGELOG update
- README update
- Changelog update
- Lint and changelog

## [0.8.4] - 2022-01-23

### Bug Fixes

- Trim strings
- Better handling of disc directories with names

### Miscellaneous Tasks

- Update changelog

## [0.8.3] - 2022-01-17

### Bug Fixes

- Rename bug
- Replace `.` with nothing in file rename

### Features

- Discover disc number

### Miscellaneous Tasks

- Update changelog
- Update CHNAGELOG
- Moved from radix to parse in str --> number
- Update changelog

## [0.7.1] - 2022-01-15

### Bug Fixes

- Crashed if unknown file found on `--tnc`

### Features

- Dry-run will tell you wait it will do, regular run will not
- Simplified the "normal" output
- File rename for FLAC
- Rename FLAC works
- MP4 rename
- File rename complete

### Miscellaneous Tasks

- Update changelog
- CHANGELOG update
- Crate version updates
- Changelog update

## [0.6.1] - 2022-01-07

### Chore

- Repo maintenance
- Changelog update

### Documentation

- Documentation update

### Features

- Added `--track-number-count` option
- Add config file support for file count

### Miscellaneous Tasks

- Added git cliff and conventional commits
- Repo update
- Update changelog

### Refactor

- Removed serde_derive crate

## [0.5.2] - 2022-01-01

## [0.5.0] - 2021-12-31

## [0.4.2] - 2021-12-31

## [0.4.0] - 2021-12-29

## [0.3.0] - 2021-12-29

<!-- generated by git-cliff -->
