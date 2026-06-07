---
id: b_f06dcbe307a4
slug: transaction-wraps-changeset
claim:
  kind: text
  text: >-
    A Transaction combines a ChangeSet with an optional explicit Selection override.
    Applying a Transaction only changes the Rope content; it does NOT automatically update
    the Selection.
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
    - helix-core/src/transaction.rs:521-553
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges:
  - kind: supports
    target: b_c49d91e2af75   # changeset-ops
coord: null
---

Transaction::apply() at line 546 only calls `self.changes.apply(doc)` on the Rope. The selection field is `Option<Selection>` and is accessed separately via `transaction.selection()`. Callers (helix-view document) must map the selection through changes themselves.
