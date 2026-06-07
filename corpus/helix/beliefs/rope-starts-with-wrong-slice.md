---
id: b_c68122b2bd6c
slug: rope-starts-with-wrong-slice
claim:
  kind: text
  text: >-
    Before September 2024, `RopeSliceExt::starts_with` was silently wrong: it sliced `len -
    text.len()` bytes from the front instead of the first `text.len()` bytes, so it was
    actually testing the wrong region of the rope.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-21T12:00:00Z
  valid_time:
    start: None
    end: 2024-09-21
  source:
    kind: conversation
    session: s-rope-fix-2024
    turn: 1
  refs:
    - git:Fix Rope.starts_with. (#11739)
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

Commit 5717aa8e3 (2024-09-21) 'Fix Rope.starts_with' changed `self.get_byte_slice(..len - text.len())` to `self.get_byte_slice(..text.len())`. The first form computes the wrong window — it drops the prefix rather than taking it.
