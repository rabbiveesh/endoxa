---
id: b_f529b6ef29d1
slug: entry-is-grammar-js
claim:
  kind: text
  text: >-
    The grammar entry point is grammar.js; it defines all rules and declares the 10 external
    tokens consumed by scanner.c.
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
    turn: 1
  refs:
    - grammar.js:1-14
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.62
edges: []
coord: null
---

grammar.js at line 1 exports a grammar() call. The externals array at lines 3-14 declares all 10 external token symbols (_eol, _start_command, _start_plain, _start_verbatim, _content_plain, _intseq_letter, _intseq_start, _intseq_end, _data_section, _intseq_escape_letter).
