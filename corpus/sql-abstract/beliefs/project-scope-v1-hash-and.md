---
id: b_2bdbe1e6a9d1
slug: project-scope-v1-hash-and
kind: project-scope
claim:
  kind: text
  text: >-
    Since at least v1.50 (2008), hashrefs in WHERE clauses are ANDed by default (the `logic`
    option still defaults to OR for arrays, but hashref keys are always ANDed).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time:
    start: 2008-11-20
    end: 2999-01-01
  source:
    kind: conversation
    session: s-2008-refine
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:1020-1022
    - lib/SQL/Abstract.pm:2896-2898
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: 0.95
edges: []
coord: null
---

The POD explicitly says 'things in arrays are OR'ed, and things in hashes are AND'ed.' The code at _expand_expr checks `$kc > 1` for a hashref and calls `_expand_logop(and => $expr)`. This supersedes the earlier OR-hash behavior.
