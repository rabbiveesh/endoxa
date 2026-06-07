---
id: b_7ee008de39f7
slug: content-hash-is-plain
claim:
  kind: text
  text: >-
    The composer.lock content-hash is a straightforward hash of composer.json bytes that
    composr can recompute directly.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-05-07T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: installer-2026-05-07
    turn: 15
  refs:
    - git:Add `install` subcommand
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.55
  asserted: 0.7
edges: []
coord: null
---

A reasonable-looking assumption while wiring the lock check. Wrong about composer's actual algorithm.
