---
id: b_56302f26083a
slug: arch-four-layers
claim:
  kind: text
  text: >-
    perl-lsp is four layers, data flows DOWN only: LSP adapter (symbols.rs/backend.rs) ->
    cross-file (module_index/_resolver/_cache) -> builder (builder.rs) -> data model
    (file_analysis.rs).
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-02T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: crossfile-2026-03-02
    turn: 3
  refs:
    - CLAUDE.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

The dependency direction is the invariant. Higher layers query, never reach around.
