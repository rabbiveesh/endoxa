---
id: b_65e0731b36ca
slug: r3-seed-fragile
claim:
  kind: text
  text: >-
    Hand-picked seeds break silently if any rng.gen() call is re-ordered before the tested
    path, hanging tests in wait_until
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
    turn: 2
  refs:
    - docs/adr/002-headless-test-harness.md:64
    - robot-buddy-game/tests/story.rs:80-83
    - git:5045e750c1798cc44eeb634d09487a87be89e4fc
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.75
  asserted: 0.9
edges:
  - kind: attacks
    target: b_379fef90b1ed   # r3-seed-stable
coord: null
---

ADR-002 explicitly documents: 'Hand-picked seeds (e.g. seed 0 for "talk rolls a challenge") are fragile to RNG-flow changes. If we re-order any rng.gen() call before the talk path, the seed picks a different draw and the test hangs in wait_until.' The mitigation note says 'revisit by injecting a "force challenge" hook if it bites.' As of 2026-06-04 no such hook exists in the codebase (grep for force_challenge or challenge_hook returns nothing). story.rs line 80 explicitly warns: 'If the random behavior changes, find a new seed via a scratch test.' This attacks [[r3-seed-stable]]: the same simplicity that makes seeds appealing makes them invisible breakage vectors on refactor.
