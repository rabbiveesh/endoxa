---
id: b_bc513609d2b4
slug: binary-name
claim:
  kind: text
  text: >-
    The WASM binary is named robot-buddy-game.wasm.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-04-26T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s4-macroquad
    turn: 1
  refs:
    - build-wasm.sh:16
    - Cargo.toml:2
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

build-wasm.sh copies `target/wasm32-unknown-unknown/release/robot-buddy-game.wasm` to `robot-buddy-game/www/`. The Cargo workspace member is `robot-buddy-game`.
