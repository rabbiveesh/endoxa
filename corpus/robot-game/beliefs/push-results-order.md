---
id: b_ecd51ab0b870
slug: push-results-order
claim:
  kind: text
  text: >-
    When a push succeeds, the pushee's `Granted` resolution is appended to `resolutions`
    BEFORE the pusher's `Granted`. The pushee's own `Stay` intent later collapses to
    `Stayed`.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-05-05T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s5-npcs
    turn: 2
  refs:
    - robot-buddy-domain/src/world/movement.rs:220-231
    - robot-buddy-domain/src/world/movement.rs:480-488
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.87
edges:
  - kind: supports
    target: b_b0bb15201da3   # pushable-mid-step-blocks
coord: null
---

In `resolve_moves`, `try_push` appends the pushee's Granted directly to `resolutions`, then `pushed_this_frame.insert(other_id)` prevents re-processing. When the pushee's own intent arrives, the `pushed_this_frame.contains(id)` guard makes it `Stayed`. The test `pushable_grants_pushee_and_pusher_when_destination_is_free` asserts `r.len() == 3` in the order [pushee Granted, pusher Granted, pushee Stayed].
