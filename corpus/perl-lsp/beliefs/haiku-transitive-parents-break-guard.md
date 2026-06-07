---
id: b_4a940bb9f90d
slug: haiku-transitive-parents-break-guard
claim:
  kind: text
  text: >-
    In transitive_parents, with 21 direct parents and no grandparents, exactly 21 parents
    appear and the loop terminates via the `if depth > 20 { break; }` guard firing on
    iteration 22.
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
    turn: 2
  refs:
    - src/builder.rs:2288
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.99
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.99 — its single most confident claim in the run, and wrong on mechanism.
