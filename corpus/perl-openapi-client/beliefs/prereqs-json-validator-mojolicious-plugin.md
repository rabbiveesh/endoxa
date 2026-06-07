---
id: b_c5dbebcd6cad
slug: prereqs-json-validator-mojolicious-plugin
claim:
  kind: text
  text: >-
    Runtime dependencies are JSON::Validator >= 5.09 and Mojolicious::Plugin::OpenAPI >=
    5.05; the plugin is required even for client-only use because it provides the
    validate_request() implementation on the schema object.
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
    turn: 2
  refs:
    - Makefile.PL:11
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

Makefile.PL PREREQ_PM lists exactly these two. JSON::Validator provides the schema parsing and validate_request; the Plugin::OpenAPI dependency ensures the right version of the JSON::Validator schema extensions is available.
