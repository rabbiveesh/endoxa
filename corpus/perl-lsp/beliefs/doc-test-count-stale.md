---
id: b_470310c93de2
slug: doc-test-count-stale
claim:
  kind: text
  text: >-
    README.md states `cargo test  # 317 unit tests`, but the figure is stale: `cargo test --
    --list` enumerates far more (~801 reported by a sweep). A hardcoded count in prose
    drifts the moment a test is added — a TRUE observation that the documented number is
    wrong.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: wrong-docs-2026-06-04
    turn: 8
  refs:
    - README.md:223
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.85
edges: []
coord: null
---

A self-verifying class of wrong doc: any literal count baked into prose rots. Recorded as a true observation (the doc IS wrong), with the exact current number left soft (agent-reported ~801, confirm with `cargo test -- --list`).
