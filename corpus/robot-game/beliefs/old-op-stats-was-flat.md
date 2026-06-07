---
id: b_b4526618038e
slug: old-op-stats-was-flat
claim:
  kind: text
  text: >-
    An agent working on the Rust+WASM bridge incorrectly assumed `OperationStats` could be
    serialized as a flat map using `#[serde(flatten)]`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 4
  refs:
    - git:Fix OperationStats shape + maxSize + expand boundary tests to 24
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

Before the fix in commit 788f6a2, the Rust struct used `#[serde(flatten)]` attempting to merge coarse and fine into one map. Two different enum-keyed HashMaps cannot round-trip via serde flatten, so serialization was silently broken.
