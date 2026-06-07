---
id: b_d9e191ae0b0f
slug: miniquad-pinned-commit
claim:
  kind: text
  text: >-
    The workspace pins `miniquad` to a specific git commit (350cb91b6f13) that includes a
    macOS focus bug fix not yet in any tagged release, bypassing crates.io.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-04-26T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s4-macroquad
    turn: 1
  refs:
    - Cargo.toml:9-18
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.82
edges: []
coord: null
---

Cargo.toml: `[patch.crates-io] miniquad = { git = '...', rev = '350cb91b6f13a1f369619c5024b563acef109d5f' }`. The comment explains the bug: app launches unfocused if mouse is moving at startup, clicks register inconsistently until alt-tab.
