---
id: b_332f20dfa30a
slug: kenken-confidence-resets-on-promotion
claim:
  kind: text
  text: >-
    When a child is promoted to a larger KenKen grid size, `logic_confidence` resets to 0.5
    to keep difficulty fair at the new level.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-04-26T12:00:00Z
  valid_time:
    start: 2026-04-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s4-macroquad
    turn: 3
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:358-363
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.82
  asserted: 0.84
edges: []
coord: null
---

In `learner_reducer` for `KenKenAttempted`, after level promotion: `conf = 0.5; // reset on promotion to keep difficulty fair`. This mirrors the band-blending philosophy: promotion happens after demonstrated mastery, then the slate is partially wiped.
