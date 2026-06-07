---
id: b_a2bfbcd8b17e
slug: data-section-checked-before-start-command
claim:
  kind: text
  text: >-
    TOKEN_DATA_SECTION must be checked before TOKEN_START_COMMAND in the scanner because
    inside a begin_paragraph the grammar expects _data_section first, and '=end' at column 0
    would otherwise be consumed as a new command paragraph.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-22T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-begin-end-2026-03
    turn: 4
  refs:
    - src/scanner.c:162-169
    - src/scanner.c:223
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

scanner.c line 163 comment: 'Must be checked before TOKEN_START_COMMAND since the parser expects _data_section first inside begin_paragraph.' The scanner branch for TOKEN_DATA_SECTION appears at line 169, before the TOKEN_START_COMMAND branch at line 223.
