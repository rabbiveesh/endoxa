---
id: b_c8771348ea24
slug: scope-v2-only-api
kind: project-scope
claim:
  kind: text
  text: >-
    The project supports OpenAPI v2 only; the JSON::Validator API is used directly without
    schema-type dispatch.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2017-08-19T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2021-02-22
  source:
    kind: conversation
    session: s2-mojo-command
    turn: 1
  refs:
    - git:Imported Swagger2::Client
    - Changes:43-46
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.8
  asserted: 0.82
edges:
  - kind: supersedes
    target: b_0f25991294f8   # scope-swagger-only
coord: null
---

Between 0.01 and 0.25, the code used JSON::Validator directly and manually traversed /paths. _generate_class() iterated paths with bundle() or manual traversal, not schema->routes.
