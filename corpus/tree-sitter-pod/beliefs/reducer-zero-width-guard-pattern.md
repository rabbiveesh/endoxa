---
id: b_1735ff4d9c35
slug: reducer-zero-width-guard-pattern
claim:
  kind: text
  text: >-
    This scanner uses a recurring pattern to prevent infinite loops: whenever it might emit
    a zero-width token, it either returns false or sets a guard flag (did_zw_data) to refuse
    the second emission.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-22T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-begin-end-2026-03
    turn: 6
  refs:
    - src/scanner.c:169-173
    - src/scanner.c:306-329
    - git:Don't confuse tree-sitter by emitting a zero-length token
  derived_from:
    - b_6638a738264e
    - b_316aef733b04
    - b_fff321876614
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.92
  asserted: null
edges:
  - kind: derived_from
    target: b_6638a738264e   # zero-width-plain-loop
  - kind: derived_from
    target: b_316aef733b04   # zero-width-data-section-loop
  - kind: derived_from
    target: b_fff321876614   # verbatim-no-intseq
coord: null
---

Three separate fixes address the same class of bug: 5b58319 (double-linefeed zero-width), e6b02d1 (single-linefeed before '='), c4f8662 (zero-width data section). Each applies the same guard: check a boolean condition before emitting, return false if no content. [[zero-width-plain-loop]] and [[zero-width-data-section-loop]] document two instances.
