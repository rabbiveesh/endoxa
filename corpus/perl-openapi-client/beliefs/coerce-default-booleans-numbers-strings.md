---
id: b_0978b9ed0f02
slug: coerce-default-booleans-numbers-strings
claim:
  kind: text
  text: >-
    The default coerce setting is 'booleans,numbers,strings', applied via JSON::Validator's
    coerce() at class-generation time.
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
    turn: 3
  refs:
    - lib/OpenAPI/Client.pm:61
    - t/client.t:74-75
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

In _generate_class(), $jv->coerce($attrs->{coerce} // 'booleans,numbers,strings') is called before loading the schema. This means string '5' for an integer param is coerced silently — the test at t/client.t line 74 exercises this.
