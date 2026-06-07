---
id: b_c49d91e2af75
slug: changeset-ops
claim:
  kind: text
  text: >-
    A ChangeSet is a sequence of three operations: Retain(n), Delete(n), and
    Insert(Tendril). It tracks both the required input length and the resulting output
    length.
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
    turn: 2
  refs:
    - helix-core/src/transaction.rs:13-20
    - helix-core/src/transaction.rs:62-68
    - helix-core/src/transaction.rs:153-154
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.95
edges: []
coord: null
---

helix-core/src/transaction.rs defines `pub enum Operation { Retain(usize), Delete(usize), Insert(Tendril) }` and `ChangeSet { changes: Vec<Operation>, len: usize, len_after: usize }`. The `compose` method asserts `self.len_after == other.len`.
