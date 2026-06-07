---
id: b_08b3ea9d47a2
slug: jumplist-backward-bug-old
claim:
  kind: text
  text: >-
    Before June 2024, jumping backward through the jumplist when the list was at capacity
    would produce incorrect positions because `self.current` was not adjusted when entries
    were dropped from the front.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-06-18T12:00:00Z
  valid_time:
    start: 2020-10-07
    end: 2024-06-18
  source:
    kind: conversation
    session: s-jumplist-fix-2024
    turn: 1
  refs:
    - git:Fix jump_backwards behaviour when jumplist is at capacity (#10968)
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

Commit 668f1239a (2024-06-18) 'Fix jump_backwards behaviour when jumplist is at capacity' rewrote `push_impl` to return `num_removed_from_front` and used it to `saturating_sub` from `self.current` in the backward method. The old `push` just discarded front elements without updating the cursor index.
