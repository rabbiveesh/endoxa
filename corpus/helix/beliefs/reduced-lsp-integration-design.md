---
id: b_24871cc1569b
slug: reduced-lsp-integration-design
claim:
  kind: text
  text: >-
    Helix's LSP integration sorts server edits into start-position order (stable since
    2024-07), uses UTF-16 offset encoding by default, and owns its own lsp-types crate
    (helix-lsp-types) since July 2024 for long-term LSP protocol control.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 6
  refs: []
  derived_from:
    - b_3db0dbfd0622
    - b_3648b2535805
    - b_9c7e4efe3333
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.9
  asserted: null
edges:
  - kind: derived_from
    target: b_3db0dbfd0622   # lsp-edits-sorted-stable
  - kind: derived_from
    target: b_3648b2535805   # lsp-default-offset-utf16
  - kind: derived_from
    target: b_9c7e4efe3333   # helix-lsp-types-extracted
coord: null
---

Synthesizes [[lsp-edits-sorted-stable]], [[lsp-default-offset-utf16]], and [[helix-lsp-types-extracted]]. These three design decisions together determine how arbitrary LSP servers interact with helix's document model.
