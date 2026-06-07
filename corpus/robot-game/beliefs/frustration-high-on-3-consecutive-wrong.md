---
id: b_05f779b819d4
slug: frustration-high-on-3-consecutive-wrong
claim:
  kind: text
  text: >-
    Three or more consecutive wrong answers triggers `FrustrationLevel::High` with a
    `drop_band` recommendation, regardless of overall accuracy.
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
    - robot-buddy-domain/src/learning/frustration_detector.rs:22-28
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.88
  asserted: 0.9
edges: []
coord: null
---

`detect_frustration` checks `window.consecutive_wrong() >= 3` first, before any accuracy check. This is intentional priority: consecutive failures signal acute distress distinct from poor rolling accuracy.
