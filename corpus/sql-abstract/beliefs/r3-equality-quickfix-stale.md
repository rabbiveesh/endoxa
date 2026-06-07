---
id: b_778b2acf526d
slug: r3-equality-quickfix-stale
claim:
  kind: text
  text: >-
    The `equality_op`/`inequality_op` regex pair is a temporary quickfix introduced in
    2007/2008 that should eventually be replaced by a more seasoned API, but has not been.
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
    turn: 4
  refs:
    - lib/SQL/Abstract.pm:325
    - git:96449e8ea5159e5448ebfc81dfa200dc674f366b
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.55
  asserted: 0.6
edges: []
coord: null
---

Inferable from 96449e8 (Laurent Dami, 2008-10-16): the comment 'temporary quickfix (in 2007), should go through a more seasoned API' accompanies `$opt{equality_op} = qr/...` and `$opt{inequality_op} = qr/...`. At write time this was inferable as unstable: the author's own comment signals design debt. Despite that, the regexes survived ~551 commits and are still in-place in the current codebase as the primary mechanism for recognizing equality/inequality operators. The 'temporary' label was later amended to include '(in 2007)' making the staleness explicit. Mechanically checkable: `lib/SQL/Abstract.pm` line 325-328 still contains the 'temporary quickfix' comment and the same regex shapes.
