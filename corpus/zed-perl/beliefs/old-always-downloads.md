---
id: b_8416f5315053
slug: old-always-downloads
claim:
  kind: text
  text: >-
    The initial GitHub-release implementation always downloaded the binary regardless of
    whether the user had perlnavigator installed locally.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2024-09-08T12:00:00Z
  valid_time:
    start: 2024-09-08
    end: 2024-09-19
  source:
    kind: conversation
    session: seed-explore-02
    turn: 5
  refs:
    - git:feat: switch to github release until the next npm version is out
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.75
  asserted: 0.8
edges: []
coord: null
---

The 88471b7 commit introduced GitHub release downloads but did not check PATH first. The check was added later in 011deca.
