---
id: b_a06464090f5f
slug: hunch-chevron-stack-overflow-implied-one
claim:
  kind: text
  text: >-
    Nesting interior sequences beyond 8 levels is silently mishandled: chevron_count_top
    returns 1 (not the actual opening count) for depth >= MAX_NESTED_CHEVRONS.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-03-03T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-chevrons-2023-03
    turn: 3
  refs:
    - src/scanner.c:51-57
    - src/scanner.c:102-107
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.52
  asserted: 0.6
edges: []
coord: null
---

scanner.c lines 102-107: chevron_count_top returns 1 if nchevrons >= MAX_NESTED_CHEVRONS. A sequence opened with C<<<< (4 chevrons) at depth 9 would be closed by a single '>' rather than 4 '>', silently producing wrong parse. This is an intentional simplification given the rarity of such nesting.
