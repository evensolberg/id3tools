# File rename algorithm

- Read through the string and validate
  - Ensure the result would be unique in each case (ie. Track name or Track number is being used)
- Get mappings of --options to corresponding tag name, depending on file type
  - Use the `get_tag_names` function to map
  - Do this for both the long-form and short-form options (ie. `album-artist` and `aa`)
- Look up existing tags (this would be after applying them)
- Substitute options for tags (ie. %aa --> Madonna)
- Rename the file
