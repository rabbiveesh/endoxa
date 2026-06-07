---
id: b_977f6b2eed5d
slug: edges-not-values
claim:
  kind: text
  text: >-
    Bag invariant: mirror facts between attachments via Edge(target), NEVER re-push a
    materialized InferredType onto an edge-reachable attachment as a 'cache' — the
    registry's edge-chase IS the canonical flow; a parallel materialized store drifts.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: nav-graph-2026-06-04
    turn: 4
  refs:
    - CLAUDE.md
    - docs/adr/bag-canonical.md
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: refines
    target: b_e9860602b115   # witness-bag-canonical
coord: null
---

Refines [[witness-bag-canonical]]. Witnesses are monotone (append-only); termination follows from the finite InferredType lattice + a snapshot check. Re-emittable passes clear-and-emit by source tag.
