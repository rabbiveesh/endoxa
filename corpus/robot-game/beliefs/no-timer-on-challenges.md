---
id: b_b261502ec67e
slug: no-timer-on-challenges
claim:
  kind: text
  text: >-
    Children never see a countdown timer on math challenges. Response time is measured
    silently for the adaptive system but never displayed.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-25T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: s2-adaptive
    turn: 1
  refs:
    - CLAUDE.md:27-29
    - robot-buddy-domain/src/learning/rolling_window.rs:7-26
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.92
  asserted: 0.93
edges: []
coord: null
---

CLAUDE.md architecture invariant #4: 'The game must never time-pressure a child. No countdown timers on challenges, ever. We measure response time silently for the adaptive system, but the child never sees a clock.' `response_time_ms` is an optional field in `WindowEntry` and `PuzzleAttempted` event.
