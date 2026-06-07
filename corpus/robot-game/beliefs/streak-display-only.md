---
id: b_7858b5be6a9b
slug: streak-display-only
claim:
  kind: text
  text: >-
    The `streak` field in `LearnerProfile` is display-only — it has zero mechanical effect
    on band promotion or demotion.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2026-03-29T12:00:00Z
  valid_time:
    start: 2026-03-26
    end: 2999-01-01
  source:
    kind: conversation
    session: s3-rustwasm
    turn: 2
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:200-225
    - docs/adr/001-band-blending.md:1-40
  derived_from:
    - b_80d57c6ade26
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.85
  asserted: 0.88
edges:
  - kind: derived_from
    target: b_80d57c6ade26   # band-blending-adr
  - kind: supports
    target: b_ec614ecca420   # promote-needs-4-attempts
coord: null
---

Streak is updated in the reducer (increments on correct, decrements on wrong, resets on promotion/demotion/frustration) but the `should_promote` and `should_demote` functions never read it. ADR-001 explicitly states: 'Streak is display-only.'
