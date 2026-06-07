---
id: b_e7dbaed447fa
slug: r3-content-repeat1
claim:
  kind: text
  text: >-
    The `_content` rule uses `repeat1` (not `repeat`) to guarantee at least one content
    token; empty content is structurally absent, not an empty node
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2023-01-17
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-agent
    turn: 2
  refs:
    - grammar.js:66
    - git:2f83f4266f797d504c13c3ef20ffd88f1c438a78
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges: []
coord: null
---

Inferred at commit 2f83f42 (2023-01-17): the initial content rule `_content: $ => repeat1(choice($._content_plain, $.interior_sequence))` uses `repeat1` from the start. At write time this was inferable from the grammar semantics: a `content` node with zero children would be structurally identical to its absence, causing ambiguity; `repeat1` forces the grammar to represent 'no content' by omitting the node entirely (grammar uses `optional($.content)`) rather than emitting an empty one. This design has survived through 12 grammar.js commits including the addition of `escape_sequence` as a third alternative (d4e6745); the `repeat1` invariant was preserved and extended: `_content: $ => repeat1(choice($._content_plain, $.interior_sequence, $.escape_sequence))`. Mechanically checkable: `grep '_content.*repeat1' grammar.js` returns the rule with `repeat1`.
