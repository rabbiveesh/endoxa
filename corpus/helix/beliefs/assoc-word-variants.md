---
id: b_263c971f8839
slug: assoc-word-variants
claim:
  kind: text
  text: >-
    The Assoc enum (used to map cursor positions through ChangeSet edits) has special
    AfterWord and BeforeWord variants that snap to word boundaries during insertions, in
    addition to the plain Before/After/Sticky variants.
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
    turn: 3
  refs:
    - helix-core/src/transaction.rs:22-60
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.82
  asserted: 0.87
edges:
  - kind: supports
    target: b_c49d91e2af75   # changeset-ops
coord: null
---

transaction.rs defines `Assoc { Before, After, AfterWord, BeforeWord, BeforeSticky, AfterSticky }`. `AfterWord` does `s.chars().take_while(|&c| char_is_word(c)).count()`, landing after the initial word run of the inserted text.
