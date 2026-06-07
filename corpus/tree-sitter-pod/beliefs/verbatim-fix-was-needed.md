---
id: b_34cec4b09252
slug: verbatim-fix-was-needed
claim:
  kind: text
  text: >-
    The assumption that verbatim paragraphs were correctly excluding interior sequences was
    wrong before 2023-02-25; the scanner emitted INTSEQ_LETTER unconditionally on any
    uppercase letter followed by '<'.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2023-02-23T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-fixes-2023-02
    turn: 3
  refs:
    - git:fix: don't accept an intseq in verbatim (will close #4)
    - src/scanner.c:287-294
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.98
edges:
  - kind: adjudicates
    target: b_aba5cbbed952   # wrong-verbatim-has-intseq
  - kind: attacks
    target: b_aba5cbbed952   # wrong-verbatim-has-intseq
coord: null
---

Commit 5f3639c diff shows the original code: 'if(c == "<") { TOKEN(TOKEN_INTSEQ_LETTER); }' with no valid_symbols gate. The fix adds 'valid_symbols[TOKEN_INTSEQ_LETTER]' as a guard.
