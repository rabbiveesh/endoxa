---
id: b_28b80ab75b1e
slug: grammar-glr-conflicts-not-structural
claim:
  kind: text
  text: >-
    Genuine grammar ambiguities (e.g. preinc vs postinc, both prec 24) are resolved at PARSE
    TIME via declared GLR conflicts, not by structural exclusion: because `_term` includes
    `preinc_expression`, the parser DOES attempt `++$x` as a postinc whose operand is the
    preinc, and that branch just fails when no trailing `++` arrives. tree-sitter has NO
    rule-order tiebreaker — a same-precedence conflict with conflicting associativity is a
    build error, not a silently-ordered choice.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: ts-perl-orient-2026-06-04
    turn: 5
  refs:
    - grammar.js:196-197
    - grammar.js:510-519
    - grammar.js:669-672
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges: []
coord: null
---

Ground truth for [[haiku-postinc-structurally-impossible]] and [[haiku-prec-rule-order-tiebreaker]]. The associativity of an alternative resolves a conflict only when it's the sole active reduce at that state.
