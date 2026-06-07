---
id: b_9d578ca2170a
slug: transitive-parents-ends-on-empty-stack
claim:
  kind: text
  text: >-
    VERDICT: partly wrong — the COUNT is right (21 out, depth=21) but the MECHANISM is
    wrong: with no grandparents the stack empties after 21 pops, so `while let Some(p) =
    stack.pop()` ends normally; the `if depth > 20 { break; }` guard NEVER fires here. Right
    answer, wrong reason — at 0.99 confidence.
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
    - src/builder.rs:2288
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_4a940bb9f90d   # haiku-transitive-parents-break-guard
  - kind: attacks
    target: b_4a940bb9f90d   # haiku-transitive-parents-break-guard
coord: null
---

Defeats [[haiku-transitive-parents-break-guard]]. The instructive bit: a belief can be output-correct and mechanism-wrong, and asserted confidence flags neither. A reducer scoring only the final answer would mark this 'correct' and miss that the model didn't understand the loop.
