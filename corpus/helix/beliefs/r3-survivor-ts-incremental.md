---
id: b_af2124284257
slug: r3-survivor-ts-incremental
claim:
  kind: text
  text: >-
    Helix uses tree-sitter with incremental re-parsing on every edit: edits are translated
    to InputEdit structs and the tree is updated in-place rather than re-parsed from scratch
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2020-09-17
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 3
  refs:
    - helix-core/src/syntax.rs:1640
    - git:088f8a82af1b90e422c495cde92b537dedb1e419
    - git:36e7e2133fe1d472600cfd935b8046b8d50146c2
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.7
edges: []
coord: null
---

Inferable at write-time (2020-09-17): commit 088f8a82af introduced a `Syntax` struct with a `Parser` field and the comment chain from the preceding skeleton made clear that the design followed the tree-sitter API's incremental update path (parse with old_tree). By commit 36e7e2133 (2020-09-29) the function `generate_transaction_to_input_edits` was implemented and tested, converting `ChangeSet` → `Vec<tree_sitter::InputEdit>`. The design was structurally committed because: (1) tree-sitter's own API is incremental-or-nothing for non-trivial grammars; (2) the Rope-based input callback was wired directly to the parser. Has survived 276+ commits to syntax.rs; mechanically checkable: `grep -n 'InputEdit' helix-core/src/syntax.rs` still shows InputEdit usage in the parse pipeline.
