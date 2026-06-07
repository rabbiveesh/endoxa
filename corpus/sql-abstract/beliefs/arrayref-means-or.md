---
id: b_b71632d3ce34
slug: arrayref-means-or
claim:
  kind: text
  text: >-
    Passing an arrayref as the value of a WHERE hash key generates OR-joined conditions: `{
    col => ['a','b'] }` yields `col = ? OR col = ?`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2008-refine
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:1098-1110
    - t/02where.t:19-29
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.97
edges: []
coord: null
---

Code at lib/SQL/Abstract.pm:1098-1109 handles ARRAY ref values by extracting logic (default OR from `$self->{logic}`) and distributing the key over elements. Tests in t/02where.t confirm the behavior.
