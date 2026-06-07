---
id: b_934e40f551e8
slug: rolling-window-size-20
claim:
  kind: text
  text: >-
    The `RollingWindow` is fixed at `max_size = 20` entries for a new `LearnerProfile`.
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
    turn: 2
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:65
    - robot-buddy-domain/src/learning/rolling_window.rs:53-63
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.85
  asserted: 0.87
edges: []
coord: null
---

`LearnerProfile::new()` calls `RollingWindow::new(20)`. The window is immutable — `push` returns a new `RollingWindow`, dropping the oldest entry when size exceeds `max_size`.
