---
id: b_d6278b451ab5
slug: all-randomness-seeded
claim:
  kind: text
  text: >-
    All randomness in domain functions is seeded. Domain functions take `&mut impl Rng`;
    tests use `SmallRng::seed_from_u64(42)`. There is no global RNG state.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 1
  refs:
    - CLAUDE.md:26
    - robot-buddy-game/src/main.rs:17
    - robot-buddy-domain/src/learning/challenge_generator.rs:56-66
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.91
edges: []
coord: null
---

CLAUDE.md invariant #3. In main.rs, `let seed = macroquad::rand::rand() as u64` seeds `Game::new(seed)`. `generate_challenge` and `band_distribution` sampling both accept an explicit `&mut impl Rng`.
