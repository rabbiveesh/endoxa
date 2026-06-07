---
id: b_7449b31f45de
slug: r3-survivor-selection-model
claim:
  kind: text
  text: >-
    Selections are the primary editing construct in helix: even a single cursor is an empty
    single-range Selection with anchor and head positions
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2020-05-25
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 1
  refs:
    - helix-core/src/selection.rs:407
    - git:44ff4d3c1f5da05e57ce99ba9d67b80a334def83
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

This was inferable at write-time (2020-05-25) from the module-level doc comment in the very first selection.rs commit (44ff4d3c1): "Selections are the primary editing construct. Even a single cursor is defined as an empty single selection range." The `Selection` struct held a `Vec<Range>` with a `primary_index`, with each `Range` having `anchor` and `head`. The design was already structurally committed: the entire transaction API in the same commit expressed edits as `change_by_selection`, making multi-range selection the load-bearing abstraction. There was no vim-style single-cursor mode fallback. Inferable at write-time from: (1) the module docstring explicitly encoding this invariant, (2) all edit commands operating on `Selection` uniformly, (3) no separate `Cursor` type. Has survived 8000+ commits; mechanically checkable: `grep -n 'pub struct Selection' helix-core/src/selection.rs` still shows the same invariant comment and SmallVec<[Range;1]> backing.
