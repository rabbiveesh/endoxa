---
id: b_260d8df0babe
slug: subprocess-isolation-removed
claim:
  kind: text
  text: >-
    VERDICT: subprocess isolation was REMOVED (#14). The IPC + serialization overhead was
    real and the crash-isolation worry never materialized; in-process parsing with Rayon
    (274-file Mojolicious in 204ms) is both faster and simpler.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-24T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: cli-tokens-2026-03-24
    turn: 8
  refs:
    - git:workspace indexing + subprocess removal (#14)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_272e373c55f8   # subprocess-isolation-needed
  - kind: attacks
    target: b_272e373c55f8   # subprocess-isolation-needed
coord: null
---

Defeats [[subprocess-isolation-needed]]. The robustness fear cost measurable latency and bought nothing observable — a hunch that traded real cost for an imagined risk.
