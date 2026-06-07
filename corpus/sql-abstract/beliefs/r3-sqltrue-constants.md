---
id: b_63a6c2b756b8
slug: r3-sqltrue-constants
claim:
  kind: text
  text: >-
    The SQL literals for boolean true and false default to '1=1' and '0=1' respectively, and
    are configurable but unlikely to change in typical usage.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2008-10-16
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-s4
    turn: 3
  refs:
    - lib/SQL/Abstract.pm:334
    - git:96449e8ea5159e5448ebfc81dfa200dc674f366b
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges: []
coord: null
---

Inferable from 96449e8 (Laurent Dami, 2008-10-16): `$opt{sqltrue} ||= '1=1'` and `$opt{sqlfalse} ||= '0=1'`. At write time these were inferable as stable defaults because: (1) they are universally recognized portable SQL boolean idioms, (2) the `||=` idiom signals that overriding is possible but the defaults are the expected path. Mechanically checkable: `SQL::Abstract->new->{sqltrue}` returns '1=1'. The constants survived ~551 commits to lib/SQL/Abstract.pm after they were introduced, including major refactors (Moo, the AQT rewrite). No commit changed the default values themselves.
