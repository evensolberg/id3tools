const completion: Fig.Spec = {
  name: "id3tag",
  description: "A simple application for updating metadata (ID3) information in music files.",
  options: [
    {
      name: ["-c", "--config-file"],
      description: "The name of the config file to be read.",
      isRepeatable: true,
      args: {
        name: "config-file",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["-l", "--log-config-file"],
      description: "The name of the YAML file containing the logging settings.",
      isRepeatable: true,
      args: {
        name: "log-config-file",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["--album-artist", "--aa"],
      description: "The album artist(s).",
      isRepeatable: true,
      args: {
        name: "album-artist",
        isOptional: true,
      },
    },
    {
      name: ["--album-artist-sort", "--aas"],
      description: "Album artist(s) sort name.",
      isRepeatable: true,
      args: {
        name: "album-artist-sort",
        isOptional: true,
      },
    },
    {
      name: ["--album-title", "--at"],
      description: "The title of the album. Use quotation marks for multi-word entries.",
      isRepeatable: true,
      args: {
        name: "album-title",
        isOptional: true,
      },
    },
    {
      name: ["--album-title-sort", "--ats"],
      description: "The album title sort name.",
      isRepeatable: true,
      args: {
        name: "album-title-sort",
        isOptional: true,
      },
    },
    {
      name: ["--disc-number", "--dn"],
      description: "The disc number.",
      isRepeatable: true,
      args: {
        name: "disc-number",
        isOptional: true,
      },
    },
    {
      name: ["--disc-number-total", "--dt"],
      description: "The total number of discs for the album.",
      isRepeatable: true,
      args: {
        name: "disc-total",
        isOptional: true,
      },
    },
    {
      name: ["--track-artist", "--ta"],
      description: "The track artist.",
      isRepeatable: true,
      args: {
        name: "track-artist",
        isOptional: true,
      },
    },
    {
      name: ["--track-album-artist", "--taa"],
      description: "Set album and track artist to be the same value.",
      exclusiveOn: [
        "--track-artist",
        "--album-artist",
      ],
      isRepeatable: true,
      args: {
        name: "track-album-artist",
        isOptional: true,
      },
    },
    {
      name: ["--track-artist-sort", "--tas"],
      description: "The sort name of the track artist(s). Use quotation marks for multi-word entries. Example: Artist is 'Alicia Keys', but this value may be 'Keys, Alicia'.",
      isRepeatable: true,
      args: {
        name: "track-artist-sort",
        isOptional: true,
      },
    },
    {
      name: ["--track-title", "--tt"],
      description: "The title of the track.",
      isRepeatable: true,
      args: {
        name: "track-title",
        isOptional: true,
      },
    },
    {
      name: ["--track-title-sort", "--tts"],
      description: "The sort title of the track.",
      isRepeatable: true,
      args: {
        name: "track-title-sort",
        isOptional: true,
      },
    },
    {
      name: ["--track-number", "--tn"],
      description: "The track number.",
      isRepeatable: true,
      args: {
        name: "track-number",
        isOptional: true,
      },
    },
    {
      name: ["--track-number-total", "--to"],
      description: "The total number of tracks for the disc.",
      isRepeatable: true,
      args: {
        name: "track-total",
        isOptional: true,
      },
    },
    {
      name: ["--track-number-count", "--tnc"],
      description: "Use number of files as total number of tracks.",
      exclusiveOn: [
        "--track-number-total",
      ],
      isRepeatable: true,
      args: {
        name: "track-count",
        isOptional: true,
      },
    },
    {
      name: ["--track-genre", "--tg"],
      description: "The track music genre.",
      isRepeatable: true,
      args: {
        name: "track-genre",
        isOptional: true,
      },
    },
    {
      name: ["--track-genre-number", "--tgn"],
      description: "The track music genre number.",
      exclusiveOn: [
        "--track-genre",
      ],
      isRepeatable: true,
      args: {
        name: "track-genre-number",
        isOptional: true,
      },
    },
    {
      name: ["--track-composer", "--tc"],
      description: "The composer(s) for the track. Use quotation marks for multi-word entries.",
      isRepeatable: true,
      args: {
        name: "track-composer",
        isOptional: true,
      },
    },
    {
      name: ["--track-composer-sort", "--tcs"],
      description: "The sort composer(s) for the track. Use quotation marks for multi-word entries. For example, if the composer is 'Ludwig van Beethoven', this value could be 'Beethoven, Ludwig van'.",
      isRepeatable: true,
      args: {
        name: "track-composer-sort",
        isOptional: true,
      },
    },
    {
      name: ["--track-date", "--td"],
      description: "The release date for the track. This is usually the album release date. Can be a year or a date.",
      isRepeatable: true,
      args: {
        name: "track-date",
        isOptional: true,
      },
    },
    {
      name: ["--track-comments", "--tm"],
      description: "The comments for the track. Use quotation marks for multi-word entries.",
      isRepeatable: true,
      args: {
        name: "track-comments",
        isOptional: true,
      },
    },
    {
      name: ["--picture-front-candidate", "--pfc"],
      description: "The front cover picture candidate file name.",
      isRepeatable: true,
      args: {
        name: "picture-front-candidate",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["--picture-back-candidate", "--pbc"],
      description: "The back cover picture candidate file name.",
      isRepeatable: true,
      args: {
        name: "picture-back-candidate",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["--picture-search-folder", "--psf"],
      description: "Folder(s) in which to look for the candidate front and back covers.",
      isRepeatable: true,
      args: {
        name: "picture-search-folder",
        isVariadic: true,
        isOptional: true,
      },
    },
    {
      name: ["--picture-max-size", "--pms"],
      description: "Picture maximum size in pixels for the longest edge.",
      isRepeatable: true,
      args: {
        name: "picture-max-size",
        isOptional: true,
      },
    },
    {
      name: ["--rename-file", "--rf"],
      description: "Renames the music file after setting the tags. Example: \"%dn-%tn %tt\"",
      isRepeatable: true,
      args: {
        name: "rename-file",
        isOptional: true,
      },
    },
    {
      name: ["-s", "--stop-on-error"],
      description: "Stop on error.",
    },
    {
      name: ["-r", "--dry-run"],
      description: "Iterate through the files and produce output without actually processing anything.",
    },
    {
      name: ["-p", "--print-summary"],
      description: "Print summary after all files are processed.",
    },
    {
      name: ["-o", "--detail-off"],
      description: "Don't display detailed information about each file processed.",
    },
    {
      name: ["-1", "--single-thread"],
      description: "Run processing single-threaded. Takes longer, but has less impact on the system.",
    },
    {
      name: ["--disc-number-count", "--dnc"],
      description: "Determine the disc number and total number of discs based on the folder structure.",
      exclusiveOn: [
        "--disc-number",
        "--disc-number-total",
      ],
    },
    {
      name: ["-h", "--help"],
      description: "Print help (see more with '--help')",
    },
    {
      name: ["-V", "--version"],
      description: "Print version",
    },
  ],
  args: {
    name: "files",
    isVariadic: true,
  },
};

export default completion;
