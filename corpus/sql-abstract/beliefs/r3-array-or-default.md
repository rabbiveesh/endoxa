---
id: b_0b2cc8c93fd0
slug: r3-array-or-default
claim:
  kind: text
  text: >-
    Arrayref values in WHERE clauses are joined with OR by default.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2006-09-28
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-s4
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:317
    - git:32eab2da957ea33622610a8abc271c7855147904
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

Inferable from the first commit (32eab2d, 2006-09-28): the original nwiger design states 'things in arrays are OR'ed, and things in hashes are AND'ed'. The `new()` constructor sets `logic => 'OR'` as the default. At write time this was inferable because: (1) the docs explicitly describe the OR-for-arrays semantics, (2) the `new()` code path sets the default without condition. No later change altered this default — it survived the LDAMI refactor (96449e8, 2008), the Moo conversion (3a9aca0, 2012), and 554+ subsequent commits to lib/SQL/Abstract.pm. Mechanically checkable: `SQL::Abstract->new->where([{a => 1}, {b => 2}])` produces `WHERE a = ? OR b = ?`.
