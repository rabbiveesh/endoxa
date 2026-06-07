---
id: b_9dbb19602ed2
slug: r3-sqlcase-inverted
claim:
  kind: text
  text: >-
    The `case` option uses an inverted boolean: a truthy `case` means lowercase SQL, and the
    absence of `case` (or any non-'lower' value) means uppercase SQL.
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
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:313
    - lib/SQL/Abstract.pm:2046
    - git:32eab2da957ea33622610a8abc271c7855147904
    - git:96449e8ea5159e5448ebfc81dfa200dc674f366b
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.68
edges: []
coord: null
---

Inferable from 32eab2d (2006-09-28): `delete $opt{case} if $opt{case} && $opt{case} ne 'lower'` — any non-'lower' value is deleted, leaving only 'lower' or undef. `_sqlcase` then checks `if $self->{case}` to decide whether to lowercase. At write time this was inferable structurally: the delete-if-not-lower pattern encodes a two-state machine where truthiness means exactly one thing ('lower'). LDNOTE commented in 96449e8 (2008-10-16): 'if $self->{case} is true, then it contains 'lower', so we don't touch the argument ... crooked logic, but let's not change it!' — a human author acknowledged the smell but preserved the invariant. It survived ~554 commits including the Moo conversion. Mechanically checkable: `SQL::Abstract->new(case=>'lower')->where({a=>1})` produces lowercase SQL.
