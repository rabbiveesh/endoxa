---
id: b_ec614ecca420
slug: promote-needs-4-attempts
claim:
  kind: text
  text: >-
    Band promotion requires at least 4 attempts at the center band in the rolling window,
    not just hitting the accuracy threshold.
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
    - robot-buddy-domain/src/learning/learner_profile.rs:141-154
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

`should_promote` returns false when `at_count < 4`. Similarly `should_demote` requires `at_count >= 4`. This prevents premature promotion/demotion on sparse data.
