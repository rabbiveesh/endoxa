---
id: b_316aef733b04
slug: zero-width-data-section-loop
claim:
  kind: text
  text: >-
    An empty =begin/=end block (no content between the two delimiters) would cause an
    infinite loop because TOKEN_DATA_SECTION could be emitted zero-width on every scan call.
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
    turn: 3
  refs:
    - src/scanner.c:53-57
    - src/scanner.c:169-173
    - git:fix: guard against zero-width data section token loops
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges: []
coord: null
---

scanner.c lines 169-173: did_zw_data flag in LexerState guards against re-emitting a zero-width TOKEN_DATA_SECTION. If got_content is false when =end is found, did_zw_data is set to 1; the next scan call checks and clears it, returning false instead of emitting again. Commit c4f8662 added this.
