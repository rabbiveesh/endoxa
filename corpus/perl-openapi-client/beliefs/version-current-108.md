---
id: b_f146cf93e08d
slug: version-current-108
claim:
  kind: text
  text: >-
    The current released version in the codebase is 1.08; version 1.09 is in progress (not
    yet released).
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s6-deepobject-fix
    turn: 1
  refs:
    - lib/OpenAPI/Client.pm:12
    - Changes:1-8
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.62
  asserted: 0.72
edges: []
coord: null
---

lib/OpenAPI/Client.pm sets $VERSION = '1.08' and Changes lists '1.09 Not Released' with the deepObject fix.
