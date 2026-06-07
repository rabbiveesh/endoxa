---
id: b_ca94cac95096
slug: recurse-where-scalar-context-deprecated
claim:
  kind: text
  text: >-
    Calling `_recurse_where` in scalar context is deprecated and emits a warning.
    DBIx::Class historically did this; the behavior is preserved but discouraged.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 7
  refs:
    - lib/SQL/Abstract.pm:1569-1596
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.87
edges: []
coord: null
---

_recurse_where at lines 1585-1595 checks `wantarray` and calls `$self->belch` with 'Calling _recurse_where in scalar context is deprecated and will go away before 2.0' if not in list context.
