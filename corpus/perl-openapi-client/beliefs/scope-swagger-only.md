---
id: b_0f25991294f8
slug: scope-swagger-only
kind: project-scope
claim:
  kind: text
  text: >-
    The project started as a Swagger 2.0-only client; OpenAPI v3 was not supported.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2017-08-18T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2021-02-23
  source:
    kind: conversation
    session: s1-initial-import
    turn: 1
  refs:
    - git:Imported Swagger2::Client
    - git:Converted Swagger2::Client to OpenAPI::Client
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

The codebase was imported as Swagger2::Client on 2017-08-18 and immediately renamed. Initial base_url construction read /schemes, /host, /basePath — fields present only in OpenAPI v2 (Swagger).
