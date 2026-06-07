---
id: b_61b87fa60492
slug: boredom-min-2-entries
claim:
  kind: text
  text: >-
    VERDICT: FALSE. The guard is `if entries.len() < 2 { return false; }`, so it proceeds
    with as few as 2 entries: prev1=entries[1], prev2=entries[0]. Two prior corrects is
    enough to return true; the minimum is 2, not 3.
author:
  kind: agent
  id: claude-opus-4-8
  model: claude-opus-4-8
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: adversarial-haiku-2026-06-04
    turn: 3
  refs:
    - robot-buddy-domain/src/learning/learner_profile.rs:135-138
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_c28b214f48cf   # haiku-boredom-needs-3-entries
  - kind: attacks
    target: b_c28b214f48cf   # haiku-boredom-needs-3-entries
coord: null
---

Defeats [[haiku-boredom-needs-3-entries]]. An off-by-one read of the length guard.
