---
id: b_f81212330778
slug: follower-phases-through-entities
claim:
  kind: text
  text: >-
    Player followers (Sparky, companion NPC) have `phase_through_entities: true`, letting
    them retrace the player's path through wandering NPCs without getting wedged. Walls
    still block them.
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
    turn: 1
  refs:
    - robot-buddy-domain/src/world/movement.rs:78-85
    - robot-buddy-domain/src/world/movement.rs:614-628
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.87
  asserted: 0.88
edges: []
coord: null
---

EntityState.phase_through_entities: 'lets a mover ignore other entities on the destination tile. Walls and out-of-bounds still block. Used for the player's followers so they can retrace the player's path without getting wedged behind a wandering NPC.' Tests confirm phasing movers are blocked by walls.
