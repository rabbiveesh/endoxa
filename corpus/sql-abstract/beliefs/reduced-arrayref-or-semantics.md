---
id: b_cd40eb296062
slug: reduced-arrayref-or-semantics
claim:
  kind: text
  text: >-
    The consistent rule throughout SQL::Abstract is: arrayrefs = OR, hashrefs = AND. This is
    the single mental model that explains all WHERE clause behavior, with `-and`/`-or`
    modifiers as the escape hatch.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2008-06-12T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s-2008-refine
    turn: 13
  refs:
    - lib/SQL/Abstract.pm:2896-2898
  derived_from:
    - b_b71632d3ce34
    - b_1bb81fca0c21
    - b_2bdbe1e6a9d1
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.95
  asserted: null
edges:
  - kind: derived_from
    target: b_b71632d3ce34   # arrayref-means-or
  - kind: derived_from
    target: b_1bb81fca0c21   # hashref-multi-key-and
  - kind: derived_from
    target: b_2bdbe1e6a9d1   # project-scope-v1-hash-and
coord: null
---

Synthesized from arrayref-means-or, hashref-multi-key-and, and the POD at line 2896. The insert-sorts-hash-keys belief is consistent with hash-always-AND. The edge cases all come from trying to express AND in an array context or OR in a hash context.
