---
id: b_7dd83209e6f8
slug: wrong-transactor-method-name
claim:
  kind: text
  text: >-
    The documentation for the custom content generator example used add_generators()
    (plural) which does not exist on Mojo::UserAgent::Transactor.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2017-08-19T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s2-mojo-command
    turn: 2
  refs:
    - git:fix the method name called on the ua transactor
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.85
edges: []
coord: null
---

This was an inferred correct API call but was wrong: Mojo::UserAgent::Transactor exposes add_generator() (singular). The doc example silently showed the wrong method name.
