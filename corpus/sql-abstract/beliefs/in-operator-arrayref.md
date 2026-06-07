---
id: b_5f38926570ad
slug: in-operator-arrayref
claim:
  kind: text
  text: >-
    The `-in` operator takes an arrayref: `{ col => { -in => [1,2,3] } }` yields `col IN
    (?,?,?)`. An empty arrayref generates sqlfalse (0=1); `-not_in => []` generates sqltrue
    (1=1).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2010-12-21T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2010-undoc-nest
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:1509-1536
    - t/05in_between.t:10-17
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.97
edges: []
coord: null
---

_expand_in at line 1509 handles this. Line 1533 handles empty: `return $self->${ \( $op =~ /^not/ ? 'sqltrue' : 'sqlfalse' ) } unless @rhs`. undef inside the list pukes with an audit warning.
