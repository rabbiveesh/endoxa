---
id: b_4ba316e49888
slug: frustration-high-drops-band
claim:
  kind: text
  text: >-
    A `FrustrationDetected { level: "high" }` event handled by `learner_reducer` drops
    `math_band` by 1, sets `wrongs_before_teach` to 1, reduces pace by 0.2, and tightens
    `spread_width`.
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
    - robot-buddy-domain/src/learning/learner_profile.rs:311-322
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges:
  - kind: supports
    target: b_05f779b819d4   # frustration-high-on-3-consecutive-wrong
coord: null
---

The reducer matches `FrustrationDetected { level }` and if level == "high" returns a new profile with band-1, wrongs_before_teach:1, pace-0.2, spread_width-0.15 (floored at 0.1). Non-high levels return state unchanged.
