---
id: b_d8a1afd302bf
slug: after-build-tx-event
claim:
  kind: text
  text: >-
    The after_build_tx event fires after request construction and validation; the $tx at
    that point may be a synthetic error tx or a real one. The operationId is stored in
    $tx->req->env->{operationId}.
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
    - lib/OpenAPI/Client.pm:211-212
    - lib/OpenAPI/Client.pm:361-374
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.83
  asserted: 0.85
edges: []
coord: null
---

_build_tx() always calls $self->emit(after_build_tx => $tx) at the end, whether or not validation succeeded. This is stated EXPERIMENTAL in the docs.
