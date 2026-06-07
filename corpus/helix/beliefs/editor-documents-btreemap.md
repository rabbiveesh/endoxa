---
id: b_90d5249240c4
slug: editor-documents-btreemap
claim:
  kind: text
  text: >-
    The Editor stores all open Documents in a BTreeMap keyed by DocumentId, and the layout
    tree in a separate Tree struct.
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
    - helix-view/src/editor.rs:1013-1018
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: supports
    target: b_ec5cb89fec5e   # per-view-selection
coord: null
---

helix-view/src/editor.rs defines `pub struct Editor` with `pub tree: Tree` and `pub documents: BTreeMap<DocumentId, Document>`. The Tree struct manages window layout; documents and views are decoupled.
