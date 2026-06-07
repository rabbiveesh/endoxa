---
id: b_5634c2e24470
slug: selection-gap-indexing
claim:
  kind: text
  text: >-
    Range anchor and head positions use gap (between-char) indexing: index 0 is before the
    first char, index N is after the last char. Ranges are inclusive-left, exclusive-right.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 2
  refs:
    - helix-core/src/selection.rs:26-47
    - helix-core/src/selection.rs:86-95
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges: []
coord: null
---

The doc comment on Range in selection.rs explicitly states 'gap indexing, meaning that their indices represent the gaps *between* chars'. Range::from() returns min(anchor,head) and Range::to() returns max; the range is inclusive on the left and exclusive on the right.
