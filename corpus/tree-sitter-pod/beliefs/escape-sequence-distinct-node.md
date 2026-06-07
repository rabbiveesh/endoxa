---
id: b_d719660487ae
slug: escape-sequence-distinct-node
claim:
  kind: text
  text: >-
    E<...> is parsed as a distinct escape_sequence node (not interior_sequence) because its
    content is a plain entity name or number, not nested POD markup.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-05-31T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-escape-seq-2026-05
    turn: 1
  refs:
    - grammar.js:77-87
    - src/scanner.c:291-294
    - git:feat: parse E<...> as a distinct escape_sequence node
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_ac45da1277f3   # escape-treated-as-generic-intseq
coord: null
---

grammar.js lines 81-87: escape_sequence uses _escape_content = repeat1(_content_plain), so the scanner never emits INTSEQ_LETTER inside an E<...>. The grammar comment explains: 'The content is a repeat1 of plain tokens so that the scanner always sees TOKEN_CONTENT_PLAIN co-valid with TOKEN_INTSEQ_END'. This landed in v1.1.0 (commit d4e6745, 2026-05-31).
