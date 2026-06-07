---
id: b_379fef90b1ed
slug: r3-seed-stable
claim:
  kind: text
  text: >-
    Hand-picked integer seeds give story tests deterministic, readable, and fast determinism
    without extra infrastructure
author:
  kind: reducer
  id: reducer-opus
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-seeds-session
    turn: 1
  refs:
    - robot-buddy-game/tests/common/mod.rs:32-34
    - robot-buddy-game/tests/story.rs:80-83
    - docs/adr/002-headless-test-harness.md:64
    - git:5045e750c1798cc44eeb634d09487a87be89e4fc
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.6
  asserted: 0.7
edges: []
coord: null
---

ADR-002 (commit 5045e75) resolved on using `Harness::new(seed: u64)` backed by `SmallRng::seed_from_u64`. Tests use specific seeds (0, 1, 7, 11, 42, etc.) that produce known gameplay paths: seed 0 makes talking to Sparky roll a challenge, seed 1 produces a wandering NPC in the push corridor, etc. The design note says: 'acceptable until many such tests exist'. Currently story.rs has 20+ test functions across those seeds. The simplicity is the upside being asserted.
