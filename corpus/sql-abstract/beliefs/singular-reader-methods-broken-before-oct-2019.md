---
id: b_d451baf5bbc9
slug: singular-reader-methods-broken-before-oct-2019
claim:
  kind: text
  text: >-
    Before October 2019, calling `$sqla->expander('name')` (singular, one arg) as a read
    operation silently fell through to the plural setter path and did nothing useful instead
    of returning the current value.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2019-01-01T12:00:00Z
  valid_time:
    start: 2007-02-07
    end: 2019-10-07
  source:
    kind: conversation
    session: s-2019-v2-rewrite
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:413-430
    - git:singular methods as readers never worked, mst is an idiot. fixed.
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.88
edges: []
coord: null
---

The original eval'd method body for singular accessor just called the plural setter. Commit f34df2a (2019-10-07) fixed this: 'singular methods as readers never worked, mst is an idiot. fixed.' Now single-arg calls to expander/renderer/etc. return `$self->_ext_rw($name, @_)`.
