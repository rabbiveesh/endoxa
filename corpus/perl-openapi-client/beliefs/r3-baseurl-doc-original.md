---
id: b_29dc556eec80
slug: r3-baseurl-doc-original
claim:
  kind: text
  text: >-
    The 'Custom server URL' POD originally showed only the in-place mutation style:
    $client->base_url->host('other.server.com') with no constructor-time string form.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2021-08-28
    end: 2021-09-22
  source:
    kind: conversation
    session: r3-vov-session
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:287
    - git:d1655f7
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: null
edges: []
coord: null
---

Before PR #33, the POD for the 'Custom server URL' section only documented `$client->base_url->host(...)` to mutate the URL after construction. There was no mention of passing `base_url =>` as a string or Mojo::URL to the constructor. This was the documented prior art that PR #33 aimed to correct.
