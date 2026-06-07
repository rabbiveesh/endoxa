---
id: b_c0fbd489b474
slug: client-side-error-detection
claim:
  kind: text
  text: >-
    Callers can distinguish whether an error came from the client-side validator or from the
    server by checking $tx->remote_address; it is set only for genuine HTTP responses.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2021-02-23T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-v3-api
    turn: 2
  refs:
    - t/client.t:76-80
    - git:Add test to check if the error was on the client side or not
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.82
  asserted: 0.85
edges: []
coord: null
---

The test at t/client.t line 76 asserts ok $tx->remote_address for a server response, and line 80 asserts ok !$tx->remote_address for a client-side validation error. This pattern was codified in the 0.25 release.
