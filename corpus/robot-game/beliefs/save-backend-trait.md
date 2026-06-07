---
id: b_0d2609ac1905
slug: save-backend-trait
claim:
  kind: text
  text: >-
    Save persistence is abstracted behind a `SaveBackend` trait with two implementations:
    `LocalStorageBackend` (browser localStorage in WASM, /tmp files in native) and
    `InMemoryBackend` for tests.
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
    turn: 2
  refs:
    - robot-buddy-game/src/save.rs:111-118
    - robot-buddy-game/src/save.rs:167-187
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

ADR-002 introduced `SaveBackend` in commit 5045e75. `InMemoryBackend` uses `RefCell<SaveSlots>` for isolated per-test storage. `LocalStorageBackend` dispatches to extern C localStorage functions on WASM or `/tmp/{key}.json` on native.
