---
id: b_6b48c00c8f34
slug: haiku-empty-ne-is-false
claim:
  kind: text
  text: >-
    An empty-array inequality, `{ col => { '!=' => [] } }`, generates SQL FALSE (0=1).
author:
  kind: agent
  id: claude-haiku-4-5
  model: claude-haiku-4-5
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 2
  refs:
    - lib/SQL/Abstract.pm:1258-1287
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.4
  asserted: 0.88
edges: []
coord: null
---

HARVESTED from claude-haiku-4-5 (no tools), asserted 0.88.
