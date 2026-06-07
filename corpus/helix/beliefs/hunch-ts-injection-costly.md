---
id: b_2730c0a9a715
slug: hunch-ts-injection-costly
claim:
  kind: text
  text: >-
    Injection layer computation is likely the dominant cost during incremental re-parse of
    large files with many embedded languages (e.g. HTML with embedded JS and CSS).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-12-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-compositor-2020
    turn: 6
  refs:
    - helix-core/src/syntax.rs:1290-1310
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.6
edges:
  - kind: supports
    target: b_adb55f6443eb   # syntax-uses-hop-slotmap-layers
coord: null
---

The Syntax::update loop processes layers iteratively, and injection queries run fresh per modified layer. With deeply nested injections the HopSlotMap can grow large. This is a hunch based on reading the layer update code — no profiling data available.
