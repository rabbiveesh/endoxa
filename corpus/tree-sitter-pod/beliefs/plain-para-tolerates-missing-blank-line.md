---
id: b_6d6d284feeac
slug: plain-para-tolerates-missing-blank-line
claim:
  kind: text
  text: >-
    The scanner intentionally allows a =cmd directive to follow a plain paragraph without an
    intervening blank line, even though the POD spec requires one.
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
    turn: 2
  refs:
    - src/scanner.c:320-329
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

scanner.c lines 321-329 comment: 'Technically there should be a blank line before the next command. But so many people omit it. We'll allow this here'. The scanner stops TOKEN_CONTENT_PLAIN when it sees '=' at the start of a line following a single newline.
