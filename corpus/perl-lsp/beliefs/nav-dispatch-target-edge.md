---
id: b_014d21d95820
slug: nav-dispatch-target-edge
claim:
  kind: text
  text: >-
    Navigation stores a resolved dispatch-target edge on MethodCall refs (invocant->target)
    and records an HONEST-MISS when it cannot resolve — it does not guess a target.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: nav-graph-2026-06-04
    turn: 6
  refs:
    - git:85d8a75
    - git:335d89c
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

The honest-miss is itself the anti-special-casing discipline: don't fabricate an answer to look complete.
