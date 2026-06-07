---
id: b_ccddd82d775f
slug: move-resolver-intent-order-priority
claim:
  kind: text
  text: >-
    The `resolve_moves` function gives priority to the first intent when two entities target
    the same tile. The caller should pass intents in (Player, Sparky, NPCs) order so the
    player has frame-priority.
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
    - robot-buddy-domain/src/world/movement.rs:113-117
    - robot-buddy-domain/src/world/movement.rs:148-155
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

movement.rs comment: 'Order of intents matters: when two intents target the same tile, the one processed first wins and the later one is Blocked against it.' The `reserved` HashSet implements this: once a tile is reserved by a granted move, later intents into that tile are blocked.
