---
id: b_c0f3b76dc397
slug: validator-is-class-global
claim:
  kind: text
  text: >-
    The JSON::Validator::Schema object returned by validator() is per-class (shared among
    all instances constructed from the same spec), not per-instance.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2017-08-18T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s1-initial-import
    turn: 3
  refs:
    - lib/OpenAPI/Client.pm:74
    - lib/OpenAPI/Client.pm:460-465
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

validator() is monkey-patched as a class method returning a closure over $schema. The POD explicitly states: 'This object global to the class, so changing it will affect all instances returned by new().'
