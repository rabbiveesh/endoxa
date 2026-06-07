---
id: b_94a43292a389
slug: inequality-op-or-array-warning
claim:
  kind: text
  text: >-
    Using a multi-element arrayref with an inequality operator (!=, <>) without `-and`
    generates a warning because it produces an always-true 1=1.
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
    turn: 8
  refs:
    - lib/SQL/Abstract.pm:1265-1275
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_expand_hashtriple at line 1265-1275 detects when `$op =~ $self->{inequality_op}` and `lc($logic) eq 'or' and @values > 1`, then calls belch with a very specific message. The correct pattern is `{ col => { '!=' => ['-and', 1, 2] } }`.
