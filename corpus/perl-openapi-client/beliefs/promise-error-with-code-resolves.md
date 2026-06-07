---
id: b_276bba8f362d
slug: promise-error-with-code-resolves
claim:
  kind: text
  text: >-
    In the _p (promise) methods, if validation fails and the synthetic error tx has an HTTP
    code (i.e., code 400), the promise RESOLVES with the tx rather than rejecting — only
    errors without an HTTP code cause rejection.
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
    turn: 3
  refs:
    - lib/OpenAPI/Client.pm:119-124
    - t/client.t:92-97
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

_generate_method_p() lines 120-123: returns reject only if !$err->{code}. For client-side 400 errors the tx has both message and code=400, so the promise resolves. This is a subtle semantic: promise rejection is reserved for networking errors or unknown operationIds.
