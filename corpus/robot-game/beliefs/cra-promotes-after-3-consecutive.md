---
id: b_bb0f2c76b427
slug: cra-promotes-after-3-consecutive
claim:
  kind: text
  text: >-
    CRA stage advances after 3 consecutive no-hint, no-told-me correct answers at the same
    CRA stage for the same operation.
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
    turn: 3
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:156-167
    - robot-buddy-domain/src/learning/learner_profile.rs:259-265
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.87
  asserted: 0.88
edges:
  - kind: supports
    target: b_222217a3c839   # cra-per-operation
coord: null
---

`count_consecutive_no_hint_correct` iterates the window in reverse and breaks on any other operation, on a wrong answer, on hint_used, told_me, or a different cra_level_shown. Promotion fires at count >= 3.
