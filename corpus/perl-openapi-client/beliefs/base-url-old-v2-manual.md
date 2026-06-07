---
id: b_9c4bec64561c
slug: base-url-old-v2-manual
claim:
  kind: text
  text: >-
    Before 1.01, base_url was computed by manually reading /schemes, /host, /basePath from
    the validator — this broke silently for OpenAPI v3 specs which use 'servers' instead.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2021-02-23T12:00:00Z
  valid_time:
    start: 2021-02-23
    end: 2021-06-17
  source:
    kind: conversation
    session: s3-v3-api
    turn: 2
  refs:
    - git:Fix generating correct base URL for OpenAPIv3 schemas, closes #31
    - Changes:40-41
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

The pre-1.01 base_url implementation accessed $schema->get('/schemes'), /host, /basePath. OpenAPI v3 puts the base URL under 'servers[0].url', so the old code produced a wrong default URL.
