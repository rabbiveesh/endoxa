---
id: b_83167492bd08
slug: r3-indented-block-original
claim:
  kind: text
  text: >-
    =over/=item/=back can be structured as a nested indented_block sub-tree in the grammar
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-vov-agent
    turn: 1
  refs:
    - grammar.js:14
    - git:356d3421134aa7942eff1da1ed2758a6cca1146c
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.65
  asserted: 0.8
edges: []
coord: null
---

The original grammar (commit 356d342, 2023-01-17) modeled `=over ... =item ... =back` as a single `indented_block` rule that required all three directives to appear in sequence within a single sub-tree. This reflected the POD spec's notion of indented list blocks as balanced, self-contained structures. At write time, commit 356d342 adds `indented_block: $ => seq($.over_paragraph, repeat(choice($.item_paragraph, ...)), $.back_paragraph)` as the grammar's list representation.
