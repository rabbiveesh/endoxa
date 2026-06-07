---
id: b_adb38a56f140
slug: reducer-consensus-immutable-domain
kind: project-scope
claim:
  kind: text
  text: >-
    All game domain state is immutable and event-sourced: every change produces a new state
    value rather than mutating the old one. This is enforced by Rust ownership, not
    discipline.
author:
  kind: reducer
  id: reducer-sonnet
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time:
    start: 2026-03-25
    end: 2999-01-01
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 6
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:177
    - robot-buddy-domain/src/learning/rolling_window.rs:242-247
  derived_from:
    - b_d6278b451ab5
    - b_7858b5be6a9b
    - b_934e40f551e8
confidence:
  directness: reduced
  observation_count: 3
  source_weight: 0.93
  asserted: null
edges:
  - kind: derived_from
    target: b_d6278b451ab5   # all-randomness-seeded
  - kind: derived_from
    target: b_7858b5be6a9b   # streak-display-only
  - kind: derived_from
    target: b_934e40f551e8   # rolling-window-size-20
coord: null
---

Converged from: [[all-randomness-seeded]], [[streak-display-only]], and [[rolling-window-size-20]]. Rust ownership means the old `LearnerProfile` is moved into `learner_reducer`; returning a new struct guarantees the old is gone. Tests verify immutability explicitly (`immutability` tests in rolling_window and operation_stats test modules).
