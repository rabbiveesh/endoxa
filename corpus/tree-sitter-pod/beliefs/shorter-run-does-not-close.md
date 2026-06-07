---
id: b_7b3df189af4e
slug: shorter-run-does-not-close
claim:
  kind: text
  text: >-
    A run of '>' characters shorter than the opening count does NOT close the interior
    sequence; only an exact count (or more) closes it.
author:
  kind: agent
  id: claude-sonnet-4-6
  model: claude-sonnet-4-6
provenance:
  txn_time: 2023-03-03T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: session-chevrons-2023-03
    turn: 2
  refs:
    - src/scanner.c:270-280
    - test/corpus/interior-sequences:63-72
    - git:Don't end an interior sequence on a shorter run of '>' chars
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.9
  asserted: 0.9
edges: []
coord: null
---

The original code (before fix fff3b6a) would pop the chevron stack as soon as it consumed one fewer '>' and the loop exited, regardless of whether count had hit zero. Fix fff3b6a added 'if(!count)' guard. Test case 'Does not consume shorter sequence of >' in test/corpus/interior-sequences confirms.
