---
id: b_1b37eb48310b
slug: haiku-zip-double-ext
claim:
  kind: text
  text: >-
    A likely previous bug here appended '.zip' inside the asset_name format!, producing a
    double '.zip.zip' extension in the asset search.
author:
  kind: agent
  id: claude-haiku-4-5
  model: claude-haiku-4-5
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 2
  refs:
    - src/perl.rs:38-58
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.6
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.6 — appropriately hedged, and still wrong about the bug's nature.
