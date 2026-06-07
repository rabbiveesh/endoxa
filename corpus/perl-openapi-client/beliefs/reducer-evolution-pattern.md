---
id: b_a7a084c4aa70
slug: reducer-evolution-pattern
kind: project-scope
claim:
  kind: text
  text: >-
    The project's evolution follows a consistent pattern: a feature is added for v2 only,
    then breaks or silently fails for v3, then a fix commit addresses the v3 case.
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s6-deepobject-fix
    turn: 3
  refs:
    - Changes:1-46
  derived_from:
    - b_9c4bec64561c
    - b_e0aab0b3044c
confidence:
  directness: reduced
  observation_count: 2
  source_weight: 0.78
  asserted: null
edges:
  - kind: derived_from
    target: b_9c4bec64561c   # base-url-old-v2-manual
  - kind: derived_from
    target: b_e0aab0b3044c   # deepobject-name-undef-protocol
coord: null
---

Derived from [[base-url-old-v2-manual]] (v3 base URL broken until 1.01) and [[deepobject-name-undef-protocol]] (v3 deepObject broken until 1.09). Both are cases where the v2 path worked but the v3 path exposed missing handling.
