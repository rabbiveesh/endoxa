---
id: b_8ab6e9b4cabb
slug: honest-miss-over-guessing
claim:
  kind: text
  text: >-
    VERDICT: guessing a dispatch target when the invocant is unresolved produced WRONG go-
    to-definition jumps. NAV now records an HONEST-MISS instead — a wrong jump is worse than
    no jump, so 'best-effort guess' was actively harmful.
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
    turn: 10
  refs:
    - git:85d8a75
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_14acf43e8257   # dispatch-always-resolvable
  - kind: refines
    target: b_014d21d95820   # nav-dispatch-target-edge
coord: null
---

Defeats [[dispatch-always-resolvable]]; refines [[nav-dispatch-target-edge]]. The corrected principle: surface the miss, never fabricate a target to look complete (an instance of [[rule-no-special-casing]] applied to navigation).
