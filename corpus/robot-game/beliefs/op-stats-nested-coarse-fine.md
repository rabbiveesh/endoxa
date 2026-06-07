---
id: b_2d2a32c126e4
slug: op-stats-nested-coarse-fine
claim:
  kind: text
  text: >-
    The correct `OperationStats` serialization shape is `{ coarse: { add: {...}, ... },
    fine: { add_single: {...}, ... } }` — a nested struct, not a flat map.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 5
  refs:
    - git:Fix OperationStats shape + maxSize + expand boundary tests to 24
    - robot-buddy-domain/src/learning/operation_stats.rs:41-45
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_b4526618038e   # old-op-stats-was-flat
  - kind: attacks
    target: b_b4526618038e   # old-op-stats-was-flat
coord: null
---

Commit 788f6a2 ('Fix OperationStats shape') established the nested shape. JS adapter reads `profileState.operationStats.coarse.add`. The Rust struct has `coarse: HashMap<Operation, StatEntry>` and `fine: HashMap<SubSkill, StatEntry>` as separate fields.
