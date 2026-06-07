---
id: b_58f5221c6d45
slug: jumplist-backward-bug-fixed
claim:
  kind: text
  text: >-
    The jumplist backward-at-capacity bug was confirmed by a code review fix: `push_impl`
    now returns the number of entries removed from the front, and `backward` applies
    `saturating_sub` to keep `self.current` aligned.
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2024-06-18T12:00:00Z
  valid_time:
    start: 2024-06-18
    end: 2999-01-01
  source:
    kind: conversation
    session: s-jumplist-fix-2024
    turn: 2
  refs:
    - helix-view/src/view.rs:43-80
    - git:Fix jump_backwards behaviour when jumplist is at capacity (#10968)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.95
  asserted: null
edges:
  - kind: adjudicates
    target: b_08b3ea9d47a2   # jumplist-backward-bug-old
  - kind: attacks
    target: b_08b3ea9d47a2   # jumplist-backward-bug-old
  - kind: supersedes
    target: b_08b3ea9d47a2   # jumplist-backward-bug-old
coord: null
---

Commit 668f1239a adds `fn push_impl(&mut self, jump: Jump) -> usize`, counting `num_removed_from_front`, and the `backward` method calls `current = current.saturating_sub(num_removed)`. This is the authoritative fix.
