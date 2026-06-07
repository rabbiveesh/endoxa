---
id: b_fe951d3221fa
slug: validation-errors-return-400-tx
claim:
  kind: text
  text: >-
    When client-side validation of request parameters fails, no HTTP request is sent;
    instead _build_tx() synthesizes a fake Mojo::Transaction::HTTP with a 400 status code
    and a JSON body containing the errors.
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
    - lib/OpenAPI/Client.pm:196-205
    - t/client.t:66-80
    - git:Fix setting correct request method on client errors
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.93
edges:
  - kind: supports
    target: b_c0fbd489b474   # client-side-error-detection
coord: null
---

_build_tx() builds a synthetic $tx with res->code(400), res->body encoded as JSON, and res->error set. The req->method is also set correctly on the synthetic tx (fixed in commit f43fe79). Callers can distinguish client-side errors from server-side by checking $tx->remote_address.
