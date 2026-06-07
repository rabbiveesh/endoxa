---
id: b_d5bc7d76f66e
slug: r3-ua-lazy-singleton
claim:
  kind: text
  text: >-
    The `ua` attribute has been a lazy singleton (one Mojo::UserAgent per client instance,
    created on first access) since the initial 2017-08-18 conversion commit.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:23
    - git:275bf6b
    - git:810f532
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

At write time (2017-08-18, commit 275bf6b), the `has ua => sub { Mojo::UserAgent->new }` pattern was inferred to be intentional: one UA per client instance, constructed lazily. This was structurally obvious — a per-instance lazy builder with no shared state. It has survived 130 commits including the inheritance refactor (810f532, 2022-06-03), the `ua` injection feature (138fc67, 2026-06-04 via `$client->ua` argument in `new()`), and the `collectionFormat` work without change to the fundamental pattern. Checkable: the current code still has `has ua => sub { Mojo::UserAgent->new }` at lib/OpenAPI/Client.pm line 23.
