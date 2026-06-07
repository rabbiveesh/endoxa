---
id: b_ac45da1277f3
slug: escape-treated-as-generic-intseq
claim:
  kind: text
  text: >-
    Before v1.1.0, E<lt> was parsed as a generic interior_sequence with content that could
    contain nested markup, which was incorrect because E<...> content is always a plain
    entity reference.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-09-30T12:00:00Z
  valid_time:
    start: 2023-01-17
    end: 2026-05-30
  source:
    kind: conversation
    session: session-generic-cmd-2023-09
    turn: 4
  refs:
    - git:feat: parse E<...> as a distinct escape_sequence node
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.7
  asserted: 0.78
edges: []
coord: null
---

Prior to commit d4e6745 there was no escape_sequence rule. The scanner had no TOKEN_INTSEQ_ESCAPE_LETTER. The valid_time closes 2026-05-30 just before the fix landed.
