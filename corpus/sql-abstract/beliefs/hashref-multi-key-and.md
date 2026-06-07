---
id: b_1bb81fca0c21
slug: hashref-multi-key-and
claim:
  kind: text
  text: >-
    A hashref with more than one key in the WHERE position is always treated as AND
    regardless of the `logic` constructor option.
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
    turn: 7
  refs:
    - lib/SQL/Abstract.pm:1018-1022
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

_expand_expr at line 1020-1022: `if ($kc > 1) { return $self->_expand_logop(and => $expr) }`. The `logic` option only affects top-level arrays and single-key hashrefs with array values.
