---
id: b_9fd2e25e06c6
slug: project-scope-v1-hashref-or
kind: project-scope
claim:
  kind: text
  text: >-
    In SQL::Abstract v1.x, hashrefs in WHERE clauses used OR as the default inter-key logic
    (matching the constructor default `logic => 'OR'`).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2007-02-07T12:00:00Z
  valid_time:
    start: 2007-02-07
    end: 2008-11-20
  source:
    kind: conversation
    session: s-2007-import
    turn: 2
  refs:
    - git:Starting import of SQL-A
    - lib/SQL/Abstract.pm:317
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges:
  - kind: supersedes
    target: b_2bdbe1e6a9d1   # project-scope-v1-hash-and
coord: null
---

The original nwiger code (imported 2007-02-07) defaulted logic to OR. Hash keys in WHERE were joined by OR. This was the documented behavior through the 1.x series.
