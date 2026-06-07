---
id: b_db0921d6a22a
slug: resolve-yields-3-stayed
claim:
  kind: text
  text: >-
    VERDICT: FALSE. resolve_moves returns 3 resolutions — the test
    pushable_grants_pushee_and_pusher_when_destination_is_free asserts len==3. The
    pushed_this_frame check does NOT skip appending: it pushes MoveResolution::Stayed for
    the pushee's own intent and then continues, so a third entry r[2]=Stayed{Npc(1)} always
    appears.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 3
  refs:
    - robot-buddy-domain/src/world/movement.rs:480-488
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_274158ede4fd   # haiku-resolve-yields-2
  - kind: attacks
    target: b_274158ede4fd   # haiku-resolve-yields-2
coord: null
---

Defeats [[haiku-resolve-yields-2]]. 'No-op' meant no movement, not no output entry — the Stayed resolution is still pushed.
