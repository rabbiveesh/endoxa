---
id: b_b310c9e9f03a
slug: bnb-returns-client-on-async
claim:
  kind: text
  text: >-
    Generated non-blocking (callback) methods return $self (the client), not the
    transaction, so chaining is possible. Blocking calls return the $tx directly.
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
    - lib/OpenAPI/Client.pm:93-111
    - t/client.t:84
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.87
  asserted: 0.9
edges: []
coord: null
---

_generate_method_bnb() closes over the route and returns $self when a callback is provided (lines 107-109), or $tx in the blocking case (line 107). Promise methods (_p suffix) always return a Mojo::Promise.
