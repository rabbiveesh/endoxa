---
id: b_c4d43775842c
slug: r3-hints-verdict1
claim:
  kind: text
  text: >-
    Dot-counting hints are broken and must be disabled — they do not scale past band 3
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time: null
  source:
    kind: conversation
    session: r3-hints-session
    turn: 2
  refs:
    - dialogue.js:756-770
    - git:abb4888bbd863842357ea47e2d9674c2412641f3
  derived_from: []
confidence:
  directness: stated
  observation_count: 1
  source_weight: 0.7
  asserted: 0.85
edges:
  - kind: adjudicates
    target: b_de8308a33d25   # r3-hints-original
  - kind: attacks
    target: b_de8308a33d25   # r3-hints-original
coord: null
---

Commit abb4888 replaced the `showTeaching = true` branch with a comment: 'Hints/teaching disabled — current dot system doesn't scale past band 3. Will reintroduce with proper CRA-aware representations (tens bars, number lines, base-10 blocks) that actually help at higher bands.' The Sparky 'Let's figure it out together!' speakLine was deleted. The system now just marks the challenge answered-wrong and moves on. This verdict directly attacks [[r3-hints-original]].
