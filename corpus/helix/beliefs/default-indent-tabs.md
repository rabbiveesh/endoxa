---
id: b_a11e48956ba6
slug: default-indent-tabs
claim:
  kind: text
  text: >-
    New documents default to tab-based indentation; the constant DEFAULT_INDENT is
    IndentStyle::Tabs.
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
    turn: 3
  refs:
    - helix-view/src/document.rs:50
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.7
edges: []
coord: null
---

helix-view/src/document.rs declares `const DEFAULT_INDENT: IndentStyle = IndentStyle::Tabs;` at line 50, and this is used in Document::open() when no language config specifies otherwise.
