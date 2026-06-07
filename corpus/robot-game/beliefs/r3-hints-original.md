---
id: b_de8308a33d25
slug: r3-hints-original
claim:
  kind: text
  text: >-
    Dot-counting visuals on 2-wrong trigger are sufficient to teach addition/subtraction at
    any band
author:
  kind: human
  id: maintainer
provenance:
  txn_time: 2026-06-04T12:00:00Z
  valid_time:
    start: 2026-03-24
    end: 2026-03-25
  source:
    kind: conversation
    session: r3-hints-session
    turn: 1
  refs:
    - dialogue.js:33-34
    - dialogue.js:912
    - git:7ae7e742ec7c23f61fd6539d77d5d06d574314f2
  derived_from: []
confidence:
  directness: inferred
  observation_count: 1
  source_weight: 0.65
  asserted: null
edges: []
coord: null
---

The initial commit (7ae7e74) introduced a `showTeaching` flag: after 2 wrong attempts the game displayed a `renderTeaching()` overlay using dot visuals and base-10 blocks, with a 'Let's figure it out together!' Sparky line. The state field `teachingData: { a, b, op, answer }` was populated by every math challenge. This assumed a single dot-based visual register would work across all difficulty bands.
