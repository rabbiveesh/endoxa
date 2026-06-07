---
id: b_3db0dbfd0622
slug: lsp-edits-sorted-stable
claim:
  kind: text
  text: >-
    LSP text edits are stable-sorted by start position before being applied, because some
    servers (notably Omnisharp) send them in reverse order and stable sort preserves equal-
    position edit ordering.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-07-27T12:00:00Z
  valid_time:
    start: 2024-07-28
    end: 2999-01-01
  source:
    kind: conversation
    session: s-lsp-types-refactor-2024
    turn: 3
  refs:
    - helix-lsp/src/lib.rs:503-506
    - git:stable sort lsp edits (#11357)
    - git:fix: lsp: Sort edits by start range, Omnisharp sends them in reverse
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.92
edges: []
coord: null
---

The original sort from commit 3d91c99c3 (2022-06-02) used `sort_unstable_by_key`. Commit 8e041c99d (2024-07-28) 'stable sort lsp edits' changed it to `sort_by_key` in generate_transaction_from_edits. The reason cited is that equal-position edits must retain their original server-specified order.
