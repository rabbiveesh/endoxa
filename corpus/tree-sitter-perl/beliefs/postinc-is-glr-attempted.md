---
id: b_cfb3dcd26166
slug: postinc-is-glr-attempted
claim:
  kind: text
  text: >-
    VERDICT: the conclusion is right but the reasoning is FALSE. postinc_expression is NOT
    structurally impossible for `++$x`: since `_term` includes `preinc_expression`, the GLR
    parser actively pursues a postinc whose operand is the whole preinc `++$x`, waiting for
    a trailing `++`/`--`; it fails only because none arrives. The conflict declaration at
    line 197 exists precisely because the ambiguity is real and must be resolved at runtime.
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
    turn: 3
  refs:
    - grammar.js:510-519
    - grammar.js:196-197
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_3aa107a163c7   # haiku-postinc-structurally-impossible
  - kind: attacks
    target: b_3aa107a163c7   # haiku-postinc-structurally-impossible
  - kind: supports
    target: b_28b80ab75b1e   # grammar-glr-conflicts-not-structural
coord: null
---

Defeats [[haiku-postinc-structurally-impossible]]; supports [[grammar-glr-conflicts-not-structural]]. Right-answer-wrong-reason at 0.95 — the model reasoned about token shape and never modeled GLR's parallel-stack exploration (out-of-snippet knowledge).
