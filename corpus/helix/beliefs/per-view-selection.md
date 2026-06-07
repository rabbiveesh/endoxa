---
id: b_ec5cb89fec5e
slug: per-view-selection
claim:
  kind: text
  text: >-
    Each Document stores a separate Selection per ViewId, not one global selection. A
    document open in two splits can have independent cursor positions.
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
    turn: 4
  refs:
    - helix-view/src/document.rs:133-137
    - helix-view/src/document.rs:1217-1220
    - helix-view/src/document.rs:1828-1834
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.93
edges: []
coord: null
---

Document has field `selections: HashMap<ViewId, Selection>` (document.rs:136) and exposes `fn selection(&self, view_id: ViewId) -> &Selection`. The `set_selection` function inserts/updates the entry for that view ID.
