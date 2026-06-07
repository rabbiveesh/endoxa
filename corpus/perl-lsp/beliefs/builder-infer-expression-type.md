---
id: b_3f25aea81063
slug: builder-infer-expression-type
claim:
  kind: text
  text: >-
    Type inference is performed by Builder::infer_expression_type plus
    Builder.resolved_returns, walking expressions at build time and storing materialized
    InferredTypes.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-03-08T12:00:00Z
  valid_time:
    start: 2026-03-08
    end: 2026-04-27
  source:
    kind: conversation
    session: typeinfer-2026-03-08
    turn: 4
  refs:
    - git:operator-based type inference
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.8
edges: []
coord: null
---

The original type-inference design. Correct for its era; entirely replaced by the witness bag — CLAUDE.md now reads 'Builder::infer_expression_type is gone', 'Builder.resolved_returns is gone'. Kept for reliving the pre-bag architecture.
