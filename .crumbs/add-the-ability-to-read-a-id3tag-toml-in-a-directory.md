---
id: id3-nn4
title: Add the ability to read a id3tag.toml in a directory
status: open
type: feature
priority: 3
tags: []
created: 2026-03-07
updated: 2026-03-08
closed_reason: ''
dependencies: []
---

# Add the ability to read a id3tag.toml in a directory

This way one can put some default values per directory. Example: folder is “FLAC/B/Bach, Johann Sebastian/“ .. The TOML can contain 

```toml
album_artist = "Bach, Johann Sebastian”
genre = “Classical”
```
(Or whatever the actual field names are).

This way, if the tool finds the file, it’ll read the values and apply them unless they have been supplied via the CLI.
This is all about bulk editing.
