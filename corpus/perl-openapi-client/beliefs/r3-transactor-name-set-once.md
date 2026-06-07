---
id: b_dcb156cc8085
slug: r3-transactor-name-set-once
claim:
  kind: text
  text: >-
    The Mojo::UserAgent transactor name is set to 'Mojo-OpenAPI (Perl)' exactly once in
    new(), and only when the caller did not inject a pre-built ua instance.
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
    turn: 3
  refs:
    - lib/OpenAPI/Client.pm:46
    - git:275bf6b
    - git:138fc67
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.65
edges: []
coord: null
---

At write time (2017-08-18, commit 275bf6b), `$self->ua->transactor->name('Mojo-OpenAPI (Perl)')` was called unconditionally in `new()`. The guard `unless $self->{ua}` was added later to support passing a pre-built UA. At write time this was structurally obvious: setting a User-Agent header string is a one-time initialization concern. The pattern has survived with only one change — the `unless $self->{ua}` guard added for ua injection. Checkable: lib/OpenAPI/Client.pm line 46 `unless $self->{ua}`.
