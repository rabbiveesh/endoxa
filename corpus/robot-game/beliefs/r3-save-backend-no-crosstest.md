---
id: b_9c275ed85811
slug: r3-save-backend-no-crosstest
claim:
  kind: text
  text: >-
    LocalStorageBackend and InMemoryBackend share no automated cross-implementation contract
    test, relying on behavioral simplicity instead
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 4
  refs:
    - robot-buddy-game/src/save.rs:111-118
    - robot-buddy-game/src/save.rs:166-187
    - docs/adr/002-headless-test-harness.md:70
    - git:5045e750c1798cc44eeb634d09487a87be89e4fc
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.55
  asserted: 0.6
edges: []
coord: null
---

ADR-002 (5045e75, 2026-04-26) documents: 'No automated cross-impl test today; the current behavior is simple enough that drift is unlikely.' At write time this was inferable from save.rs: both implementations handle slot >= 3 identically (skip/ignore), missing-slot returns None, and timestamp semantics diverge intentionally (InMemoryBackend always writes 0, LocalStorageBackend writes a real epoch). The gap is structurally present: SaveBackend trait has three methods, no blanket test exercises both impls against a shared spec. Survived 30+ commits since ADR-002 was written. Checkable: grep for cross-impl or backend parametrized test in robot-buddy-game/tests/ returns nothing.
