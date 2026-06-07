---
id: b_dcc4bc09e544
slug: reduced-core-data-model
claim:
  kind: text
  text: >-
    The helix-core data model is: documents are Ropes; edits are ChangeSets composed of
    Retain/Delete/Insert operations; the Selection is a non-empty list of (anchor,head)
    Ranges with gap indexing; and all three are unified into Transactions for atomic
    application.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2020-05-20T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-early-arch-2020
    turn: 5
  refs: []
  derived_from:
    - b_c49d91e2af75
    - b_f9febdae5c18
    - b_5634c2e24470
    - b_f06dcbe307a4
confidence:
  directness: reduced
  observation_count: 4
  source_weight: 0.92
  asserted: null
edges:
  - kind: derived_from
    target: b_c49d91e2af75   # changeset-ops
  - kind: derived_from
    target: b_f9febdae5c18   # rope-text-storage
  - kind: derived_from
    target: b_5634c2e24470   # selection-gap-indexing
  - kind: derived_from
    target: b_f06dcbe307a4   # transaction-wraps-changeset
coord: null
---

This consensus summarizes [[changeset-ops]], [[rope-text-storage]], [[selection-gap-indexing]], and [[transaction-wraps-changeset]]. The model closely mirrors CodeMirror 6's design (per commit 44ff4d3c1) and was present from the earliest commits.
