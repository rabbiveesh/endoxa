---
id: b_94a6984407b9
slug: hunch-no-response-validation
claim:
  kind: text
  text: >-
    The client probably does not validate HTTP responses against the OpenAPI spec's response
    schemas — only request parameters are validated.
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
    turn: 5
  refs:
    - lib/OpenAPI/Client.pm:127-194
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.5
  asserted: 0.6
edges: []
coord: null
---

Reading _build_tx() there is only a validate_request() call. No validate_response() call is visible anywhere. Response validation would be the server plugin's job (Mojolicious::Plugin::OpenAPI).
