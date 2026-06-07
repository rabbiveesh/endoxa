---
id: b_61a82c08adb5
slug: r3-nulls-verdict1
claim:
  kind: text
  text: >-
    NULLS FIRST/LAST is standard SQL (ISO/ANSI) and should be supported via -nulls =>
    'first'|'last' alongside -asc/-desc.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-s1
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:1150
    - git:2266ca5c0bf34c24ba7fbf6448ad1c34a082f240
    - git:b137b0744a3aaea3df1ba497345378e9d3f8da40
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_fdfcd0010369   # r3-nulls-original
  - kind: attacks
    target: b_fdfcd0010369   # r3-nulls-original
coord: null
---

Commit 2266ca5 (Dagfinn Ilmari Mannsåker, 2013-01-26) added a `-nulls` key to the ORDER BY hash handler. The implementation iterated all hash keys to allow both `-asc`/`-desc` and the new `-nulls` key simultaneously, generating `NULLS FIRST` or `NULLS LAST` SQL. A follow-up commit b137b07 fixed case-insensitivity for the `-nulls` value. This ATTACKS the prior assumption by treating NULLS placement as a first-class feature. [[r3-nulls-original]] was no longer true after this.
