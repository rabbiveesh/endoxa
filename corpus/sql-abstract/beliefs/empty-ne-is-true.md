---
id: b_f64fb35d9b11
slug: empty-ne-is-true
claim:
  kind: text
  text: >-
    VERDICT: inverted. _dwim_op_to_is() returns 0 for an inequality op, and the ternary is
    `... ? sqlfalse : sqltrue`, so 0 selects sqltrue — an empty `!=` array generates SQL
    TRUE (1=1), not FALSE. (The result is unintuitive, but the opposite of the claim.)
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
    - lib/SQL/Abstract.pm:1276-1285
    - lib/SQL/Abstract.pm:1319-1321
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_6b48c00c8f34   # haiku-empty-ne-is-false
  - kind: attacks
    target: b_6b48c00c8f34   # haiku-empty-ne-is-false
coord: null
---

Defeats [[haiku-empty-ne-is-false]]. Right that it's surprising, wrong on direction — a sign-flip on the ternary.
