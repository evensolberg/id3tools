# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Chore

- Update Changelog

### Documentation

- README update

### Miscellaneous Tasks

- Remove unused debugs

## [0.14.2] - 2023-02-20

### Refactor

- Imaging and tagging rewrite

## [0.14.1] - 2023-02-20

### Bug Fixes

- Image assignment fixed - hopefully

### Refactor

- Don't save resized images
- Simplify cover search

## [0.13.2] - 2023-02-18

### Miscellaneous Tasks

- CHANGELOG update
- Lockfile rebuild and Changelog Update

### Refactor

- Simplify FLAC processing code

## [0.13.1] - 2023-02-18

### Refactor

- Update parse_options

## [0.13.0] - 2023-02-18

### Bug Fixes

- Trackinfo not set correctly for FLAC

### Features

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
- Bump tokio from 1.18.2 to 1.24.1
- Bump tokio from 1.24.1 to 1.25.0
- Update CHANGELOG

### Refactor

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

### Testing

- Added a bunch of tests
- Add tests for functions
- Add various tests in the images crate
- Remove Assay

### Refact

- Refactor and clean functions

## [0.11.1] - 2022-03-20

### Features

- Add DSF (rename) support

## [0.11.0] - 2022-03-19

### Miscellaneous Tasks

- Changelog update

### Refactor

- Minor updates

## [0.10.8] - 2022-03-14

### Features

- Check if filename is unchanged

### Miscellaneous Tasks

- Changelog update

## [0.10.7] - 2022-03-14

### Documentation

- Updated docs to reflect changes

### Miscellaneous Tasks

- CHANGELOG update

### Refactor

- Minor changes

## [0.10.6] - 2022-03-11

### Features

- Add --taa flag
- -taa sets both track and album artist

### Miscellaneous Tasks

- CHANGELOG
- Tidy up

## [0.10.4] - 2022-03-06

### Bug Fixes

- Disc count bug fixed. Hopefully

### Miscellaneous Tasks

- Clean up lints
- Update changelog

## [0.10.2] - 2022-03-05

### Miscellaneous Tasks

- Cargo.lock
- Changelog update

## [0.10.1] - 2022-03-05

### Bug Fixes

- Bug fixes

### Features

- Multi-threading
- Multithreading

### Miscellaneous Tasks

- Code maintenance
- Changelog
- GH Action and code cleanup

## [0.9.8] - 2022-02-19

### Bug Fixes

- Flags not working correctly
- Trying to figure out a bug
- Disc count bug

### Miscellaneous Tasks

- Changelog update
- Tidy up the code a little

## [0.9.5] - 2022-02-13

### Bug Fixes

- Disc count 0
- --dnc didn't include disc number

### Miscellaneous Tasks

- Lint and changelog
- CHANGELOG
- Clean up the code a bit
- Changelog update

### Refactor

- Separated tags, moved funcs
- Simplified the config CLI check

## [0.9.4] - 2022-02-12

### Features

- Set total number of discs automagically

### Miscellaneous Tasks

- Changelog update

## [0.9.3] - 2022-02-11

### Features

- Rename ensure unique names

### Miscellaneous Tasks

- CHANGELOG update
- README update

## [0.9.2] - 2022-01-30

### Features

- Disc numbering upgrade

### Miscellaneous Tasks

- Update CHANGELOG
- Clean up some lint

## [0.9.1] - 2022-01-30

### Bug Fixes

- Handle the case where no log config is specified

### Miscellaneous Tasks

- CHANGELOG update

## [0.9.0] - 2022-01-30

### Bug Fixes

- Better handling of disc directories with names

### Features

- Logging update

### Miscellaneous Tasks

- Update changelog
- Comments update

## [0.8.4] - 2022-01-22

### Bug Fixes

- Trim strings

### Miscellaneous Tasks

- Update changelog

## [0.8.3] - 2022-01-17

### Bug Fixes

- Rename bug
- Replace `.` with nothing in file rename

### Miscellaneous Tasks

- Update CHNAGELOG
- Moved from radix to parse in str --> number

## [0.8.1] - 2022-01-16

### Features

- Discover disc number

### Miscellaneous Tasks

- Update changelog

## [0.8.0] - 2022-01-15

### Miscellaneous Tasks

- Crate version updates
- Changelog update

## [0.7.1] - 2022-01-15

### Features

- File rename complete

### Miscellaneous Tasks

- CHANGELOG update

## [0.7.0] - 2022-01-14

### Features

- Dry-run will tell you wait it will do, regular run will not
- Simplified the "normal" output
- File rename for FLAC
- Rename FLAC works
- MP4 rename

## [0.6.3] - 2022-01-07

### Bug Fixes

- Crashed if unknown file found on `--tnc`

### Miscellaneous Tasks

- Update changelog

## [0.6.1] - 2022-01-07

### Features

- Added `--track-number-count` option
- Add config file support for file count

### Miscellaneous Tasks

- Update changelog

## [0.6.0] - 2022-01-03

### Chore

- Repo maintenance
- Changelog update

### Documentation

- Documentation update

### Miscellaneous Tasks

- Added git cliff and conventional commits
- Repo update

### Refactor

- Removed serde_derive crate

## [0.5.2] - 2022-01-01

## [0.5.0] - 2021-12-31

## [0.4.0] - 2021-12-29

## [0.3.0] - 2021-12-29

<!-- generated by git-cliff -->
