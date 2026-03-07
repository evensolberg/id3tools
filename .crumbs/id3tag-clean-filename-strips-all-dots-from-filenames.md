---
id: id3-ncs
title: 'id3tag: clean_filename strips all dots from filenames'
status: open
type: bug
priority: 2
tags:
- id3tag
- medium
created: 2026-03-06
updated: 2026-03-06
closed_reason: ''
dependencies: []
description: id3tag/src/rename_file.rs:133 — replace('.', "") strips ALL dots from filenames. 'Dr. Dre' becomes 'Dr Dre', 'No. 5' becomes 'No 5'. Too aggressive — should only strip leading/trailing dots or consecutive dots.
---

# id3tag: clean_filename strips all dots from filenames

id3tag/src/rename_file.rs:133 — replace('.', "") strips ALL dots from filenames. 'Dr. Dre' becomes 'Dr Dre', 'No. 5' becomes 'No 5'. Too aggressive — should only strip leading/trailing dots or consecutive dots.
