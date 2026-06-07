---
id: b_543ed2442d24
slug: hunch-cut-is-soft-stop
claim:
  kind: text
  text: >-
    =cut probably causes the parser to stop consuming POD nodes when embedded in a Perl
    source file, but the grammar comment says 'we don't really stop at =cut'.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-01-17T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-init-2023-01
    turn: 6
  refs:
    - grammar.js:26-27
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.55
edges: []
coord: null
---

grammar.js line 26 comment: 'We don't really stop at a =cut but it's handy for highlighting purposes'. Whether the embedding host (tree-sitter-perl) respects =cut as a real boundary is unclear from this repo alone.
