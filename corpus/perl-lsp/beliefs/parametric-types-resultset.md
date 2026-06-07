---
id: b_b580692cde73
slug: parametric-types-resultset
claim:
  kind: text
  text: >-
    DBIC ResultSet column completion resolves through resultset_class discovery + RowOf
    projection emission — and the rule lives on InferredType::hash_key_class() (a property
    of the type), NOT a method-name allowlist like {search, find}.
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
    turn: 3
  refs:
    - docs/adr/parametric-types.md
    - git:feat parametric-resultset (#35)
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.9
edges:
  - kind: refines
    target: b_de13663b1e7d   # rule-no-special-casing
coord: null
---

The canonical application of [[rule-no-special-casing]]: push the 'parametric arg' behavior onto the type so consumers ask the value, never branch on the method name.
