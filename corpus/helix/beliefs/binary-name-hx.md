---
id: b_3a7352fce906
slug: binary-name-hx
claim:
  kind: text
  text: >-
    The compiled binary is named `hx`, not `helix`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 1
  refs:
    - helix-term/src/main.rs:50
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

The CLI help text in main.rs prints `hx [FLAGS] [files]...` and the binary crate is `helix-term` with default-member `helix-term`. Running `cargo build` produces a binary named `hx`.
