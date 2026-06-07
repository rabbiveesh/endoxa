---
id: b_c2e428269478
slug: array-datatypes-option
claim:
  kind: text
  text: >-
    The `array_datatypes => 1` constructor option changes the meaning of arrayrefs in
    INSERT/UPDATE: they are passed directly to DBI as array-type bind values instead of
    being interpreted as `[$sql, @bind]` literal SQL.
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
    turn: 11
  refs:
    - lib/SQL/Abstract.pm:636-658
    - lib/SQL/Abstract.pm:2332-2351
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_expand_insert_value at line 641-644 checks `$self->{array_datatypes}` before treating an arrayref as a bind. Without this option, an arrayref in insert values is treated as `[$sql, @bind]` literal SQL.
