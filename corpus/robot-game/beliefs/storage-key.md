---
id: b_f898bd77919a
slug: storage-key
claim:
  kind: text
  text: >-
    Save data is stored in browser localStorage under the key `robotBuddySaves`.
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
    - robot-buddy-game/src/save.rs:100
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges: []
coord: null
---

The constant `STORAGE_KEY = "robotBuddySaves"` is defined in save.rs. All three save slots are serialized into a single JSON array under this key.
