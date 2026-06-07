---
id: b_26e21c30dc62
slug: subclass-hardwired-to-base
claim:
  kind: text
  text: >-
    Before 1.04, dynamically-generated subclasses were hardwired to inherit from
    OpenAPI::Client itself (via a $BASE constant), making it impossible to subclass
    OpenAPI::Client and have the generated methods belong to the subclass.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2022-06-03T12:00:00Z
  valid_time:
    start: 2017-08-18
    end: 2022-06-03
  source:
    kind: conversation
    session: s4-inheritance-fix
    turn: 1
  refs:
    - git:Allow inheritance and roles to be applied before new() #35, #37
    - Changes:27-29
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

Pre-1.04 code used my $BASE = __PACKAGE__ and generated eval'd packages with 'use Mojo::Base $BASE'. When a user subclassed OpenAPI::Client (e.g. OpenAPI::Child), the generated class still pointed at the base, not the child.
