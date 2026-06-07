---
id: b_b3b7057c1abf
slug: entry-point-client-pm
claim:
  kind: text
  text: >-
    The single entry-point source file for the library is lib/OpenAPI/Client.pm; there is
    also lib/Mojolicious/Command/openapi.pm for the CLI.
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
    - lib/OpenAPI/Client.pm:1
    - lib/Mojolicious/Command/openapi.pm:1
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.58
  asserted: 0.7
edges: []
coord: null
---

find output confirms exactly two .pm files in lib/. The main package and a Mojolicious command.
