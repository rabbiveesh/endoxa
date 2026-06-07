---
id: b_6032efa0caaa
slug: between-requires-two-defined
claim:
  kind: text
  text: >-
    `-between` requires exactly an arrayref of two defined values, or a single
    scalar/arrayref literal. Any other combination (one value, three values, undef in the
    pair) throws a fatal error.
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
    turn: 9
  refs:
    - lib/SQL/Abstract.pm:1490-1507
    - t/05in_between.t:66-84
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.94
edges: []
coord: null
---

_expand_between at line 1490-1507 validates: `(@rhs == 1 and ref($rhs[0]) eq 'HASH' and $rhs[0]->{-literal})` OR `(@rhs == 2 and defined($rhs[0]) and defined($rhs[1]))`. Otherwise pukes. t/05in_between.t has exhaustive invalid tests.
