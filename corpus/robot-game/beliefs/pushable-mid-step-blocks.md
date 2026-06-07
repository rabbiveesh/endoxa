---
id: b_b0bb15201da3
slug: pushable-mid-step-blocks
claim:
  kind: text
  text: >-
    A pushable NPC that is mid-step (moving_to is Some) cannot be pushed even when pressure
    threshold is met. The pusher is blocked until the pushee settles.
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
    - robot-buddy-domain/src/world/movement.rs:275-277
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges: []
coord: null
---

`try_push` checks `if pushee_state.moving_to.is_some() { return false; }`. The comment says 'Mid-step pushees: their tile bookkeeping makes a clean push ambiguous. Wait until they settle.' This is a deliberate simplification — no chain pushes either.
