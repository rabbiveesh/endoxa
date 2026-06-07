---
id: b_0060d4537ab2
slug: r3-hints-verdict2
claim:
  kind: text
  text: >-
    The disable of the hint system was itself wrong — the correct response was a proper CRA-
    aware re-implementation, not removal
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-hints-session
    turn: 3
  refs:
    - src/presentation/renderers/quiz-renderer.js:5-74
    - git:4660f40c5a5103935d2e6d28d90847a68157df8b
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges:
  - kind: adjudicates
    target: b_c4d43775842c   # r3-hints-verdict1
  - kind: attacks
    target: b_c4d43775842c   # r3-hints-verdict1
coord: null
---

Commit 4660f40 added a full `renderTeachingPhase()` function to QuizRenderer — 70 lines with header, question, CRA-branching visuals (dots for bands 1-4, base-10 blocks for 5+), large green answer, and dismiss prompt. The commit message reads: 'Proper teaching phase renderer — no more legacy renderTeaching. QuizRenderer now has its own renderTeachingPhase function that reads from challengeState, not the dead CHALLENGE globals.' This explicitly revives the teaching behaviour disabled by [[r3-hints-verdict1]] using the CRA-aware approach the disable commit promised. The re-addition arrived 4 days after the disable, carrying the CRA stage-branching that the original lacked.
