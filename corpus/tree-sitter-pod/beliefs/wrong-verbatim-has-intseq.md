---
id: b_aba5cbbed952
slug: wrong-verbatim-has-intseq
claim:
  kind: text
  text: >-
    Interior sequences should be recognised inside verbatim paragraphs because the scanner
    emits TOKEN_INTSEQ_LETTER whenever it sees a capital letter followed by '<'.
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
    turn: 4
  refs:
    - git:fix: don't accept an intseq in verbatim (will close #4)
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

This was the initial behaviour before fix 5f3639c. The scanner originally called TOKEN(TOKEN_INTSEQ_LETTER) unconditionally on 'X<' without checking valid_symbols, so verbatim paragraphs would incorrectly parse B<1> as a sequence.
