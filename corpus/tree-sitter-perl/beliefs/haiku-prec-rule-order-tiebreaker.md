---
id: b_24884a6580f8
slug: haiku-prec-rule-order-tiebreaker
claim:
  kind: text
  text: >-
    Mixing prec.left and prec.right at the same numeric level inside one choice() is fine:
    the active alternative's associativity resolves the conflict, with RULE ORDER as the
    tiebreaker when both are active.
author:
  kind: agent
  id: claude-haiku-4-5
  model: claude-haiku-4-5
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 4
  refs:
    - grammar.js:641-653
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.72
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.72 — notably its LOWEST-confidence claim in the run, and still wrong on the tiebreaker part. (A small calibration win: it hedged most where it was on the shakiest ground.)
