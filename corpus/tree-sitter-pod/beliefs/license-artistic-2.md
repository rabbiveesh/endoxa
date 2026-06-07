---
id: b_5e0d2d784326
slug: license-artistic-2
claim:
  kind: text
  text: >-
    The project is licensed under Artistic-2.0 (not MIT); it was originally started without
    any formal license file.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-09-30T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-generic-cmd-2023-09
    turn: 2
  refs:
    - package.json:22
    - Cargo.toml:5
    - tree-sitter.json:18
    - git:Change license to Artistic-2.0
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges:
  - kind: supersedes
    target: b_ea79c76a69b9   # license-was-missing
coord: null
---

The LICENSE file and package.json license field were changed to Artistic-2.0 in commit a50d3b6. Before that commit the repo had an MIT-ish stub. Cargo.toml and tree-sitter.json also carry the Artistic-2.0 identifier.
