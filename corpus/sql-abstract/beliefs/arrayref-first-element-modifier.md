---
id: b_816d5609f6ea
slug: arrayref-first-element-modifier
claim:
  kind: text
  text: >-
    If the first element of an arrayref value is `-and` or `-or`, it overrides the logic for
    that list: `{ col => ['-and', {'>'=>1}, {'<'=>5}] }` generates `col > ? AND col < ?`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2008-refine
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:1101-1109
    - lib/SQL/Abstract.pm:3040-3054
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supports
    target: b_b71632d3ce34   # arrayref-means-or
coord: null
---

_expand_hashpair_ident checks if `$v->[0]` matches /^-(and|or)$/i and shifts it, using that as the subjoin logic. This is how you force AND on an otherwise OR-defaulting arrayref. POD documents this pattern.
