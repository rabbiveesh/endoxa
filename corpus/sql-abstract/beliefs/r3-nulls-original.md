---
id: b_fdfcd0010369
slug: r3-nulls-original
claim:
  kind: text
  text: >-
    SQL::Abstract ORDER BY only supports -asc/-desc; no NULLS placement control is provided.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2006-09-28
    end: 2013-01-26
  source:
    kind: conversation
    session: r3-s1
    turn: 1
  refs:
    - lib/SQL/Abstract.pm:1150
    - git:32eab2da957ea33622610a8abc271c7855147904
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: null
edges: []
coord: null
---

From the original SQLA design (nwiger, 2006) through the 1.x series, the `_order_by_chunks` HASHREF handler accepted exactly one key, which must be `-desc` or `-asc`. Any attempt to pass other keys caused a `puke`. NULLS FIRST / NULLS LAST control was simply absent — the tacit assumption was that sort-direction covers all useful ORDER BY modifiers.
