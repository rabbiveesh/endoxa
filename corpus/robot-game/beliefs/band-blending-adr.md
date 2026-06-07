---
id: b_80d57c6ade26
slug: band-blending-adr
claim:
  kind: text
  text: >-
    Band progression uses a weighted probability distribution (ADR-001), not hard levels.
    `math_band` is a center of a bell-curve spread, not a wall.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time:
    start: 2026-03-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 1
  refs:
    - docs/adr/001-band-blending.md:1-40
    - robot-buddy-domain/src/learning/challenge_generator.rs:9-54
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.93
edges:
  - kind: supports
    target: b_7858b5be6a9b   # streak-display-only
coord: null
---

ADR-001 describes three problems with hard bands (cliffs, oscillation, floor traps) and replaces them with `band_distribution(center_band, spread_width)`. Accuracy-based promotion (75% at center + 60% on stretch, min 4 attempts) replaces streak-based. Streak is display-only since this change.
