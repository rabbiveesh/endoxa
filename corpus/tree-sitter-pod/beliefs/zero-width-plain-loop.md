---
id: b_6638a738264e
slug: zero-width-plain-loop
claim:
  kind: text
  text: >-
    If TOKEN_CONTENT_PLAIN is emitted at the start of a line beginning with '=' (with
    got_plain=false), tree-sitter will spin forever because it receives a zero-width token
    at the same position.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-02-23T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-fixes-2023-02
    turn: 1
  refs:
    - src/scanner.c:306-329
    - git:fix: don't loop forever on broken intseqs
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: supports
    target: b_316aef733b04   # zero-width-data-section-loop
coord: null
---

scanner.c lines 321-329: after seeing a '=' at the start of a line (at_linefeed=true), if got_plain is false the scanner returns false instead of TOKEN(TOKEN_CONTENT_PLAIN). Fix e6b02d1 added this guard. A similar guard existed for double-linefeed (lines 306-314) from the earlier commit 5b58319.
