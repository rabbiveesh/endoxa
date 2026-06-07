---
id: b_eeef76dabaa0
slug: history-tree-not-linear
claim:
  kind: text
  text: >-
    Undo history is a branching tree of Revisions, not a flat stack. Each Revision has a
    parent index and a `last_child` pointer, enabling `:earlier`/`:later` time-travel
    navigation.
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
    turn: 4
  refs:
    - helix-core/src/history.rs:1-60
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges: []
coord: null
---

helix-core/src/history.rs: History stores `revisions: Vec<Revision>` where each Revision has `parent: usize` and `last_child: Option<NonZeroUsize>`. The comment explains ':earlier and :later can be used to jump to the closest revision to a moment in time'.
