---
id: b_78e46b3ab1d7
slug: no-rule-order-tiebreaker
claim:
  kind: text
  text: >-
    VERDICT: FALSE on the tiebreaker. tree-sitter has NO rule-order tiebreaker anywhere. A
    single active reduce's associativity resolves a shift/reduce cleanly, but if two same-
    precedence alternatives with conflicting associativity are both active reduces, the `_
    => {}` arm in build_parse_table.rs fires (does nothing) and the conflict is left
    UNRESOLVED — a grammar build error, not an order-decided pick.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 5
  refs:
    - ~/personal/tree-sitter/crates/generate/src/build_tables/build_parse_table.rs:786-800
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_24884a6580f8   # haiku-prec-rule-order-tiebreaker
  - kind: attacks
    target: b_24884a6580f8   # haiku-prec-rule-order-tiebreaker
  - kind: supports
    target: b_28b80ab75b1e   # grammar-glr-conflicts-not-structural
coord: null
---

Defeats [[haiku-prec-rule-order-tiebreaker]]; supports [[grammar-glr-conflicts-not-structural]]. Refuting this required reading the tree-sitter CORE repo (the adjudicator cross-repo'd to build_parse_table.rs) — the weak model couldn't have known it from grammar.js alone.
