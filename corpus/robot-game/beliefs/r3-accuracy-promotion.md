---
id: b_521a4f285152
slug: r3-accuracy-promotion
claim:
  kind: text
  text: >-
    Band promotion is accuracy-only; streak is a display counter with zero mechanical effect
    on promotion or demotion
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
    turn: 3
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:201-225
    - docs/adr/001-band-blending.md:29
    - git:925262b23c4605ff69415a2517c8bb033829bda9
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.63
  asserted: 0.7
edges: []
coord: null
---

Commit 925262b (2026-03-26) removed streak-based promotion ('Kill streak-based promotion, accuracy-only with CRA ramp on stretch'). At write time this was inferable from ADR-001 which states 'Streak is display-only. Kids like seeing streaks, Sparky celebrates them, but streaks have zero mechanical effect on band progression.' The decision was justified by band blending eliminating the cliff problem that streaks were originally solving. Checkable in current Rust: learner_profile.rs lines 201-219 show streak updated for display but not read by should_promote() or should_demote(); the doc comment at line 201 says 'Streak (display only)'. Survived 125 commits from 925262b through 2026-06-04 with no regression.
