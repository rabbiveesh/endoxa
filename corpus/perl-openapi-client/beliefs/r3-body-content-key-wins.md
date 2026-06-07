---
id: b_1a33188b823d
slug: r3-body-content-key-wins
claim:
  kind: text
  text: >-
    When the body param name is NOT in $params, the %content hash generators take precedence
    — meaning body data can legitimately flow through %content (e.g., json =>, body =>, xml
    =>) bypassing the $params hash entirely.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-conflict-session
    turn: 2
  refs:
    - lib/OpenAPI/Client.pm:145-151
    - lib/OpenAPI/Client.pm:208
    - t/body-validation.t:27-30
    - t/json-request.t:18-22
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: null
edges:
  - kind: attacks
    target: b_5e2149519455   # r3-body-param-key-wins
coord: null
---

The else branch of the body sub (lib/OpenAPI/Client.pm lines 145-151) scans `('body', sort keys %{$self->ua->transactor->generators})` and reads from `%content` to populate `$params->{$name}`. This means %content is a fully valid second channel for body data. These two paths ([[r3-body-param-key-wins]] and this belief) are in real tension: the API accepts body data through both `$params->{bodyParamName}` and `%content` key, but they interact silently — if a caller accidentally provides both, `$params` wins and `%content` is silently ignored for that body param. There is no FIXME but also no documentation of the precedence rule.
