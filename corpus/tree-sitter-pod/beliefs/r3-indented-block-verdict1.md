---
id: b_3e6a1e87d6a9
slug: r3-indented-block-verdict1
claim:
  kind: text
  text: >-
    Nesting =over/=item/=back in an indented_block sub-tree is the correct grammar structure
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-vov-agent
    turn: 2
  refs:
    - grammar.js:17
    - git:356d3421134aa7942eff1da1ed2758a6cca1146c
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_83167492bd08   # r3-indented-block-original
  - kind: attacks
    target: b_83167492bd08   # r3-indented-block-original
coord: null
---

Commit 356d342 (2023-01-17) establishes `indented_block` as the grammar rule for list content: `indented_block: $ => seq($.over_paragraph, repeat(choice($.item_paragraph, $.plain_paragraph, $.verbatim_paragraph, $._blank_line)), $.back_paragraph)`. This verdict claims that balanced nesting is both structurally correct and sufficient for POD. Attacks the flat-listing assumption in [[r3-indented-block-original]] by replacing it with an explicit tree structure.
