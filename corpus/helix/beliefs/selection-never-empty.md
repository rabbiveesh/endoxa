---
id: b_495cc59a8d82
slug: selection-never-empty
claim:
  kind: text
  text: >-
    A Selection always contains at least one Range (the primary). The SmallVec is allocated
    inline for the common single-selection case.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 3
  refs:
    - helix-core/src/selection.rs:405-409
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges:
  - kind: supports
    target: b_5634c2e24470   # selection-gap-indexing
coord: null
---

selection.rs line 405: 'invariant: A selection can never be empty (always contains at least primary range)'. The internal storage is `SmallVec<[Range; 1]>`, so single-selection mode is stack-allocated.
