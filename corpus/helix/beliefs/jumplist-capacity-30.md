---
id: b_e84ca206cd9a
slug: jumplist-capacity-30
claim:
  kind: text
  text: >-
    Each View's JumpList has a fixed capacity of 30 entries. When full and a new jump is
    pushed, the oldest entry is silently discarded.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-06-18T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-jumplist-fix-2024
    turn: 1
  refs:
    - helix-view/src/view.rs:26-57
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

helix-view/src/view.rs: `const JUMP_LIST_CAPACITY: usize = 30;`. The `push_impl` method: `while self.jumps.len() >= JUMP_LIST_CAPACITY { self.jumps.pop_front(); ... }`.
