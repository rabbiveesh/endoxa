---
id: b_761169c5ecfe
slug: memo-coarse-receiver-key-sound
claim:
  kind: text
  text: >-
    The cold-start type-query memo can key its receiver slot by InferredType VARIANT only
    (collapsing every ClassName(_) to one tag) and stay sound — the existing cycle-guard
    already shares that coarse key safely, so the result memo can reuse it.
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
    turn: 12
  refs:
    - git:a159008
    - git:cold-start perf — QueryState memoization
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

A perf optimization (2026-06-03) to memoize away the inheritance-diamond re-chase blowup. Asserted 0.85 — confident BECAUSE the cycle-guard precedent looked authoritative. That precedent was the trap.
