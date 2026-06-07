---
id: b_79df12c5f40b
slug: scope-begin-end-supported
kind: project-scope
claim:
  kind: text
  text: >-
    Since 2026-03-22 the grammar models =begin/=end blocks as begin_paragraph with a data
    child node, and =for paragraphs as for_paragraph, using a new external token
    _data_section.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-22T12:00:00Z
  valid_time:
    start: 2026-03-22
    end: 2999-01-01
  source:
    kind: conversation
    session: session-begin-end-2026-03
    turn: 2
  refs:
    - grammar.js:37-50
    - src/scanner.c:162-221
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges: []
coord: null
---

grammar.js lines 37-50 define begin_paragraph and for_paragraph. _data_section (TOKEN_DATA_SECTION in scanner.c) consumes all text until a bare =end at column 0.
