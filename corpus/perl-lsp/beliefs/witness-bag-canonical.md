---
id: b_e9860602b115
slug: witness-bag-canonical
claim:
  kind: text
  text: >-
    Type inference has ONE source of truth: the witness bag. Production is bag.push(...);
    consumption is a ReducerRegistry query. There is no second source. Adding type behavior
    = adding a reducer; never write InferredType directly or build a parallel query helper.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-04-27T12:00:00Z
  valid_time:
    start: 2026-04-27
    end: 2999-01-01
  source:
    kind: conversation
    session: rhai-plugins-2026-04-27
    turn: 4
  refs:
    - CLAUDE.md
    - docs/adr/bag-canonical.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges:
  - kind: supersedes
    target: b_3f25aea81063   # builder-infer-expression-type
coord: null
---

Supersedes [[builder-infer-expression-type]]. Two strict phases: collect in populate_witness_bag(), reduce via the ReducerRegistry (reducers claim attachment shapes in registration order).
