---
id: b_05260076e447
slug: collection-format-type-check-bug
claim:
  kind: text
  text: >-
    The initial collectionFormat implementation in 1.05 would die with 'Use of uninitialized
    value' when a param had no 'type' key, because it used $param->{type} eq 'array' without
    a defined check.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2022-08-17T12:00:00Z
  valid_time:
    start: 2022-08-17
    end: 2022-09-11
  source:
    kind: conversation
    session: s5-collection-format
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:217-225
    - git:Fix "Use of uninitialized value" warning, closes #39
    - git:Add support for "collectionFormat" for query params, closes #38
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges: []
coord: null
---

Commit 6aeb6c8 (1.05) added _coerce_collection_format with: ($param->{type} eq 'array' ? 'csv' : ''). Commit e2434a3 (1.06) fixed this to: +(($param->{type} // '') eq 'array' ? 'csv' : '') adding the // '' guard.
