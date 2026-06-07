---
id: b_fff321876614
slug: verbatim-no-intseq
claim:
  kind: text
  text: >-
    Verbatim paragraphs (lines starting with whitespace) do not parse interior sequences;
    B<bold> inside a verbatim block is treated as literal text.
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
    - grammar.js:60
    - src/scanner.c:282-294
    - git:fix: don't accept an intseq in verbatim (will close #4)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.92
edges:
  - kind: attacks
    target: b_aba5cbbed952   # wrong-verbatim-has-intseq
coord: null
---

grammar.js line 60 uses _content_plain (not _content) for verbatim_paragraph. scanner.c lines 287-294 check valid_symbols[TOKEN_INTSEQ_LETTER] before emitting TOKEN_INTSEQ_LETTER or TOKEN_INTSEQ_ESCAPE_LETTER, so a verbatim context never triggers sequence parsing. Fix landed in commit 5f3639c.
