---
id: b_ba80bb9ef6f1
slug: base-url-current-delegate
claim:
  kind: text
  text: >-
    Since 1.01, base_url delegates to validator->base_url->clone if the validator supports
    it, falling back to defaults of http://localhost; this correctly handles both v2 and v3
    specs.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2021-02-23T12:00:00Z
  valid_time:
    start: 2021-06-17
    end: 2999-01-01
  source:
    kind: conversation
    session: s3-v3-api
    turn: 2
  refs:
    - lib/OpenAPI/Client.pm:14-21
    - git:Fix generating correct base URL for OpenAPIv3 schemas, closes #31
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.92
edges:
  - kind: supersedes
    target: b_9c4bec64561c   # base-url-old-v2-manual
coord: null
---

lib/OpenAPI/Client.pm lines 14-21 show the current base_url lazy builder. It checks ->can('base_url') to stay safe if the schema object doesn't support it.
