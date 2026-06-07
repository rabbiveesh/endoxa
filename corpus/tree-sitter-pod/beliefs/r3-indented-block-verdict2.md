---
id: b_d1535e2f9855
slug: r3-indented-block-verdict2
claim:
  kind: text
  text: >-
    Nesting =over/=item/=back in a sub-tree is wrong because POD can split these directives
    across sections when embedded in a Perl file
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-vov-agent
    turn: 3
  refs:
    - grammar.js:16
    - git:4b7f3ba55befbd86f1dc5baa674b746429a78944
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_3e6a1e87d6a9   # r3-indented-block-verdict1
  - kind: attacks
    target: b_3e6a1e87d6a9   # r3-indented-block-verdict1
coord: null
---

Commit 4b7f3ba (2023-01-18, just ~27 hours after 356d342) removes `indented_block` entirely. The commit message explicitly states: 'Don't attempt to nest over/item/back in a sub-tree because it might be intentionally split across multiple sections when embedded in a Perl file.' The fix replaces the nested rule with three flat sibling rules: `over_paragraph`, `item_paragraph`, `back_paragraph` at the top-level `pod: $ => repeat(choice(...))`. This defeats [[r3-indented-block-verdict1]] because the balanced-tree assumption breaks when POD is embedded piecemeal in Perl source. The flat structure has remained unchanged through 12 subsequent grammar.js commits.
