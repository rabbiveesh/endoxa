---
id: b_d22012dea35b
slug: insert-sorts-hash-keys
claim:
  kind: text
  text: >-
    When insert() or update() is given a hashref, keys are sorted alphabetically to produce
    deterministic SQL and bind order. The `values()` method returns binds in the same sorted
    order.
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
    turn: 10
  refs:
    - lib/SQL/Abstract.pm:573-575
    - lib/SQL/Abstract.pm:3880-3896
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.93
edges: []
coord: null
---

_expand_insert_values at line 573-575 sorts hash keys: `[ sort keys %$data ], [ @{$data}{ sort keys %$data } ]`. The PERFORMANCE section of the POD documents using this property to reuse statement handles.
