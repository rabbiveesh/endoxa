---
id: b_35361080bb8d
slug: call-croaks-no-such-operation
claim:
  kind: text
  text: >-
    call() croaks with '[OpenAPI::Client] No such operationId' for unknown operation IDs;
    call_p() rejects the promise instead of croaking.
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
    turn: 4
  refs:
    - lib/OpenAPI/Client.pm:25-35
    - t/client.t:103-121
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

call() uses Carp::croak on unknown ops. call_p() uses Mojo::Promise->reject. See lines 27-28 and 32-33.
