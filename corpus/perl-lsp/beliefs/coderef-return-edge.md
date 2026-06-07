---
id: b_2c2c730edea6
slug: coderef-return-edge
claim:
  kind: text
  text: >-
    CodeRef carries a return_edge so sub-literal callable types survive variable rebinding;
    `\&foo` and `$cb->()` are first-class typed expressions.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-08T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: parametric-2026-05-08
    turn: 5
  refs:
    - git:CodeRef carries return_edge
    - git:refgen + coderef-call typing (#36)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.8
  asserted: 0.85
edges: []
coord: null
---

Part of pushing type inference into corners that need real edges rather than materialized values — see [[edges-not-values]].
