---
id: b_02a4bd6c4fc7
slug: r3-band-blending
claim:
  kind: text
  text: >-
    Difficulty is a weighted probability distribution centered on mathBand, not a hard level
    — the center is a magnet, not a wall
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2026-03-26
    end: 2999-01-01
  source:
    kind: conversation
    session: r3-survivors-session
    turn: 2
  refs:
    - robot-buddy-domain/src/learning/challenge_generator.rs:469
    - robot-buddy-domain/src/learning/learner_profile.rs:16
    - docs/adr/001-band-blending.md
    - git:0509efc947da68282e8b28236419c6d4a579f664
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.62
  asserted: 0.68
edges: []
coord: null
---

Commit 0509efc (2026-03-26) implemented band blending. At write time this was inferable from the ADR-001 spec (a4c86ff, same day) which documented three observed problems with hard bands: cliffs, oscillation, and floor traps, all verified by CLI simulator QA. The design was inferred from the band-blending spec before and independently of the implementation: distribution center + spread_width dial was the natural solution to all three pathologies simultaneously. The assumption survived the complete Rust rewrite (ac48738, 2026-04-26) — challenge_generator.rs contains band_distribution() and sample_from_distribution(), LearnerProfile still has spread_width: f64, and should_promote() still gates on accuracy_at_band and accuracy_above_band. Checkable: grep spread_width in robot-buddy-domain/src/ returns >10 hits.
