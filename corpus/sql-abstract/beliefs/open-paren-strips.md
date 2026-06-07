---
id: b_315299c4f0d7
slug: open-paren-strips
claim:
  kind: text
  text: >-
    VERDICT: FALSE — it does the OPPOSITE. _open_outer_paren iteratively STRIPS outer
    parens: it matches `^\s*\((.*)\)\s*$` and sets sql=inner, repeating until no full
    wrapper remains, so `(SELECT id FROM t)` becomes `SELECT id FROM t`. The name means
    'opening up' (unwrapping), never adding.
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
    - lib/SQL/Abstract.pm:1803-1829
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_44d9b8faf359   # haiku-open-paren-adds
  - kind: attacks
    target: b_44d9b8faf359   # haiku-open-paren-adds
coord: null
---

Defeats [[haiku-open-paren-adds]]. The method name 'open' was read as 'add an opening paren' rather than 'open up / unwrap'.
