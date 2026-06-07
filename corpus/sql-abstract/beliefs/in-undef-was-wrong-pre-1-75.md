---
id: b_e5e38ae1f542
slug: in-undef-was-wrong-pre-1-75
claim:
  kind: text
  text: >-
    Before v1.75, SQL::Abstract silently generated incorrect SQL when an -in list contained
    undef. This was a known incorrect behavior.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2010-12-21T12:00:00Z
  valid_time:
    start: 2007-02-07
    end: 2013-12-27
  source:
    kind: conversation
    session: s-2010-undoc-nest
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:1522-1526
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.94
edges: []
coord: null
---

The error message at lib/SQL/Abstract.pm:1522-1526 explicitly says 'SQL::Abstract before v1.75 used to generate incorrect SQL when the -in operator was given an undef-containing list: !!!AUDIT YOUR CODE AND DATA!!!'.
