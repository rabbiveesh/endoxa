---
id: b_da06968585cb
slug: hunch-mode-tristate
claim:
  kind: text
  text: >-
    The Mode enum in helix-view has exactly three variants (Normal, Select, Insert); there
    is no separate Command or Pending mode unlike some other modal editors.
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
    - helix-view/src/document.rs:57-70
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.62
edges: []
coord: null
---

helix-view/src/document.rs: `pub enum Mode { Normal = 0, Select = 1, Insert = 2 }` with explicit Display and FromStr. Helix uses keymap layers and the Compositor for command entry rather than a dedicated Command mode.
