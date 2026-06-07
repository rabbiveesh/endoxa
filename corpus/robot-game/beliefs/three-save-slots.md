---
id: b_a68f7ff44662
slug: three-save-slots
claim:
  kind: text
  text: >-
    The game has exactly 3 save slots, typed as `[Option<SaveData>; 3]`.
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
    turn: 2
  refs:
    - robot-buddy-game/src/save.rs:103
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

The type alias `SaveSlots = [Option<SaveData>; 3]` is defined in save.rs. LocalStorageBackend serializes/deserializes this fixed-size array.
